#[allow(unused_imports)]
use std::{
    env,
    path::{Path, PathBuf},
};

#[cfg(feature = "regenerate-capnp")]
fn capnpc_compile_dataset(name: &'static str) -> capnp::Result<()> {
    let mut command = capnpc::CompilerCommand::new();
    #[cfg(windows)]
    command.capnp_executable("prebuilt/capnp.exe");
    command.file(format!("src/datasets/{0}/{0}.capnp", name));
    command.output_path(".");
    command.default_parent_module(vec!["datasets".into(), name.into()]);
    command.run()
}

#[cfg(feature = "regenerate-flatbuffers")]
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

#[cfg(feature = "regenerate-prost")]
fn prost_compile_dataset(name: &'static str) -> std::io::Result<()> {
    if cfg!(windows) && env::var("PROTOC").is_err() {
        env::set_var("PROTOC", "./prebuilt/protoc.exe");
    }
    let mut prost_config = prost_build::Config::new();
    prost_config.protoc_arg("--experimental_allow_proto3_optional");
    prost_config.out_dir(format!("./src/datasets/{name}"));
    prost_config.compile_protos(
        &[format!("./src/datasets/{name}/{name}.proto").as_str()],
        &["src"],
    )
}

#[cfg(feature = "regenerate-protobuf")]
fn protobuf_compile_dataset(name: &'static str) -> std::io::Result<()> {
    if cfg!(windows) && env::var("PROTOC").is_err() {
        env::set_var("PROTOC", "./prebuilt/protoc.exe");
    }
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_extra_arg("--experimental_allow_proto3_optional")
        .out_dir(format!("./src/datasets/{name}/{name}_protobuf"))
        .inputs(&[format!("./src/datasets/{name}/{name}.proto")])
        .include(format!("./src/datasets/{name}/"))
        .run()
        .unwrap();

    Ok(())
}

fn main() {
    #[cfg(any(
        feature = "regenerate-capnp",
        feature = "regenerate-flatbuffers",
        feature = "regenerate-prost",
        feature = "regenerate-protobuf"
    ))]
    {
        const DATASETS: &[&str] = &["log", "mesh", "minecraft_savedata", "mk48"];
        for &name in DATASETS.iter() {
            #[cfg(feature = "regenerate-capnp")]
            capnpc_compile_dataset(name).unwrap();
            #[cfg(feature = "regenerate-flatbuffers")]
            flatc_compile_dataset(name).unwrap();
            #[cfg(feature = "regenerate-prost")]
            prost_compile_dataset(name).unwrap();
            #[cfg(feature = "regenerate-protobuf")]
            protobuf_compile_dataset(name).unwrap();
        }
    }
}
