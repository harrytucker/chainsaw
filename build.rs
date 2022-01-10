fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/grpc")
        .include_file("mod.rs")
        .compile(&["proto/helloworld.proto"], &["proto"])?;
    Ok(())
}
