use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = "../benchmark_results";
    println!("cargo:rerun-if-changed={path}");

    let (_, from) = fs::read_dir(path)
        .unwrap()
        .filter_map(|path| {
            let path = path.unwrap().path();
            let time = parse_path(&path)?;
            Some((time, path))
        })
        .max_by_key(|(time, _)| *time)
        .expect("no benchmark results found");

    let to = Path::new("src/latest.json");
    fs::copy(&from, to).unwrap();
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

fn parse_path(path: &Path) -> Option<DateTime> {
    let s = path.file_name()?.to_str()?;

    // {year}-{month}-{day}_{hour}-{minute}-{second}.json
    let (year, s) = s.split_once('-')?;
    let year = u16::from_str(year).ok()?;
    let (month, s) = s.split_once('-')?;
    let month = u8::from_str(month).ok()?;
    let (day, s) = s.split_once('_')?;
    let day = u8::from_str(day).ok()?;
    let (hour, s) = s.split_once('-')?;
    let hour = u8::from_str(hour).ok()?;
    let (minute, s) = s.split_once('-')?;
    let minute = u8::from_str(minute).ok()?;
    let (second, ext) = s.split_once('.')?;
    let second = u8::from_str(second).ok()?;
    if ext != "json" {
        return None;
    }

    Some(DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}
