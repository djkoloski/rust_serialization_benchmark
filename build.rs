use std::{
    env,
    path::{Path, PathBuf},
};

// TODO: re-enable bebop
#[allow(dead_code)]
fn bebop_compile_dataset(name: &'static str) {
    bebop_tools::download_bebopc(PathBuf::from("target").join("bebopc"));

    bebop_tools::build_schema(
        format!("./src/datasets/{0}/{0}.bop", name),
        format!("./src/datasets/{0}/{0}_bebop.rs", name),
        &bebop_tools::BuildConfig::default(),
    );
}

fn capnpc_compile_dataset(name: &'static str) -> capnp::Result<()> {
    let mut command = capnpc::CompilerCommand::new();
    #[cfg(windows)]
    command.capnp_executable("prebuilt/capnp.exe");
    command.file(&format!("src/datasets/{0}/{0}.capnp", name));
    command.output_path(".");
    command.default_parent_module(vec!["datasets".into(), name.into()]);
    command.run()
}

fn flatc_compile_dataset(name: &'static str) -> flatc_rust::Result<()> {
    #[cfg(windows)]
    let flatc = flatc_rust::Flatc::from_path("./prebuilt/flatc.exe");
    #[cfg(not(windows))]
    let flatc = flatc_rust::Flatc::from_env_path();

    flatc.run(flatc_rust::Args {
        lang: "rust",
        inputs: &[Path::new(&format!("./src/datasets/{0}/{0}.fbs", name))],
        out_dir: Path::new(&format!("./src/datasets/{}", name)),
        extra: &["--gen-onefile"],
        ..Default::default()
    })
}

#[cfg(feature = "prost-build")]
fn prost_compile_dataset(name: &'static str) -> std::io::Result<()> {
    if cfg!(windows) {
        match env::var("PROTOC") {
            Err(_) => {
                env::set_var("PROTOC", "./prebuilt/protoc.exe");
            }
            _ => {}
        }
    }
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    prost_config.compile_protos(
        &[format!("./src/datasets/{0}/{0}.proto", name).as_str()],
        &["src"],
    )
}

fn main() {
    const DATASETS: &[&str] = &["log", "mesh", "minecraft_savedata", "mk48"];
    for &name in DATASETS.iter() {
        // bebop_compile_dataset(name);
        #[cfg(all(feature = "capnp", not(feature = "use-committed-capnp")))]
        capnpc_compile_dataset(name).unwrap();
        #[cfg(all(feature = "flatbuffers", not(feature = "use-committed-flatbuffers")))]
        flatc_compile_dataset(name).unwrap();
        #[cfg(feature = "prost-build")]
        prost_compile_dataset(name).unwrap();
    }
}
