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
}
