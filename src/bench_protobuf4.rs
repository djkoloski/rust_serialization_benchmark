use criterion::{black_box, Criterion};
use protobuf4::{AsView, Parse, Serialize as _};

pub trait Serialize: Sized {
    type Message: Default + protobuf4::Message;

    fn serialize_pb(&self) -> Self::Message;

    fn from_view(view: <Self::Message as protobuf4::Message>::MessageView<'_>) -> Self;

    fn from_owned(owned: Self::Message) -> Self {
        Self::from_view(owned.as_view())
    }
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + PartialEq,
{
    let mut group = c.benchmark_group(format!("{}/protobuf4", name));

    group.bench_function("serialize (populate + encode)", |b| {
        b.iter(|| {
            black_box(data.serialize_pb().serialize().unwrap());
            black_box(());
        })
    });

    let message = data.serialize_pb();
    group.bench_function("serialize (encode)", |b| {
        b.iter(|| {
            black_box(message.serialize().unwrap());
            black_box(());
        })
    });

    let deserialize_buffer = data.serialize_pb().serialize().unwrap();

    // protobuf 4 doesn't, for example, validate string values as valid utf8 at all until you ask
    // for them
    group.bench_function("deserialize (decode, unvalidated)", |b| {
        b.iter(|| {
            black_box(T::Message::parse(black_box(&deserialize_buffer)).unwrap());
        })
    });

    // protobuf 4 has extra-inaccessible ownership of its deserialized data, so we should undergo
    // conversion to our original owned type here and measure that as well
    group.bench_function("deserialize (decode + convert)", |b| {
        b.iter(|| {
            let val = T::Message::parse(black_box(&deserialize_buffer)).unwrap();
            black_box(T::from_owned(val));
        })
    });

    crate::bench_size(name, "protobuf4", deserialize_buffer.as_slice());

    assert!(T::from_owned(T::Message::parse(&deserialize_buffer).unwrap()) == *data);

    group.finish();
}

// protobuf 4 has "view" types for every message type, but they cannot be directly decoded into.
