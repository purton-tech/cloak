use ructe::{Result, Ructe};
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<()> {
    tonic_build::configure()
        .compile(
            &["api.proto"], // Files in the path
            &["../protos"], // The path to search
        )
        .unwrap();

    cornucopia()?;

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path_buf = PathBuf::from(format!("{}/ructe", &out_dir.into_string().unwrap()));

    ructe()?;

    // See https://github.com/kaj/ructe/issues/112
    //pub content: &'static [u8],
    let file_path = path_buf.join("templates/statics.rs");

    // s/pub content: \&\x27static \[u8\]/pub file_name: \&\x27static str/g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/pub content: \&\x27static \[u8\]/pub file_name: \&\x27static str/g")
        .arg(file_path.clone())
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    // s/content: include_bytes!(/file_name: /g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/content: include_bytes!(/file_name: /g")
        .arg(file_path.clone())
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    // s/),/,/g
    let output = std::process::Command::new("sed")
        .arg("-i")
        .arg(r"s/),/,/g")
        .arg(file_path)
        .output()?;
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    Ok(())
}

fn ructe() -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let path_buf = PathBuf::from(format!("{}/ructe", &out_dir.into_string().unwrap()));
    let mut ructe = Ructe::new(path_buf).unwrap();
    let mut statics = ructe.statics().unwrap();
    statics.add_files("dist").unwrap();
    statics.add_files("asset-pipeline/images").unwrap();

    ructe.compile_templates("templates").unwrap();

    Ok(())
}

fn cornucopia() -> Result<()> {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";

    // Again, for simplicity, we generate the module in our project, but
    // we could've also generated it elsewhere if we wanted to.
    // For example, you could make the destination the `target` folder
    // and include the generated file with a `include_str` statement in your project.

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("cornucopia.rs");

    let db_url = env::var_os("DATABASE_URL").unwrap();

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");

    // Call cornucopia. Use whatever CLI command you need.
    let output = std::process::Command::new("cornucopia")
        .arg("-q")
        .arg(queries_path)
        .arg("-d")
        .arg(&file_path)
        .arg("live")
        .arg(db_url)
        .output()?;

    // If Cornucopia couldn't run properly, try to display the error.
    if !output.status.success() {
        panic!("{}", &std::str::from_utf8(&output.stderr).unwrap());
    }

    Ok(())
}
