fn main() {
    capnpc::CompilerCommand::new()
        .capnp_executable("tools/capnp.exe")
        .file("src/datasets/minecraft_savedata/minecraft_savedata.capnp")
        .output_path(".")
        .default_parent_module(vec!["datasets".into(), "minecraft_savedata".into()])
        .run()
        .expect("schema compiler command");
    capnpc::CompilerCommand::new()
        .capnp_executable("tools/capnp.exe")
        .file("src/datasets/log/log.capnp")
        .output_path(".")
        .default_parent_module(vec!["datasets".into(), "log".into()])
        .run()
        .expect("schema compiler command");
}
