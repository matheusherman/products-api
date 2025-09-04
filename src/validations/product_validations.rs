use crate::dto::product_dto::{CreateProduct, UpdateProduct};
use crate::errors::ApiError;
use crate::models::product::Product;
use std::collections::HashMap;
use uuid::Uuid;

pub fn ensure_unique(
    map: &HashMap<Uuid, Product>,
    sku: &str,
    ean: &str,
    id: Option<Uuid>,
) -> Result<(), ApiError> {
    for (_, p) in map {
        if Some(p.id) != id {
            if p.sku == sku || p.ean13 == ean {
                return Err(ApiError::Conflict("SKU ou EAN13 já existe".into()));
            }
        }
    }
    Ok(())
}

/// Valida um EAN-13 pelo algoritmo GS1 mod-10
pub fn is_valid_ean13(ean: &str) -> bool {
    if ean.len() != 13 || !ean.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    let digits: Vec<u32> = ean.chars().filter_map(|c| c.to_digit(10)).collect();
    let check_digit = digits[12];

    let sum: u32 = digits[..12]
        .iter()
        .enumerate()
        .map(|(i, d)| if i % 2 == 0 { *d } else { *d * 3 })
        .sum();

    let calc_digit = (10 - (sum % 10)) % 10;
    check_digit == calc_digit
}

/// Valida a criação de um produto
pub fn validate_create(dto: &CreateProduct) -> Result<(), ApiError> {
    if dto.product_name.len() < 3 || dto.product_name.len() > 120 {
        return Err(ApiError::BadRequest(
            "product_name deve ter entre 3 e 120 caracteres".into(),
        ));
    }

    if dto.price_cents < 0 {
        return Err(ApiError::BadRequest("price_cents deve ser >= 0".into()));
    }

    if dto.stock_count < 0 {
        return Err(ApiError::BadRequest("stock_count deve ser >= 0".into()));
    }

    if !is_valid_ean13(&dto.ean13) {
        return Err(ApiError::BadRequest("ean13 inválido".into()));
    }

    Ok(())
}

/// Valida atualização parcial (PATCH)
pub fn validate_patch(dto: &UpdateProduct) -> Result<(), ApiError> {
    if let Some(ref name) = dto.product_name {
        if name.len() < 3 || name.len() > 120 {
            return Err(ApiError::BadRequest(
                "product_name deve ter entre 3 e 120 caracteres".into(),
            ));
        }
    }

    if let Some(price) = dto.price_cents {
        if price < 0 {
            return Err(ApiError::BadRequest("price_cents deve ser >= 0".into()));
        }
    }

    if let Some(stock) = dto.stock_count {
        if stock < 0 {
            return Err(ApiError::BadRequest("stock_count deve ser >= 0".into()));
        }
    }

    if let Some(ref ean) = dto.ean13 {
        if !is_valid_ean13(ean) {
            return Err(ApiError::BadRequest("ean13 inválido".into()));
        }
    }

    Ok(())
}
