use actix_web::{web::Data, App, HttpServer};
mod config;
mod errors;
mod layout;
mod vaults;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::new();
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .configure(vaults::routes)
            .service(statics::static_images)
            .service(statics::static_file)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}

// Error here disabled with "rust-analyzer.diagnostics.disabled": ["macro-error"]
// in .vscode/settings.json
pub mod statics {
    include!(concat!(env!("OUT_DIR"), "/statics.rs"));
}
