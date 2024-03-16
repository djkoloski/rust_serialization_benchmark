#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector3 {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
    #[prost(float, tag = "3")]
    pub z: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Triangle {
    #[prost(message, optional, tag = "1")]
    pub v0: ::core::option::Option<Vector3>,
    #[prost(message, optional, tag = "2")]
    pub v1: ::core::option::Option<Vector3>,
    #[prost(message, optional, tag = "3")]
    pub v2: ::core::option::Option<Vector3>,
    #[prost(message, optional, tag = "4")]
    pub normal: ::core::option::Option<Vector3>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mesh {
    #[prost(message, repeated, tag = "1")]
    pub triangles: ::prost::alloc::vec::Vec<Triangle>,
}
