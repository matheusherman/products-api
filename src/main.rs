use actix_web::{App, HttpServer};
use std::{collections::HashMap, sync::{Arc, RwLock}};
use actix_web::middleware::Logger;
use uuid::Uuid;
use std::env;

mod models;
mod dto;
mod repository;
mod routes;
mod handlers;
mod validations;
pub mod errors;

use models::product::Product;
use repository::product_repository::ProductRepository;
use routes::product_router::config_product_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let store: Arc<RwLock<HashMap<Uuid, Product>>> = Arc::new(RwLock::new(HashMap::new()));


    log::info!("Loading seed file: {}", "seed_products.json");

    match ProductRepository::load_seed(&store, "seed_products.json") {
        Ok(_) => {
            let repo = store.read().unwrap();
            log::info!("Seed loaded with {} products", repo.len());
        }
        Err(e) => log::error!("Failed to load seed: {:?}", e),
    }      

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(store.clone()))
            .configure(config_product_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// fonte: www.fullstackmaster.io/2024/12/30/rust-api-com-actix-como-construir-seu-primeiro-crud-completo/