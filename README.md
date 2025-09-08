# Products API

Tiny REST API to manage products, built in Rust with **Actix Web**.

---

## Project Structure

```
src/
├── dto/           # Data Transfer Object (DTO) for creating and updating a product
├── handlers/      # Business logic for the endpoints
├── models/        # Data structures and domain structs
├── repository/    # Data access layer (in-memory)
├── routes/        # API endpoint definitions
├── validations/   # Field validations (price, stock, EAN-13)
├── errors.rs      # Error handling and standardized responses
└── main.rs        # Application entry point
```
---

## API Endpoints (Core)

| Method | Endpoint               | Description                                   |
|--------|-----------------------|-----------------------------------------------|
| POST   | /products             | Creates a product (id and timestamps set by server) |
| GET    | /products/{id}        | Retrieves a specific product                 |
| PATCH  | /products/{id}        | Partial update of a product                  |
| DELETE | /products/{id}        | Removes a product (hard delete, idempotent) |

### Example Usage

**Create Product**
```bash
curl -X POST http://localhost:8080/products \
  -H "Content-Type: application/json" \
  -d '{
    "sku": "1234",
    "product_name": "Milk",
    "category": "Dairy",
    "ean13": "7891234567895",
    "price_cents": 550,
    "currency": "BRL",
    "stock_count": 100
  }'
```

**Get Product**
```bash
curl http://localhost:8080/products/<product-id>
```

**Update Product**
```bash
curl -X PATCH http://localhost:8080/products/<product-id> \
  -H "Content-Type: application/json" \
  -d '{"price_cents": 600}'
```

**Delete Product**
```bash
curl -X DELETE http://localhost:8080/products/<product-id>
```

---

## Validations

- `product_name`: 3–120 characters  
- `price_cents` ≥ 0  
- `stock_count` ≥ 0  
- `ean13`: GS1 Mod-10 check digit  
- Seed loader: loads `seed_products.json` at startup, skipping duplicates by `sku` or `ean13`

---

## API Responses

- **POST /products**:  
  - `201 Created` → product created  
  - `400 Bad Request` → validation error  
  - `409 Conflict` → duplicate SKU or EAN-13

- **GET /products/{id}**:  
  - `200 OK` → product found  
  - `404 Not Found` → product does not exist

- **PATCH /products/{id}**:  
  - `200 OK` → product updated  
  - `404 Not Found` → product does not exist

- **DELETE /products/{id}**:  
  - `204 No Content` → product deleted (idempotent)

---

## Running the Project

### 1. Clone the repository
```bash
git clone https://github.com/matheusherman/products-api.git
cd products-api
```

### 2. Run Locally (with Rust installed)
```bash
cargo run
```
The server will be available at: `http://0.0.0.0:8080`

### 3. Run with Docker (without Dockerfile)
```bash
docker run --rm -it \
  -p 8080:8080 \
  -v "$(pwd)":/usr/src/myapp \
  -w /usr/src/myapp \
  rust:1.82 bash
```
Inside the container:
```bash
# Build the project
cargo build

# Run the application
cargo run
```
The server will be available at: `http://localhost:8080`

---

## Tests

```bash
cargo test
```

Tests include:
- EAN-13 validation  
- Validation for `price_cents`, `stock_count`, `product_name`  
- CRUD operations (create, get, patch, delete)
