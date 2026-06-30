//! This workspace crate exists purely because the codegen for protobuf 4.0 conflicts with the
//! codegen for 3.7.2: both of them generate code referencing items under the path `::protobuf`, so
//! we have to place the generated code for this version of the crate in a different crate where
//! `::protobuf` actually references the correct version of the library.

#[path = "log/generated.rs"]
pub mod log;
#[path = "mesh/generated.rs"]
pub mod mesh;
#[path = "minecraft_savedata/generated.rs"]
pub mod minecraft_savedata;
#[path = "mk48/generated.rs"]
pub mod mk48;

impl From<minecraft_savedata::Vector3dView<'_>> for (f64, f64, f64) {
    fn from(value: minecraft_savedata::Vector3dView) -> Self {
        (value.x(), value.y(), value.z())
    }
}

impl From<minecraft_savedata::Vector2fView<'_>> for (f32, f32) {
    fn from(value: minecraft_savedata::Vector2fView) -> Self {
        (value.x(), value.y())
    }
}

impl From<minecraft_savedata::UuidView<'_>> for [u32; 4] {
    fn from(value: minecraft_savedata::UuidView) -> Self {
        [value.x0(), value.x1(), value.x2(), value.x3()]
    }
}

impl From<mk48::Vector2fView<'_>> for (f32, f32) {
    fn from(value: mk48::Vector2fView) -> Self {
        (value.x(), value.y())
    }
}

impl From<mk48::ChunkIdView<'_>> for (i8, i8) {
    fn from(value: mk48::ChunkIdView) -> Self {
        (value.x().try_into().unwrap(), value.y().try_into().unwrap())
    }
}
