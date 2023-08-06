use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{get, post, web::Json, web, App, HttpRequest, HttpServer, Result};
use actix_web::middleware::Logger;
use chrono::Local;
use fern::{Dispatch, log_file};
use log::LevelFilter;
use planet_generator::prelude::*;
use std::path::PathBuf;

#[post("/universe")]
async fn universe_api(settings: Json<GenerationSettings>) -> Json<Universe> {
    let universe = Universe::generate(&settings);
    Json(universe)
}

#[post("/galaxy")]
async fn galaxy_api(settings: Json<GenerationSettings>) -> Json<Galaxy> {
    let universe = Universe::generate(&settings);
    let neighborhood = GalacticNeighborhood::generate(universe, &settings);
    let galaxy = Galaxy::generate(neighborhood, 0, &settings);
    Json(galaxy)
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
struct SystemPayload {
    pub settings: GenerationSettings,
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[post("/system")]
async fn system_api(payload: Json<SystemPayload>) -> Json<StarSystem> {
    let settings = &payload.settings;
    let universe = Universe::generate(settings);
    let neighborhood = GalacticNeighborhood::generate(universe, settings);
    let mut galaxy = Galaxy::generate(neighborhood, 0, settings);
    let coord = SpaceCoordinates::new(payload.x, payload.y, payload.z);
    let sub_sector = galaxy
        .get_division_at_level(coord, 1)
        .expect("Should have returned a sub-sector.");
    let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
    let system = StarSystem::generate(0, coord, &hex, &sub_sector, &mut galaxy);
    println!("{:#?}", system);
    Json(system)
}

#[get("/test-settings")]
async fn test_settings() -> Json<GenerationSettings> {
    Json(GenerationSettings {
        seed: String::from("default"),
        ..Default::default()
    })
}

async fn index() -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".into();
    Ok(NamedFile::open(path)?)
}

async fn serve_static(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("tail").parse().unwrap();
    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    Ok(NamedFile::open(path)?.set_content_type(mime))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let base_config = Dispatch::new()
        .level(LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(
            log_file("actix-web.log").expect("Failed to create log file"),
        );

    let config = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .chain(base_config);

    config.apply().expect("Failed to set up logging");

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(universe_api)
            .service(galaxy_api)
            .service(system_api)
            .service(test_settings)
           // .service(test_system)
            .route("/", web::get().to(index))
            .service(
                Files::new("/", "static")
                    .prefer_utf8(true)
                    .default_handler(web::get().to(serve_static)),
            )
    })
    .bind("127.0.0.1:8042")?
    .run()
    .await
}
