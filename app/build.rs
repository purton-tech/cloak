use sha1::Sha1;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["api.proto"],     // Files in the path
            &["../api/protos"], // The path to search
        )
        .unwrap();

    // Asset pipelibe
    let mut data = String::new();

    data.push_str("use actix_files as fs;\n");
    data.push_str("use actix_web::{ get, HttpRequest, Error};\n");

    data.push_str(&create_route(
        "/static/images",
        "static_images",
        "./asset-pipeline/images/",
    ));
    data.push_str(&create_route("/static/assets", "static_file", "./dist/"));
    data.push_str(&parse_folder(".//dist/", "/static/assets"));
    data.push_str(&parse_folder("./asset-pipeline/images/", "/static/images"));

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("statics.rs");

    let mut dst = File::create(&file_path).unwrap();
    dst.write_all(data.as_bytes()).unwrap();
}

fn create_route(route: &str, method_name: &str, folder: &str) -> String {
    format!(
        r#"
        #[get("{}/{{filename:.*}}")]
        async fn {}(req: HttpRequest) -> Result<fs::NamedFile, Error> {{
            let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();

            let mut parts: Vec<&str> = path.to_str().unwrap().split('.').collect();
            parts.remove(parts.len() - 2);
            let name = parts.join(".");
            let name = format!("{}{{}}", name);

            if let Ok(file) = fs::NamedFile::open(name) {{
                return Ok(file);
            }} 
            
            let name = format!("{}{{}}", path.to_str().unwrap());
            let file = fs::NamedFile::open(name)?;
            Ok(file)
        }}
    "#,
        route, method_name, folder, folder
    )
}

fn parse_folder(folder: &str, route: &str) -> String {
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
