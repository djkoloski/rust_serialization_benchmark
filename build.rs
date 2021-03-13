fn capnpc_compile_dataset(name: &'static str) -> capnp::Result<()> {
    let mut command = capnpc::CompilerCommand::new();
    #[cfg(windows)]
    command.capnp_executable("tools/capnp.exe");
    command.file(&format!("src/datasets/{0}/{0}.capnp", name));
    command.output_path(".");
    command.default_parent_module(vec!["datasets".into(), name.into()]);
    command.run()
}

fn prost_compile_dataset(name: &'static str) -> std::io::Result<()> {
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    prost_config.compile_protos(&[format!("src/datasets/{0}/{0}.proto", name).as_str()], &["src"])
}

fn main() {
    const DATASETS: [&'static str; 3] = ["log", "mesh", "minecraft_savedata"];
    for &name in DATASETS.iter() {
        capnpc_compile_dataset(name).unwrap();
        prost_compile_dataset(name).unwrap();
    }
}
