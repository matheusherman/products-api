# Products API

Uma API REST para gerenciamento de produtos construída com Rust e Axum.

## Funcionalidades

- ✅ **POST /products** - Criar um novo produto
- ✅ **GET /products/{id}** - Buscar um produto por ID
- ✅ **PATCH /products/{id}** - Atualizar um produto (atualização parcial)
- ✅ **DELETE /products/{id}** - Deletar um produto

## Como executar

### Pré-requisitos

1. Instale o Rust: https://rustup.rs/
2. Execute o comando para instalar as dependências e compilar:

```bash
cargo build
```

3. Execute o servidor:

```bash
cargo run
```

O servidor estará rodando em `http://localhost:3000`

## Exemplos de uso

### Criar um produto (POST /products)

```bash
curl -X POST http://localhost:3000/products \
  -H "Content-Type: application/json" \
  -d '{
    "sku": "PROD001",
    "product_name": "Produto Exemplo",
    "category": "Eletrônicos",
    "ean13": "1234567890123",
    "price_cents": 2999,
    "currency": "BRL",
    "stock_count": 100
  }'
```

### Buscar um produto (GET /products/{id})

```bash
curl http://localhost:3000/products/{product-id}
```

### Atualizar um produto (PATCH /products/{id})

```bash
curl -X PATCH http://localhost:3000/products/{product-id} \
  -H "Content-Type: application/json" \
  -d '{
    "price_cents": 3499,
    "stock_count": 50
  }'
```

### Deletar um produto (DELETE /products/{id})

```bash
curl -X DELETE http://localhost:3000/products/{product-id}
```

## Estrutura do projeto

```
src/
├── main.rs              # Ponto de entrada da aplicação
├── models/
│   └── product.rs       # Modelos de dados (Product, CreateProductRequest, UpdateProductRequest)
└── routes/
    └── productRoutes.rs # Handlers dos endpoints
```

## Validações implementadas

- **SKU, product_name e category**: Campos obrigatórios
- **EAN13**: Deve ter exatamente 13 caracteres
- **price_cents e stock_count**: Devem ser valores positivos
- **ID**: Deve ser um UUID válido
- **Currency**: Padrão "BRL" se não especificado

## Códigos de status HTTP

- **201**: Produto criado com sucesso
- **200**: Operação realizada com sucesso
- **204**: Produto deletado com sucesso
- **400**: Dados inválidos ou malformados
- **404**: Produto não encontrado
- **409**: Conflito (ex: SKU ou EAN13 duplicado) - *A ser implementado com banco de dados*

## Próximos passos

- [ ] Integração com banco de dados (PostgreSQL/SQLite)
- [ ] Implementar validação de SKU/EAN13 únicos
- [ ] Adicionar autenticação/autorização
- [ ] Implementar paginação para listagem de produtos
- [ ] Adicionar testes unitários e de integração
- [ ] Implementar logging
- [ ] Adicionar documentação da API (OpenAPI/Swagger)
