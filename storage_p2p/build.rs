use std::fs;

fn main() {
    tonic_build::configure()
        .build_server(false)
        .out_dir("./src")
        .compile(
            &["./protos/data.proto"],
            &["./protos"],
        ).unwrap();
    fs::rename("./src/_.rs", "./src/protobuf.rs").unwrap();
}