fn main() {
    capnpc::CompilerCommand::new()
        .capnp_executable("tools/capnp.exe")
        .file("src/datasets/log/log.capnp")
        .output_path(".")
        .default_parent_module(vec!["datasets".into(), "log".into()])
        .run()
        .unwrap();
    capnpc::CompilerCommand::new()
        .capnp_executable("tools/capnp.exe")
        .file("src/datasets/mesh/mesh.capnp")
        .output_path(".")
        .default_parent_module(vec!["datasets".into(), "mesh".into()])
        .run()
        .unwrap();
    capnpc::CompilerCommand::new()
        .capnp_executable("tools/capnp.exe")
        .file("src/datasets/minecraft_savedata/minecraft_savedata.capnp")
        .output_path(".")
        .default_parent_module(vec!["datasets".into(), "minecraft_savedata".into()])
        .run()
        .unwrap();
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    prost_config.compile_protos(&["src/datasets/log/log.proto"], &["src"])
        .unwrap();
    prost_config.compile_protos(&["src/datasets/mesh/mesh.proto"], &["src"])
        .unwrap();
    prost_config.compile_protos(&["src/datasets/minecraft_savedata/minecraft_savedata.proto"], &["src"])
        .unwrap();
}
