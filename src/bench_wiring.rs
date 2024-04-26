use criterion::{black_box, Criterion};
use wiring::prelude::{BufUnWire, BufWire, Unwiring, Wiring};

pub fn bench<T: Wiring + Unwiring + PartialEq>(name: &'static str, c: &mut Criterion, data: &T) {
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/wiring", name));

    let mut wire: Vec<u8> = Vec::new();
    wire.reserve(BUFFER_LEN); // Optional

    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(BufWire::new(&mut wire).wire(black_box(data)).unwrap());
        })
    });

    let buffer = wire.as_slice();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(BufUnWire::new(black_box(buffer)).unwire::<T>().unwrap());
        })
    });

    crate::bench_size(name, "wiring", wire.as_slice());

    let unwired: T = BufUnWire::new(buffer).unwire().unwrap();
    assert!(&unwired == data);

    group.finish();
}
