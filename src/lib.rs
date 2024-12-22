#![cfg_attr(feature = "serialization", feature(const_trait_impl))]
#![cfg_attr(feature = "serialization", feature(generic_const_exprs))]
#![cfg_attr(feature = "serialization", feature(specialization))]
#![cfg_attr(feature = "serialization", feature(new_zeroed_alloc))]
// wiring causes this clippy lint everywhere
#![cfg_attr(feature = "wiring", allow(clippy::manual_async_fn))]

#[cfg(feature = "bilrost")]
pub mod bench_bilrost;
#[cfg(feature = "bincode")]
pub mod bench_bincode;
#[cfg(feature = "bincode1")]
pub mod bench_bincode1;
#[cfg(feature = "bitcode")]
pub mod bench_bitcode;
#[cfg(feature = "borsh")]
pub mod bench_borsh;
#[cfg(feature = "capnp")]
pub mod bench_capnp;
#[cfg(feature = "cbor4ii")]
pub mod bench_cbor4ii;
#[cfg(feature = "ciborium")]
pub mod bench_ciborium;
#[cfg(feature = "databuf")]
pub mod bench_databuf;
#[cfg(feature = "dlhn")]
pub mod bench_dlhn;
#[cfg(feature = "flatbuffers")]
pub mod bench_flatbuffers;
#[cfg(feature = "msgpacker")]
pub mod bench_msgpacker;
#[cfg(feature = "nachricht-serde")]
pub mod bench_nachricht_serde;
#[cfg(feature = "nanoserde")]
pub mod bench_nanoserde;
#[cfg(feature = "scale")]
pub mod bench_parity_scale_codec;
#[cfg(feature = "postcard")]
pub mod bench_postcard;
#[cfg(feature = "pot")]
pub mod bench_pot;
#[cfg(feature = "prost")]
pub mod bench_prost;
#[cfg(feature = "rkyv")]
pub mod bench_rkyv;
#[cfg(feature = "rmp-serde")]
pub mod bench_rmp_serde;
#[cfg(feature = "ron")]
pub mod bench_ron;
#[cfg(feature = "savefile")]
pub mod bench_savefile;
#[cfg(feature = "serde_bare")]
pub mod bench_serde_bare;
#[cfg(feature = "serde-brief")]
pub mod bench_serde_brief;
#[cfg(feature = "serde_cbor")]
pub mod bench_serde_cbor;
#[cfg(feature = "serde_json")]
pub mod bench_serde_json;
#[cfg(feature = "serialization")]
pub mod bench_serialization;
#[cfg(feature = "simd-json")]
pub mod bench_simd_json;
#[cfg(feature = "speedy")]
pub mod bench_speedy;
#[cfg(feature = "wiring")]
pub mod bench_wiring;
pub mod datasets;

use core::{mem, ops};

use rand::Rng;

pub trait Generate {
    fn generate<R: Rng>(rng: &mut R) -> Self;
}

impl Generate for () {
    fn generate<R: Rng>(_: &mut R) -> Self {}
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
    };
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

impl<T: Generate, const N: usize> Generate for [T; N] {
    fn generate<R: Rng>(rng: &mut R) -> Self {
        let mut result = mem::MaybeUninit::<Self>::uninit();
        let result_ptr = result.as_mut_ptr().cast::<T>();
        for i in 0..N {
            unsafe {
                result_ptr.add(i).write(T::generate(rng));
            }
        }
        unsafe { result.assume_init() }
    }
}

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
    println!(
        "{}/{}/zstd_time {}",
        name,
        lib,
        bench_compression(|| zstd_size(bytes))
    );
}

fn bench_compression(compress: impl Fn() -> usize) -> String {
    let start = std::time::Instant::now();
    let size = compress();
    let elapsed = start.elapsed();

    let s = format!("{elapsed:.4?}");
    let (number, unit): (String, String) = s.chars().partition(|c| c.is_numeric() || *c == '.');
    let s = format!("{number} {unit}");

    let mb_per_second = ((size as f64 / 1_000_000.0) / elapsed.as_secs_f64()) as u64;
    format!("time:   [{s} {s} {s}] {mb_per_second} MB/s")
}

fn zlib_size(mut bytes: &[u8]) -> usize {
    let mut encoder = libflate::zlib::Encoder::new(Vec::new()).unwrap();
    std::io::copy(&mut bytes, &mut encoder).unwrap();
    encoder.finish().into_result().unwrap().len()
}

fn zstd_size(bytes: &[u8]) -> usize {
    zstd::stream::encode_all(bytes, 0).unwrap().len()
}
