use criterion::{black_box, Criterion};
use wiring::prelude::{BufUnWire, BufWire, Unwiring, Wiring};

pub fn bench<T: Wiring + Unwiring + PartialEq>(name: &'static str, c: &mut Criterion, data: &T) {
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/wiring", name));

    let mut wire: Vec<u8> = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize", |b| {
        b.iter(|| BufWire::new(&mut wire).wire(black_box(data)).unwrap())
    });

    BufWire::new(&mut wire).wire(data).unwrap();

    let buffer = wire.as_slice();

    group.bench_function("deserialize", |b| {
        b.iter(|| BufUnWire::new(black_box(buffer)).unwire::<T>().unwrap())
    });

    crate::bench_size(name, "wiring", wire.as_slice());

    let unwired: T = BufUnWire::new(buffer).unwire().unwrap();
    assert!(&unwired == data);

    group.finish();
}
