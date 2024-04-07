use bilrost::buf::ReverseBuffer;
use bilrost::bytes::BufMut;
use bilrost::Message;
use criterion::{black_box, Criterion};
use std::borrow::Borrow;

pub trait ToBilrost: Sized {
    type Message: Message + Into<Self>;
    type Serializable<'a>: Borrow<Self::Message>
    where
        Self: 'a;

    fn to_bilrost(&self) -> Self::Serializable<'_>;

    fn is_already_bilrost() -> bool {
        false // provided: false
    }
}

impl<T> ToBilrost for T
where
    T: Clone + Message,
{
    type Message = Self;
    type Serializable<'a> = &'a Self
    where
        Self: 'a;

    fn to_bilrost(&self) -> &Self {
        self
    }

    fn is_already_bilrost() -> bool {
        true // true for this covering impl only
    }
}

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: ToBilrost + PartialEq,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/bilrost", name));

    let mut serialize_buffer = Vec::with_capacity(BUFFER_LEN);
    let mut prepend_buffer = ReverseBuffer::with_capacity(BUFFER_LEN);

    if !T::is_already_bilrost() {
        group.bench_function("serialize (populate + encode)", |b| {
            b.iter(|| {
                black_box(&mut serialize_buffer).clear();
                data.to_bilrost()
                    .borrow()
                    .encode(&mut serialize_buffer)
                    .unwrap();
                black_box(());
            })
        });
        group.bench_function("serialize (populate + prepend)", |b| {
            b.iter(|| {
                black_box(&mut prepend_buffer).clear();
                data.to_bilrost().borrow().prepend(&mut prepend_buffer);
                black_box(());
            })
        });
    }

    let message = data.to_bilrost();
    group.bench_function("serialize (only encode)", |b| {
        b.iter(|| {
            black_box(&mut serialize_buffer).clear();
            message.borrow().encode(&mut serialize_buffer).unwrap();
            black_box(());
        })
    });

    let message = data.to_bilrost();
    group.bench_function("serialize (only prepend)", |b| {
        b.iter(|| {
            black_box(&mut prepend_buffer).clear();
            message.borrow().prepend(&mut prepend_buffer);
            black_box(());
        })
    });

    let mut deserialize_buffer = Vec::new();
    message.borrow().encode(&mut deserialize_buffer).unwrap();
    let mut prepended_data = Vec::new();
    prepended_data.put(message.borrow().encode_fast());
    // Because there are no unordered collections in the benchmarked types, we can assert that the
    // prepended encoding path emits precisely the same bytes as the forward-encoded one.
    assert_eq!(prepended_data, deserialize_buffer);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(<T::Message>::decode(black_box(&deserialize_buffer).as_slice()).unwrap());
        })
    });

    crate::bench_size(name, "bilrost", deserialize_buffer.as_slice());

    assert!(<T::Message>::decode(&*deserialize_buffer).unwrap().into() == *data);

    group.finish();
}
