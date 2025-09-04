use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use uuid::Uuid;
use crate::repository::product_repository::Store;
use log::{info, error};

use crate::{
    dto::product_dto::{CreateProduct, UpdateProduct},
    errors::ApiError,
    models::product::Product,
    repository::product_repository::ProductRepository
};

pub async fn create_product(
    state: Data<Store>,
    Json(body): Json<CreateProduct>,
) -> Result<HttpResponse, ApiError> {
    info!(">> Entrou no handler: create_product (sku = {})", body.sku);

    let mut repo = state.write().map_err(|_| ApiError::Internal)?;
    let product = ProductRepository::create(&mut repo, body)?;

    Ok(HttpResponse::Created().json(product))
}




pub async fn get_product(
    state: Data<Store>,
    path: Path<Uuid>,
) -> Result<Json<Product>, ApiError> {
    let id = path.into_inner();
    info!(">> Entrou no handler: get_product (id = {})", id);

    let repo = state.read().map_err(|_| {
        error!("Falha ao obter lock de leitura no store");
        ApiError::Internal
    })?;

    let product = ProductRepository::get(&repo, &id)?;
    info!("Produto retornado com id = {}", product.id);

    Ok(Json(product))
}

pub async fn patch_product(
    state: Data<Store>,
    path: Path<Uuid>,
    Json(body): Json<UpdateProduct>,
) -> Result<Json<Product>, ApiError> {
    let id = path.into_inner();
    let mut repo = state.write().map_err(|_| ApiError::Internal)?;
    let product = ProductRepository::patch(&mut repo, id, body)?;
    Ok(Json(product))
}

pub async fn delete_product(
    state: Data<Store>,
    path: Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let mut repo = state.write().map_err(|_| ApiError::Internal)?;
    ProductRepository::delete(&mut repo, &id)?;
    Ok(HttpResponse::NoContent().finish())
}
