#[path="mesh.u.pb.rs"]
#[allow(nonstandard_style, unused, unreachable_pub)]
#[doc(hidden)]
mod internal_do_not_use_mesh;

#[allow(nonstandard_style, unused)]
#[doc(inline)]
pub use internal_do_not_use_mesh::*;
#[allow(nonstandard_style, unused)]
pub mod __unstable {
pub static MESH_DESCRIPTOR_INFO: ::protobuf::__internal::runtime::__unstable::DescriptorInfo = ::protobuf::__internal::runtime::__unstable::DescriptorInfo {
  descriptor: b"\n\nmesh.proto\x12\nprost.mesh\"*\n\x07Vector3\x12\t\n\x01x\x18\x01 \x01(\x02\x12\t\n\x01y\x18\x02 \x01(\x02\x12\t\n\x01z\x18\x03 \x01(\x02\"\x92\x01\n\x08Triangle\x12\x1f\n\x02v0\x18\x01 \x01(\x0b\x32\x13.prost.mesh.Vector3\x12\x1f\n\x02v1\x18\x02 \x01(\x0b\x32\x13.prost.mesh.Vector3\x12\x1f\n\x02v2\x18\x03 \x01(\x0b\x32\x13.prost.mesh.Vector3\x12#\n\x06normal\x18\x04 \x01(\x0b\x32\x13.prost.mesh.Vector3\"/\n\x04Mesh\x12\'\n\ttriangles\x18\x01 \x03(\x0b\x32\x14.prost.mesh.Triangleb\x06proto3",
  deps: &[
  ],
};
}
