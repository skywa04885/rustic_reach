fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir("./src")
        .compile(&["./proto/arm.proto"], &["./proto"])?;

    Ok(())
}
