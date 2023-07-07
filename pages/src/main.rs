use crate::calc::calc;
use crate::compression::Compression;
use crate::event::event_target;
use crate::mode::Mode;
use row::Row;
use schema::Results;
use std::str::FromStr;
use stylist::css;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

mod calc;
mod compression;
mod event;
mod mode;
mod row;

// TODO latest json.
const RAW: &str = include_str!("../../benchmark_results/2023-7-7_2-13-59.json");

#[derive(PartialEq)]
struct Var<T> {
    value: T,
    on_change: Callback<T>,
}

impl<T: Clone + 'static> Var<T> {
    fn new(state: UseStateHandle<T>) -> Self {
        Self::with_map(state, |next, _| next)
    }

    /// Maps (next, current) -> next.
    fn with_map(state: UseStateHandle<T>, map: fn(T, &T) -> T) -> Self {
        Var {
            value: (*state).clone(),
            on_change: {
                let state = state.clone();
                Callback::from(move |next| {
                    let next = map(next, &*state);
                    state.set(next)
                })
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
    let results: Results = serde_json::from_str(RAW).unwrap();
    let datasets: Vec<_> = results.datasets.keys().collect();

    let bandwidth_state = Var::with_map(use_state(|| "1".to_owned()), step_zero_down);
    let bandwidth = f64::from_str(&bandwidth_state.value).unwrap_or_default();

    let cpus_state = Var::with_map(use_state(|| "0.01".to_owned()), step_zero_down);
    let cpus = f64::from_str(&cpus_state.value).unwrap_or_default();

    let dataset_state = Var::new(use_state(|| {
        let default_dataset = "mk48";
        if results.datasets.contains_key(default_dataset) {
            default_dataset.to_owned()
        } else {
            (*datasets.iter().next().unwrap()).clone()
        }
    }));
    let dataset = dataset_state.value;
    let (message_name, messages_per_benchmark) = match dataset.as_str() {
        "log" => ("logs", 10_000),
        "mesh" => ("meshes", 1),
        "minecraft_savedata" => ("saves", 500),
        "mk48" => ("updates", 1_000),
        _ => ("messages", 1),
    };

    let mode_state = Var::new(use_state(|| Mode::Serialize));
    let mode = mode_state.value;

    let compression_set_state = Var::new(use_state(|| Compression::iter().collect()));
    let compression_set = &compression_set_state.value;

    let rows: Result<Vec<Row>, _> = results
        .datasets
        .get(&dataset)
        .unwrap()
        .crates
        .iter()
        .map(TryFrom::try_from)
        .collect();
    let rows = match rows {
        Ok(rows) => rows,
        Err(err) => return html! { { format!("error: {err}") } },
    };

    let bandwidth_bytes = (bandwidth * 1_000_000_000_000.0) as u64;

    let rows = calc(
        rows,
        messages_per_benchmark,
        bandwidth_bytes,
        cpus as f32,
        compression_set,
        mode,
    );

    let on_bandwidth = on_event!(bandwidth_state, InputEvent, HtmlInputElement);
    let on_cpus = on_event!(cpus_state, InputEvent, HtmlInputElement);
    let on_dataset = on_event!(dataset_state, Event, HtmlSelectElement);
    let on_mode = on_event!(mode_state, Event, HtmlSelectElement);

    let selection_style = css!(
        r#"
        margin: auto;
        margin-bottom: 0.7rem;
        input[type=number], select {
            width: 6rem;
        }
        th, td {
            padding-left: 0.2rem;
            padding-right: 0.2rem;
        }
        "#
    );

    let inner_selection_style = css!(
        r#"
        margin-left: 0.7rem;
        margin-right: 0.7rem;
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

    let bandwidth_step = float_step(bandwidth);
    let cpus_step = float_step(cpus);
    let mode_description = mode.description();

    html! {
        <div>
            <div style="text-align:center;">
                <h2 style="margin: 0.5rem; margin-top: 0.75rem;">{ "Rust Serialization Benchmark" }</h2>
                <p style="white-space: initial; width: 80%; margin: auto; margin-bottom: 1rem; color: #a9a9a9; font-style: italic;">
                    { format!("Given monthly bandwidth and CPU allocation, how many {message_name} can be {mode_description} per second?") }
                </p>
            </div>
            <table class={selection_style}><tr>
                <td><table class={inner_selection_style.clone()}>
                    <tr title="Bandwidth allocated in terabytes per month">
                        <td><label for="bandwidth"> { "Bandwidth " } </label></td>
                        <td><input name="bandwidth" type="number" min="0" step={bandwidth_step} oninput={on_bandwidth} value={bandwidth_state.value}/></td>
                        <td><label for="bandwidth"> { " TB/Mo" } </label></td>
                    </tr>
                    <tr title="Fractional CPU cores allocated for serialization and compression">
                        <td><label for="cpus"> { "CPU " } </label></td>
                        <td><input name="cpus" type="number" min="0" step={cpus_step} oninput={on_cpus} value={cpus_state.value}/></td>
                        <td><label for="bandwidth"> { " cores" } </label></td>
                    </tr>
                    <tr title="See rust_serialization_benchmark">
                        <td><label for="dataset"> { "Dataset " } </label></td>
                        <td>
                            <select name="dataset" onchange={on_dataset}>
                                {
                                    datasets.iter().map(|&d| html! {
                                        <option value={d.clone()} selected={d == &dataset}> {d} </option>
                                    }).collect::<Html>()
                                }
                            </select>
                        </td>
                        <td/>
                    </tr>
                    <tr title="Measure serialization, deserialization or both">
                        <td><label for="mode"> { "Mode " } </label></td>
                        <td>
                            <select name="mode" onchange={on_mode}>
                                {
                                    Mode::iter().map(|m| html! {
                                        <option value={m.to_string()} selected={m == mode}> {m.to_string()} </option>
                                    }).collect::<Html>()
                                }
                            </select>
                        </td>
                        <td/>
                    </tr>
                </table></td>
                <td><table class={inner_selection_style}>
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
                </table></td>
            </tr></table>
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
                                <td><a href={format!("https://crates.io/crates/{}/{}", row.crate_, results.meta.crate_versions.get(&row.crate_).unwrap())} target="_blank">{ &row.crate_ }</a></td>
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

fn float_ilog2(float: f64) -> i32 {
    if float <= 0.0 {
        -2
    } else {
        // Work around some rounding errors.
        let offset = 64;
        ((float * 1.001).log10().floor() + 0.25 + offset as f64) as u32 as i32 - offset
    }
}

fn float_step(float: f64) -> String {
    10f64.powi(float_ilog2(float)).to_string()
}

// Overrides step down 0.1 - 0.1 => 0.09.
#[allow(clippy::ptr_arg)]
fn step_zero_down(next: String, previous: &String) -> String {
    if let Ok(float) = f64::from_str(&next) {
        if float == 0.0 {
            if let Ok(previous) = f64::from_str(previous) {
                if previous >= 1e-10 {
                    // Max 11 decimals and trim trailing zeros to work around rounding errors.
                    let s = format!("{:.11}", (10f64.powi(float_ilog2(previous) - 1) * 9.0));
                    return s.trim_end_matches('0').trim_end_matches('.').to_owned();
                }
            }
        }
    }
    next
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
