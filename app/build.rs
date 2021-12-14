use sha1::Sha1;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    tonic_build::configure()
        .compile(
            &["api.proto"], // Files in the path
            &["../protos"], // The path to search
        )
        .unwrap();

    // Asset pipeline
    let mut data = String::new();

    data.push_str(&generate_file_routes("./dist/", "asset_pipeline_routes"));
    data.push_str(&generate_file_routes(
        "./asset-pipeline/images/",
        "image_routes",
    ));

    data.push_str(&generate_get_methods("./dist/", "/static/assets"));
    data.push_str(&generate_get_methods(
        "./asset-pipeline/images/",
        "/static/images",
    ));

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("statics.rs");

    let mut dst = File::create(&file_path).unwrap();
    dst.write_all(data.as_bytes()).unwrap();
}

fn generate_file_routes(folder: &str, method_name: &str) -> String {
    let paths = fs::read_dir(folder).unwrap();

    let mut data = String::new();

    data.push_str(&format!("pub fn {}() -> axum::Router {{\n", method_name));
    data.push_str("    axum::Router::new()\n");

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = std::fs::metadata(&path).unwrap();

        if metadata.is_file() {
            let name: String = path.file_name().unwrap().to_string_lossy().into();
            let file_name = format!("{}{}", folder, name);

            let method = format!(
                r#".route(
                    &get_{}(),
                    axum::routing::get_service(tower_http::services::ServeFile::new("{}")).handle_error(|error: std::io::Error| async move {{
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {{}}", error),
                        )
                    }}),
                )
                "#,
                &name.replace(".", "_").replace("-", "_"),
                &file_name
            );

            data.push_str(&method);
        }
    }

    data.push_str("}\n");
    data
}

fn generate_get_methods(folder: &str, route: &str) -> String {
    let paths = fs::read_dir(folder).unwrap();

    let mut data = String::new();

    for entry in paths {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = std::fs::metadata(&path).unwrap();

        if metadata.is_file() {
            let name: String = path.file_name().unwrap().to_string_lossy().into();
            let file_name = format!("{}{}", folder, name);

            println!("cargo:rerun-if-changed={}", folder);

            let hashed_filename = add_hash_to_file_name(&file_name, &name);

            let method = format!(
                r#"
                pub fn get_{}() -> String {{
                    "{}/{}".into()
                }}
            "#,
                &name.replace(".", "_").replace("-", "_"),
                route,
                &hashed_filename
            );

            data.push_str(&method);
        }
    }
    data
}

fn add_hash_to_file_name(file_name: &str, name: &str) -> String {
    let mut file = dbg!(std::fs::File::open(&file_name))
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {:?}", &file_name));

    let mut buffer = Vec::new();
    // read the whole file
    file.read_to_end(&mut buffer).unwrap();

    let hash = Sha1::from(buffer).digest().to_string();

    let mut parts: Vec<&str> = name.split('.').collect();
    parts.insert(parts.len() - 1, &hash);

    parts.join(".")
}
