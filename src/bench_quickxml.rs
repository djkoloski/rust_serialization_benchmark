use serde::{Serialize, Deserialize};
use criterion::{black_box, Criterion};
use quick_xml;

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/serde_quickxml", name));

    let mut serialize_buffer = String::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                quick_xml::se::to_writer(
                    black_box(&mut serialize_buffer),
                    black_box(&data),
                )
                .unwrap()
            );
        })
    });

    let mut deserialize_buffer = String::new();
    quick_xml::se::to_writer(&mut deserialize_buffer, &data).unwrap();
    println!("{}", &deserialize_buffer);


    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(
                quick_xml::de::from_reader::<&[u8], T>(black_box(&deserialize_buffer.as_bytes()))
                .unwrap()
            );
        })
    });

    println!("{}/serde_quickxml/size {}", name, deserialize_buffer.len());
    println!("{}/serde_quickxml/zlib {}", name, crate::zlib_size(deserialize_buffer.as_bytes()));

    group.finish();
}
