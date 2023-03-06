fn main() {
    // Compile our proto
    tonic_build::configure()
        // We want to be able to convert proto to json, this enables that.
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        // Prost will automatically include it's own prosy_types for google Timestamp etc.
        // But this doesn't have Serialice and Deseriealize so we switch it off.
        // Maybe we could use https://github.com/fdeantoni/prost-wkt
        .compile_well_known_types(true)
        .compile(&["api.proto"], &["./"])
        .unwrap();

    // Build a Web gRPC client. As cloudflare doesn't support gRPC.
    let path = std::path::PathBuf::from(format!("{}/grpc_web", std::env::var("OUT_DIR").unwrap()));
    std::fs::create_dir_all(&path).unwrap();
    rust_grpc_web::configure()
        // Don't generate streaming support (it's not working)
        .support_streaming(false)
        .out_dir(path)
        .compile(&["api.proto"], &["./"])
        .unwrap();
}
