#[cfg(feature = "abomonation")]
pub mod bench_abomonation;
#[cfg(feature = "bincode")]
pub mod bench_bincode;
#[cfg(feature = "borsh")]
pub mod bench_borsh;
#[cfg(feature = "capnp")]
pub mod bench_capnp;
#[cfg(feature = "serde_cbor")]
pub mod bench_cbor;
#[cfg(feature = "flatbuffers")]
pub mod bench_flatbuffers;
#[cfg(feature = "nachricht-serde")]
pub mod bench_nachricht;
#[cfg(feature = "postcard")]
pub mod bench_postcard;
#[cfg(feature = "prost")]
pub mod bench_prost;
#[cfg(feature = "rkyv")]
pub mod bench_rkyv;
#[cfg(feature = "rmp-serde")]
pub mod bench_rmp;
#[cfg(feature = "ron")]
pub mod bench_ron;
#[cfg(feature = "serde_json")]
pub mod bench_serde_json;
#[cfg(feature = "simd-json")]
pub mod bench_simd_json;
#[cfg(feature = "speedy")]
pub mod bench_speedy;
#[cfg(feature = "alkahest")]
pub mod bench_alkahest;

pub mod datasets;

use core::{mem, ops};
use rand::Rng;

pub trait Generate {
    fn generate<R: Rng>(rng: &mut R) -> Self;
}

impl Generate for () {
    fn generate<R: Rng>(_: &mut R) -> Self {
        ()
    }
}

impl Generate for bool {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        rng.gen_bool(0.5)
    }
}

macro_rules! impl_generate {
    ($ty:ty) => {
        impl Generate for $ty {
            fn generate<R: Rng>(rng: &mut R) -> Self {
                rng.gen()
            }
        }
    }
}

impl_generate!(u8);
impl_generate!(u16);
impl_generate!(u32);
impl_generate!(u64);
impl_generate!(u128);
impl_generate!(usize);
impl_generate!(i8);
impl_generate!(i16);
impl_generate!(i32);
impl_generate!(i64);
impl_generate!(i128);
impl_generate!(isize);
impl_generate!(f32);
impl_generate!(f64);

macro_rules! impl_tuple {
    () => {};
    ($first:ident, $($rest:ident,)*) => {
        impl<$first: Generate, $($rest: Generate,)*> Generate for ($first, $($rest,)*) {
            fn generate<R: Rng>(rng: &mut R) -> Self {
                ($first::generate(rng), $($rest::generate(rng),)*)
            }
        }

        impl_tuple!($($rest,)*);
    };
}

impl_tuple!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,);

macro_rules! impl_array {
    () => {};
    ($len:literal, $($rest:literal,)*) => {
        impl<T: Generate> Generate for [T; $len] {
            fn generate<R: Rng>(rng: &mut R) -> Self {
                let mut result = mem::MaybeUninit::<Self>::uninit();
                let result_ptr = result.as_mut_ptr().cast::<T>();
                for i in 0..$len {
                    unsafe {
                        result_ptr.add(i).write(T::generate(rng));
                    }
                }
                unsafe {
                    result.assume_init()
                }
            }
        }

        impl_array!($($rest,)*);
    }
}

impl_array!(31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, );

impl<T: Generate> Generate for Option<T> {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        if rng.gen_bool(0.5) {
            Some(T::generate(rng))
        } else {
            None
        }
    }
}

pub fn generate_vec<R: Rng, T: Generate>(rng: &mut R, range: ops::Range<usize>) -> Vec<T> {
    let len = rng.gen_range(range);
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::generate(rng));
    }
    result
}

pub fn bench_size(name: &str, lib: &str, bytes: &[u8]) {
    println!("{}/{}/size {}", name, lib, bytes.len());
    println!("{}/{}/zlib {}", name, lib, zlib_size(bytes));
    println!("{}/{}/zstd {}", name, lib, zstd_size(bytes));
}

fn zlib_size(mut bytes: &[u8]) -> usize {
    let mut encoder = libflate::zlib::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut bytes, &mut encoder).unwrap();
    encoder.finish().into_result().unwrap().len()
}

fn zstd_size(bytes: &[u8]) -> usize {
    zstd::stream::encode_all(bytes, 0).unwrap().len()
}
