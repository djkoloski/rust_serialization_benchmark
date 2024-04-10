use criterion::{black_box, Criterion};
use tokio::runtime::Runtime;
use wiring::prelude::{Unwire, Unwiring, Wire, Wiring};
use zstd::zstd_safe::WriteBuf;

pub fn bench<'a, T>(name: &'static str, c: &mut Criterion, data: &'a T)
where
    &'a T: Wiring,
    T: Unwiring + PartialEq,
{
    async fn wire_data<W: Wire, T: Wiring>(mut wire: W, data: T) {
        (&mut wire).wire(data).await.unwrap();
    }

    let rt = Runtime::new().unwrap();
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/wiring", name));

    let mut wire: Vec<u8> = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize", |b| {
        b.to_async(Runtime::new().unwrap()).iter_batched(
            || wire.clone(),
            |w| wire_data(black_box(w), black_box(data)),
            criterion::BatchSize::SmallInput,
        );
    });

    rt.block_on(async {
        (&mut wire).wire(data).await.unwrap();
    });

    let mut unwire = std::io::Cursor::new(wire);

    group.bench_function("deserialize", |b| {
        b.to_async(Runtime::new().unwrap()).iter_batched(
            || unwire.clone(),
            |w| async move {
                black_box(w).unwire::<T>().await.unwrap();
            },
            criterion::BatchSize::SmallInput,
        );
    });

    crate::bench_size(name, "wiring", unwire.as_slice());

    rt.block_on(async {
        let unwired = unwire.unwire::<T>().await.unwrap();
        assert!(&unwired == data);
    });

    group.finish();
}
