use criterion::{black_box, Criterion};
use omnibuf::{DecodeBorrowed, Message};

pub trait Serialize: Sized {
    type Message: Default + Into<Self> + Message;

    fn serialize_omnibuf(&self) -> Self::Message;
}

pub trait SerializeBorrowed: Serialize {
    type Borrowed<'a>: DecodeBorrowed<'a> + Into<Self>;
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/omnibuf", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            data.serialize_omnibuf()
                .encode_omnibuf_append(&mut serialize_buffer, false)
                .unwrap();
            black_box(());
        })
    });

    let message = data.serialize_omnibuf();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            message
                .encode_omnibuf_append(&mut serialize_buffer, false)
                .unwrap();
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    data.serialize_omnibuf()
        .encode_omnibuf_append(&mut deserialize_buffer, false)
        .unwrap();

    group.bench_function("deserialize (decode)", |b| {
        b.iter(|| {
            black_box(<T::Message>::from_omnibuf(black_box(&deserialize_buffer)).unwrap());
        })
    });

    group.bench_function("deserialize (decode + convert)", |b| {
        b.iter(|| {
            let decoded = <T::Message>::from_omnibuf(black_box(&deserialize_buffer)).unwrap();
            black_box(decoded.into());
        })
    });

    crate::bench_size(name, "omnibuf", deserialize_buffer.as_slice());

    assert!(
        <T::Message>::from_omnibuf(&deserialize_buffer)
            .unwrap()
            .into()
            == *data
    );

    group.finish();
}

pub fn bench_borrowable<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: SerializeBorrowed + PartialEq,
{
    bench(name, c, data);

    let mut group = c.benchmark_group(format!("{}/omnibuf", name));

    let mut deserialize_buffer = Vec::new();
    data.serialize_omnibuf()
        .encode_omnibuf_append(&mut deserialize_buffer, false)
        .unwrap();

    assert!(
        <T::Borrowed<'_>>::from_omnibuf_borrowed(&deserialize_buffer)
            .unwrap()
            .into()
            == *data
    );

    group.bench_function("borrow", |b| {
        b.iter(|| {
            black_box(
                <T::Borrowed<'_>>::from_omnibuf_borrowed(black_box(&deserialize_buffer)).unwrap(),
            );
        })
    });

    group.finish();
}
