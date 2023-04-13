use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_service::Service;
use actix_web::{get, post, web::Json, web, App, HttpRequest, HttpServer, Result};
use actix_web::http::header::HeaderValue;
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

#[post("/system")]
async fn system_api(settings: Json<GenerationSettings>) -> Json<StarSystem> {
    let universe = Universe::generate(&settings);
    let neighborhood = GalacticNeighborhood::generate(universe, &settings);
    let mut galaxy = Galaxy::generate(neighborhood, 0, &settings);
    let coord = SpaceCoordinates::new(0, 0, 0);
    let sub_sector = galaxy
        .get_division_at_level(coord, 1)
        .expect("Should have returned a sub-sector.");
    let hex = galaxy.get_hex(coord).expect("Should have returned an hex.");
    let system = StarSystem::generate(0, coord, &hex, &sub_sector, &mut galaxy);
    Json(system)
}

#[get("/test-settings")]
async fn test_settings() -> Json<GenerationSettings> {
    Json(GenerationSettings {
        seed: String::from("default"),
        ..Default::default()
    })
}

#[get("/test-system")]
async fn test_system() -> Json<StarSystem> {
    Json(StarSystem {
        name: "Octupla".to_string(),
        center_id: 16,
        main_star_id: 0,
        all_objects: vec![
            OrbitalPoint {
                id: 1,
                primary_body_id: Some(3),
                distance_from_primary: Some(0.03745821439248134),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla Ba (M7 V)".to_string(),
                    mass: 0.114816464,
                    luminosity: 0.0017136049,
                    radius: 0.177,
                    age: 4.6,
                    temperature: 2791,
                    spectral_type: StarSpectralType::M(7),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 2,
                primary_body_id: Some(3),
                distance_from_primary: Some(0.04973226716467603),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla Bb (M9 V)".to_string(),
                    mass: 0.08647946,
                    luminosity: 0.0008467538,
                    radius: 0.141,
                    age: 4.6,
                    temperature: 2622,
                    spectral_type: StarSpectralType::M(9),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 0,
                primary_body_id: Some(4),
                distance_from_primary: Some(0.05513547187959574),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla A (G0 V)".to_string(),
                    mass: 1.234416,
                    luminosity: 4.4682164,
                    radius: 2.0020883,
                    age: 4.6,
                    temperature: 5931,
                    spectral_type: StarSpectralType::G(0),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 3,
                primary_body_id: Some(4),
                distance_from_primary: Some(0.3381097176403421),
                satellite_ids: vec![1, 2],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 4,
                primary_body_id: Some(6),
                distance_from_primary: Some(5.197310496129988),
                satellite_ids: vec![
                    0, // A
                    3, // B1 B2
                ],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 5,
                primary_body_id: Some(6),
                distance_from_primary: Some(20.900606604977213),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla C (K9 V)".to_string(),
                    mass: 0.35701552,
                    luminosity: 0.03640418,
                    radius: 0.439,
                    age: 4.6,
                    temperature: 3805,
                    spectral_type: StarSpectralType::K(9),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 6,
                primary_body_id: Some(8),
                distance_from_primary: Some(19.766755970567417),
                satellite_ids: vec![
                    4, // A
                    5, // C
                ],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 7,
                primary_body_id: Some(8),
                distance_from_primary: Some(266.1746531138536),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla D (M6 V)".to_string(),
                    mass: 0.13313216,
                    luminosity: 0.0025596572,
                    radius: 0.19899999,
                    age: 4.6,
                    temperature: 2910,
                    spectral_type: StarSpectralType::M(6),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 8,
                primary_body_id: Some(10),
                distance_from_primary: Some(309.734801744982),
                satellite_ids: vec![
                    6, // A
                    7, // D
                ],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 9,
                primary_body_id: Some(10),
                distance_from_primary: Some(2765.0653613474487),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla E (M4 V)".to_string(),
                    mass: 0.21572936,
                    luminosity: 0.010239862,
                    radius: 0.293,
                    age: 4.6,
                    temperature: 3392,
                    spectral_type: StarSpectralType::M(4),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 12,
                primary_body_id: Some(13),
                distance_from_primary: Some(0.0003740043956514335),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla Fa (DB VII)".to_string(),
                    mass: 0.6113024,
                    luminosity: 0.00029274347,
                    radius: 0.009897539,
                    age: 4.6,
                    temperature: 7589,
                    spectral_type: StarSpectralType::DB,
                    luminosity_class: StarLuminosityClass::VII,
                }),
            },
            OrbitalPoint {
                id: 11,
                primary_body_id: Some(13),
                distance_from_primary: Some(0.002122341315415619),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla Fb (M7 V)".to_string(),
                    mass: 0.10772526,
                    luminosity: 0.0014501228,
                    radius: 0.168,
                    age: 4.6,
                    temperature: 2748,
                    spectral_type: StarSpectralType::M(7),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 10,
                primary_body_id: Some(14),
                distance_from_primary: Some(8398.912409629105),
                satellite_ids: vec![8, 9],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 13,
                primary_body_id: Some(14),
                distance_from_primary: Some(25015.75579368719),
                satellite_ids: vec![12, 11],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 14,
                primary_body_id: Some(16),
                distance_from_primary: Some(62671.17324970373),
                satellite_ids: vec![10, 13],
                object: AstronomicalObject::Void,
            },
            OrbitalPoint {
                id: 15,
                primary_body_id: Some(16),
                distance_from_primary: Some(299130.7939912374),
                satellite_ids: vec![],
                object: AstronomicalObject::Star(Star {
                    name: "Octupla G (K4 V)".to_string(),
                    mass: 0.5993305,
                    luminosity: 0.12051362,
                    radius: 0.5913681,
                    age: 4.6,
                    temperature: 4421,
                    spectral_type: StarSpectralType::K(4),
                    luminosity_class: StarLuminosityClass::V,
                }),
            },
            OrbitalPoint {
                id: 16,
                primary_body_id: None,
                distance_from_primary: None,
                satellite_ids: vec![14, 15],
                object: AstronomicalObject::Void,
            },
        ],
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
            .service(test_system)
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
