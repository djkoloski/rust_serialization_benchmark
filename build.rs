use std::path::{Path, PathBuf};

fn bebop_compile_dataset(name: &'static str) {
    bebop_tools::download_bebopc(PathBuf::from("target").join("bebopc"));

    bebop_tools::build_schema(
        format!("./src/datasets/{0}/{0}.bop", name),
        format!("./src/datasets/{0}/{0}_bebop.rs", name),
    );
}

fn capnpc_compile_dataset(name: &'static str) -> capnp::Result<()> {
    let mut command = capnpc::CompilerCommand::new();
    #[cfg(windows)]
    command.capnp_executable("tools/capnp.exe");
    command.file(&format!("src/datasets/{0}/{0}.capnp", name));
    command.output_path(".");
    command.default_parent_module(vec!["datasets".into(), name.into()]);
    command.run()
}

fn flatc_compile_dataset(name: &'static str) -> flatc_rust::Result<()> {
    #[cfg(windows)]
    let flatc = flatc_rust::Flatc::from_path("./tools/flatc.exe");
    #[cfg(not(windows))]
    let flatc = flatc_rust::Flatc::from_env_path();

    flatc.run(flatc_rust::Args {
        lang: "rust",
        inputs: &[Path::new(&format!("./src/datasets/{0}/{0}.fbs", name))],
        out_dir: Path::new(&format!("./src/datasets/{}", name)),
        ..Default::default()
    })
}

fn prost_compile_dataset(name: &'static str) -> std::io::Result<()> {
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    prost_config.compile_protos(&[format!("./src/datasets/{0}/{0}.proto", name).as_str()], &["src"])
}

fn main() {
    const DATASETS: [&'static str; 3] = ["log", "mesh", "minecraft_savedata"];
    for &name in DATASETS.iter() {
        bebop_compile_dataset(name);
        capnpc_compile_dataset(name).unwrap();
        flatc_compile_dataset(name).unwrap();
        prost_compile_dataset(name).unwrap();
    }
}
