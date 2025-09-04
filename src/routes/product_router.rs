use actix_web::web;

use crate::handlers::product_handler;

pub fn config_product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("", web::post().to(product_handler::create_product))
            .route("/{id}", web::get().to(product_handler::get_product))
            .route("/{id}", web::patch().to(product_handler::patch_product))
            .route("/{id}", web::delete().to(product_handler::delete_product)),
    );
}
