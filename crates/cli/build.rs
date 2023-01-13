fn main() {
    rust_grpc_web::configure()
        // Don't generate streaming support (it's not working)
        .support_streaming(false)
        .compile(&["api.proto"], &["../grpc-api"])
        .unwrap();
}
