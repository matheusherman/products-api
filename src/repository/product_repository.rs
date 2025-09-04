use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use chrono::Utc;
use serde::Deserialize;
use log::{info, error};

use crate::models::product::Product;
use crate::dto::product_dto::{CreateProduct, UpdateProduct};
use crate::errors::ApiError;

pub type Store = Arc<RwLock<HashMap<Uuid, Product>>>;

pub struct ProductRepository;

impl ProductRepository {
    pub fn create(
        map: &mut HashMap<Uuid, Product>,
        dto: CreateProduct,
    ) -> Result<Product, ApiError> {
        info!("Criando produto (sku = {}, ean13 = {})", dto.sku, dto.ean13);

        Self::validate_create(&dto)?;
        Self::ensure_unique(&map, &dto.sku, &dto.ean13, None)?;

        let now = Utc::now();
        let product = Product {
            id: Uuid::new_v4(),
            sku: dto.sku,
            product_name: dto.product_name,
            category: dto.category,
            ean13: dto.ean13,
            price_cents: dto.price_cents,
            currency: dto.currency.unwrap_or_else(|| "BRL".to_string()),
            stock_count: dto.stock_count,
            created_at: now,
            updated_at: now,
        };

        map.insert(product.id, product.clone());

        info!("Produto criado com sucesso (id = {})", product.id);
        Ok(product)
    }

    pub fn get(map: &HashMap<Uuid, Product>, id: &Uuid) -> Result<Product, ApiError> {
        info!("Buscando produto (id = {})", id);
        let result = map.get(id).cloned();
        match result {
            Some(ref p) => info!("Produto encontrado (id = {}, sku = {})", p.id, p.sku),
            None => error!("Produto não encontrado (id = {})", id),
        }
        result.ok_or(ApiError::NotFound)
    }

    pub fn patch(
        map: &mut HashMap<Uuid, Product>,
        id: Uuid,
        dto: UpdateProduct,
    ) -> Result<Product, ApiError> {
        info!("Atualizando produto (id = {})", id);

        Self::validate_patch(&dto)?;

        let (new_sku, new_ean) = {
            let existing = map.get(&id).ok_or(ApiError::NotFound)?;
            (
                dto.sku.as_deref().unwrap_or(&existing.sku).to_string(),
                dto.ean13.as_deref().unwrap_or(&existing.ean13).to_string(),
            )
        };

        Self::ensure_unique(&map, &new_sku, &new_ean, Some(id))?;

        let existing = map.get_mut(&id).ok_or(ApiError::NotFound)?;

        if let Some(v) = dto.sku { existing.sku = v; }
        if let Some(v) = dto.product_name { existing.product_name = v; }
        if let Some(v) = dto.category { existing.category = v; }
        if let Some(v) = dto.ean13 { existing.ean13 = v; }
        if let Some(v) = dto.price_cents { existing.price_cents = v; }
        if let Some(v) = dto.currency { existing.currency = v; }
        if let Some(v) = dto.stock_count { existing.stock_count = v; }

        existing.updated_at = Utc::now();

        info!("Produto atualizado com sucesso (id = {})", id);
        Ok(existing.clone())
    }

    pub fn delete(map: &mut HashMap<Uuid, Product>, id: &Uuid) -> Result<(), ApiError> {
        info!("Deletando produto (id = {})", id);
        match map.remove(id) {
            Some(_) => {
                info!("Produto deletado com sucesso (id = {})", id);
                Ok(())
            }
            None => {
                error!("Produto não encontrado para deletar (id = {})", id);
                Err(ApiError::NotFound)
            }
        }
    }

    pub fn validate_create(dto: &CreateProduct) -> Result<(), ApiError> {
        if dto.sku.is_empty() || dto.ean13.is_empty() {
            error!("Falha na validação: SKU ou EAN13 vazio");
            return Err(ApiError::BadRequest("SKU ou EAN13 vazio".into()));
        }
        Ok(())
    }

    pub fn validate_patch(_dto: &UpdateProduct) -> Result<(), ApiError> {
        Ok(())
    }

    pub fn ensure_unique(
        map: &HashMap<Uuid, Product>,
        sku: &str,
        ean: &str,
        id: Option<Uuid>,
    ) -> Result<(), ApiError> {
        for (_, p) in map {
            if Some(p.id) != id {
                if p.sku == sku || p.ean13 == ean {
                    error!("Violação de unicidade: SKU ou EAN13 já existe (sku = {}, ean13 = {})", sku, ean);
                    return Err(ApiError::BadRequest("SKU ou EAN13 já existe".into()));
                }
            }
        }
        Ok(())
    }
}

#[derive(Deserialize)]
struct Seed {
    products: Vec<Product>,
}

impl ProductRepository {
    pub fn load_seed(store: &Store, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        info!("Carregando seed do arquivo: {}", path);

        let file = std::fs::File::open(path)?;
        let seed: Seed = serde_json::from_reader(file)?;
        let mut map = store.write().unwrap();

        for product in seed.products {
            log::info!("Carregando produto da seed (id = {}, sku = {})", product.id, product.sku);
            map.insert(product.id, product);
        }

        info!("Seed carregada com sucesso (total = {})", map.len());
        Ok(())
    }
}
