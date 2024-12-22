#[cfg(feature = "capnp")]
pub mod mesh_capnp;
#[cfg(feature = "flatbuffers")]
#[path = "mesh_generated.rs"]
#[allow(unused_imports, clippy::all)]
pub mod mesh_fb;
#[cfg(feature = "prost")]
#[path = "prost.mesh.rs"]
pub mod mesh_prost;

#[cfg(feature = "flatbuffers")]
use flatbuffers::{FlatBufferBuilder, WIPOffset};
#[cfg(feature = "capnp")]
pub use mesh_capnp as cp;
#[cfg(feature = "flatbuffers")]
pub use mesh_fb::mesh as fb;
#[cfg(feature = "nanoserde")]
use nanoserde::{DeBin, SerBin};
use rand::Rng;
#[cfg(feature = "wiring")]
use wiring::prelude::{Unwiring, Wiring};

#[cfg(feature = "capnp")]
use crate::bench_capnp;
#[cfg(feature = "flatbuffers")]
use crate::bench_flatbuffers;
#[cfg(feature = "prost")]
use crate::bench_prost;
use crate::Generate;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]

pub struct Vector3 {
    #[cfg_attr(feature = "wiring", fixed)]
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Generate for Vector3 {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            x: rand.gen(),
            y: rand.gen(),
            z: rand.gen(),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl From<Vector3> for fb::Vector3 {
    #[inline]
    fn from(value: Vector3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Vector3 {
    type Reader = cp::vector3::Reader<'a>;
    type Builder = cp::vector3::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        builder.set_x(self.x);
        builder.set_y(self.y);
        builder.set_z(self.z);
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Vector3 {
    type Message = mesh_prost::Vector3;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[cfg(feature = "prost")]
impl From<mesh_prost::Vector3> for Vector3 {
    fn from(value: mesh_prost::Vector3) -> Self {
        Vector3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]
pub struct Triangle {
    #[cfg_attr(feature = "wiring", fixed)]
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub normal: Vector3,
}

impl Generate for Triangle {
    fn generate<R: Rng>(rand: &mut R) -> Self {
        Self {
            v0: Vector3::generate(rand),
            v1: Vector3::generate(rand),
            v2: Vector3::generate(rand),
            normal: Vector3::generate(rand),
        }
    }
}

#[cfg(feature = "flatbuffers")]
impl From<Triangle> for fb::Triangle {
    #[inline]
    fn from(value: Triangle) -> Self {
        Self::new(
            &value.v0.into(),
            &value.v1.into(),
            &value.v2.into(),
            &value.normal.into(),
        )
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Triangle {
    type Reader = cp::triangle::Reader<'a>;
    type Builder = cp::triangle::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        self.v0.serialize_capnp(&mut builder.reborrow().init_v0());
        self.v1.serialize_capnp(&mut builder.reborrow().init_v1());
        self.v2.serialize_capnp(&mut builder.reborrow().init_v2());
        self.normal
            .serialize_capnp(&mut builder.reborrow().init_normal());
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Triangle {
    type Message = mesh_prost::Triangle;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        Self::Message {
            v0: Some(self.v0.serialize_pb()),
            v1: Some(self.v1.serialize_pb()),
            v2: Some(self.v2.serialize_pb()),
            normal: Some(self.normal.serialize_pb()),
        }
    }
}

#[cfg(feature = "prost")]
impl From<mesh_prost::Triangle> for Triangle {
    fn from(value: mesh_prost::Triangle) -> Self {
        Triangle {
            v0: value.v0.unwrap().into(),
            v1: value.v1.unwrap().into(),
            v2: value.v2.unwrap().into(),
            normal: value.normal.unwrap().into(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "bilrost", derive(bilrost::Message))]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(
    feature = "borsh",
    derive(borsh::BorshSerialize, borsh::BorshDeserialize)
)]
#[cfg_attr(feature = "databuf", derive(databuf::Encode, databuf::Decode))]
#[cfg_attr(feature = "msgpacker", derive(msgpacker::MsgPacker))]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(
    feature = "scale",
    derive(parity_scale_codec_derive::Encode, parity_scale_codec_derive::Decode)
)]
#[derive(serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "simd-json",
    derive(simd_json_derive::Serialize, simd_json_derive::Deserialize)
)]
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[cfg_attr(feature = "savefile", derive(savefile_derive::Savefile))]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::SerBin, nanoserde::DeBin))]
#[cfg_attr(feature = "wiring", derive(Wiring, Unwiring))]
#[cfg_attr(feature = "serialization", derive(serialization::Serializable))]
pub struct Mesh {
    #[cfg_attr(feature = "bilrost", bilrost(encoding(packed)))]
    pub triangles: Vec<Triangle>,
}

#[cfg(feature = "flatbuffers")]
impl<'a> bench_flatbuffers::Serialize<'a> for Mesh {
    type Target = fb::Mesh<'a>;

    #[inline]
    fn serialize_fb<'b>(&self, fbb: &'b mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Target>
    where
        'a: 'b,
    {
        fbb.start_vector::<fb::Triangle>(self.triangles.len());
        for triangle in self.triangles.iter().cloned() {
            fbb.push::<fb::Triangle>(triangle.into());
        }
        let triangles = fbb.end_vector(self.triangles.len());

        let mut builder = fb::MeshBuilder::new(fbb);
        builder.add_triangles(triangles);
        builder.finish()
    }
}

#[cfg(feature = "capnp")]
impl<'a> bench_capnp::Serialize<'a> for Mesh {
    type Reader = cp::mesh::Reader<'a>;
    type Builder = cp::mesh::Builder<'a>;

    #[inline]
    fn serialize_capnp(&self, builder: &mut Self::Builder) {
        let mut mesh = builder
            .reborrow()
            .init_triangles(self.triangles.len() as u32);
        for (i, value) in self.triangles.iter().enumerate() {
            value.serialize_capnp(&mut mesh.reborrow().get(i as u32));
        }
    }
}

#[cfg(feature = "prost")]
impl bench_prost::Serialize for Mesh {
    type Message = mesh_prost::Mesh;

    #[inline]
    fn serialize_pb(&self) -> Self::Message {
        let mut result = Self::Message::default();
        for triangle in self.triangles.iter() {
            result.triangles.push(triangle.serialize_pb());
        }
        result
    }
}

#[cfg(feature = "prost")]
impl From<mesh_prost::Mesh> for Mesh {
    fn from(value: mesh_prost::Mesh) -> Self {
        Mesh {
            triangles: value.triangles.into_iter().map(Into::into).collect(),
        }
    }
}
