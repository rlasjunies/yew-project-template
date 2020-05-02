// #[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use actix_files as fs;
use actix_web::{http, web};
use actix_web::{App, HttpResponse, HttpServer, Result as WebResult};
use clap::Arg;
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppConfig {
    spa: PathBuf,
    static_: PathBuf,
    port: String,
}

#[derive(Clone)]
pub struct AppState {
    config: AppConfig,
}

fn get_config() -> AppConfig {
    let app = clap::App::new("server")
        .about("Server")
        .version("0.1.0")
        .arg(
            Arg::with_name("spa")
                .long("spa")
                .value_name("DIR")
                .default_value("../spa/pkg")
                // .required(true)
                .help("Path to the directory containing UI js & wasm script files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("static")
                .long("static")
                .value_name("DIR")
                .default_value("../site")
                // .required(true)
                .help("Path to the directory containing HTML & static files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .default_value("8080")
                .help("Port for the HTTP server")
                .takes_value(true),
        );

    let matches = app.get_matches();
    return AppConfig {
        spa: PathBuf::from(matches.value_of("spa").expect("Missing spa parameter")),
        static_: PathBuf::from(matches.value_of("static").expect("Missing site parameter")),
        port: matches.value_of("port").unwrap_or("8080").to_string(),
    };
}

async fn index() -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, "/app/main")
        .finish()
}

async fn app_page(state: web::Data<AppState>) -> WebResult<fs::NamedFile> {
    Ok(fs::NamedFile::open(state.config.static_.join("index.html"))?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = get_config();

    let addr = format!("127.0.0.1:{}", config.port);
    info!("Listening on {}", addr);
    HttpServer::new(move || {
        let app_state = AppState {
            config: config.clone(),
        };

        App::new()
            .data(app_state)
            .service(fs::Files::new("/spa", &config.spa))
            .service(fs::Files::new("/static", &config.static_.join("static")))
            .route("/", web::get().to(index))
            .route("/app{_:/?}", web::get().to(index))
            .route("/app/{app:[a-zA-z0-9_\\-/]+}", web::get().to(app_page))
    })
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
