use crate::calc::calc;
use crate::compression::Compression;
use crate::event::event_target;
use row::Row;
use std::str::FromStr;
use stylist::css;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

mod calc;
mod compression;
mod event;
mod row;

const RAW: &str = include_str!("../../README.md");
const DATASETS: [(&str, &str, u32); 4] = [
    ("log", "logs", 10_000),
    ("mesh", "meshes", 1),
    ("minecraft_savedata", "saves", 500),
    ("mk48", "updates", 1_000),
];

#[derive(PartialEq)]
struct Var<T> {
    value: T,
    on_change: Callback<T>,
}

impl<T: Clone + 'static> Var<T> {
    fn new(state: UseStateHandle<T>) -> Self {
        Var {
            value: (*state).clone(),
            on_change: {
                let state = state.clone();
                Callback::from(move |value| state.set(value))
            },
        }
    }
}

macro_rules! on_event {
    ($var:ident, $event:ty, $element:ty) => {{
        let on_change = $var.on_change.clone();
        Callback::from(move |event: $event| {
            let element: $element = event_target(&event);
            on_change.emit(FromStr::from_str(&element.value()).unwrap());
        })
    }};
}

#[function_component(Benchmark)]
fn benchmark() -> Html {
    let bandwidth_state = Var::new(use_state(|| "1".to_owned()));
    let bandwidth = f32::from_str(&bandwidth_state.value).unwrap_or_default();

    let cpus_state = Var::new(use_state(|| "0.01".to_owned()));
    let cpus = f32::from_str(&cpus_state.value).unwrap_or_default();

    let dataset_state = Var::new(use_state(|| 3usize));
    let (name, message_name, messages_per_benchmark) = DATASETS[dataset_state.value];

    let compression_set_state = Var::new(use_state(|| Compression::iter().collect()));
    let compression_set = &compression_set_state.value;

    let s = RAW;
    let (_, s) = s.split_once(&format!("## `{}`", name)).unwrap();
    let (s, _) = s
        .split_once("#### Zero-copy deserialization speed")
        .unwrap();

    let lines = s.lines().filter(|l| l.starts_with("| ["));
    let rows: Result<Vec<Row>, _> = lines.map(FromStr::from_str).collect();
    let rows = rows.unwrap();

    let bandwidth_bytes = (bandwidth * 1_000_000_000_000.0) as u64;

    let rows = calc(
        rows,
        messages_per_benchmark,
        bandwidth_bytes,
        cpus,
        compression_set,
    );

    let on_bandwidth = on_event!(bandwidth_state, InputEvent, HtmlInputElement);
    let on_cpus = on_event!(cpus_state, InputEvent, HtmlInputElement);
    let on_dataset = on_event!(dataset_state, Event, HtmlSelectElement);

    let selection_style = css!(
        r#"
        margin: 0.5rem auto;
        input[type=number], select {
            width: 8rem;
        }
        th, td {
            padding-left: 0.3rem;
            padding-right: 0.3rem;
        }
        "#
    );

    let table_style = css!(
        r#"
        border-collapse: collapse;
        tr {
            background-color: #3498db;
        }
        tr:nth-child(even) {
            background-color: #2980b9;
        }
        th, td {
            text-align: left;
            padding: 0.3rem;
            padding-left: 0.6rem;
            padding-right: 0.6rem;
            min-width: 100px;
        }
        a {
            color: white;
            text-decoration: none;
        }
        a:hover {
            filter: brightness(75%);
        }
        "#
    );

    html! {
        <div>
            <div style="text-align:center;">
                <h2 style="margin: 0.5rem; margin-top: 0.75rem;">{ "Rust Serialization Benchmark" }</h2>
                <p style="white-space: initial; width: 65%; margin: auto; margin-bottom: 1rem; color: #a9a9a9; font-style: italic;">
                    { format!("Given monthly bandwidth and CPU allocation, how many {} can be serialized and sent per second?", message_name) }
                </p>
            </div>
            <table class={selection_style}>
                <tr title="Bandwidth allocated in terabytes per month">
                    <td><label for="bandwidth"> { "Bandwidth " } </label></td>
                    <td><input name="bandwidth" type="number" min="0" step="0.1" oninput={on_bandwidth} value={bandwidth_state.value}/></td>
                    <td><label for="bandwidth"> { " TB/Mo" } </label></td>
                </tr>
                <tr title="Fractional CPU cores allocated for serialization and compression">
                    <td><label for="cpus"> { "CPU " } </label></td>
                    <td><input name="cpus" type="number" min="0" step="0.01" oninput={on_cpus} value={cpus_state.value}/></td>
                    <td><label for="bandwidth"> { " cores" } </label></td>
                </tr>
                <tr title="See rust_serialization_benchmark">
                    <td><label for="dataset"> { "Dataset " } </label></td>
                    <td>
                        <select name="dataset" onchange={on_dataset}>
                            {
                                DATASETS.iter().enumerate().map(|(i, (b, ..))| html! {
                                    <option value={i.to_string()}> {b} </option>
                                }).collect::<Html>()
                            }
                        </select>
                    </td>
                    <td/>
                </tr>
                {
                    Compression::iter().filter(|c| c.is_some()).map(|compression| {
                        let checked = compression_set.contains(compression);

                        let on_change = compression_set_state.on_change.clone();
                        let compression_set = *compression_set;
                        let oninput = Callback::from(move |event: InputEvent| {
                            let element: HtmlInputElement = event_target(&event);
                            let mut s = compression_set;
                            if element.checked() {
                                s.insert(compression);
                            } else {
                                s.remove(compression);
                            }
                            on_change.emit(s);
                        });

                        let name = format!("{compression}");
                        let title = format!("Allow {compression} compression");

                        html! {
                            <tr {title}>
                                <td><label for={name.clone()}> { format!("{compression} ") } </label></td>
                                <td><input {name} {oninput} {checked} type="checkbox"/></td>
                                <td/>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
            <table class = {table_style.clone()}>
                <tr>
                    <th> { "Crate" } </th>
                    <th> { "Compression" } </th>
                    <th> { format!("{}/s", message_name) } </th>
                    <th> { "Relative" } </th>
                    <th> { "Bottleneck" } </th>
                </tr>
                {
                    rows.iter().map(|row| {
                        html! {
                            <tr>
                                <td><a href={format!("https://crates.io/crates/{}/{}", row.crate_, row.version)} target="_blank">{ &row.crate_ }</a></td>
                                <td> { row.compression.to_string() } </td>
                                <td> { format_float(row.messages_per_second, 3) } </td>
                                <td> { format!("{}%", (row.relative * 100.0) as u32) } </td>
                                <td> { row.limit.to_string() } </td>
                            </tr>
                        }
                    }).collect::<Html>()
                }
            </table>
            <div class={table_style} style="text-align: center; margin: 0.6rem;">
                { "See " } <a href="https://github.com/djkoloski/rust_serialization_benchmark" target="_blank"> { "rust_serialization_benchmark" } </a>
            </div>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let style = css!(
        r#"
        color: white;
        user-select: none;
        font-family: sans-serif;
        width: min-content;
        margin: auto;
        white-space: nowrap;
        "#
    );

    html! {
        <div class={style}>
            <Benchmark/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr
fn format_float(float: f32, precision: usize) -> String {
    let a = float.abs();
    let precision = if a >= 1. {
        let n = (1. + a.log10().floor()) as usize;
        if n <= precision {
            precision - n
        } else {
            0
        }
    } else if a > 0. {
        let n = -(1. + a.log10().floor()) as usize;
        precision + n
    } else {
        0
    };
    format!("{0:.1$}", float, precision)
}
