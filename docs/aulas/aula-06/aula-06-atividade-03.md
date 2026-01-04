# Atividade 03 - Implementar IntoResponse

> **Aula:** 06  
> **Tempo estimado:** 20min

## Objetivo

Implementar `IntoResponse` para `AppError` para converter erros em HTTP responses automaticamente.

## Passos

### 1. Abrir src/error.rs

### 2. Adicionar imports

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
```

### 3. Implementar IntoResponse trait

Adicione no final do arquivo `src/error.rs`:

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                self.to_string()
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                msg
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string()
            ),
            AppError::MutexError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao acessar dados".to_string()
            ),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}
```

### 4. Arquivo completo src/error.rs

Deve ficar assim:

```rust
use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Task não encontrada")]
    NotFound,
    
    #[error("Erro de validação: {0}")]
    ValidationError(String),
    
    #[error("Erro interno do servidor")]
    InternalError,
    
    #[error("Erro ao travar mutex")]
    MutexError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                self.to_string()
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                msg
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string()
            ),
            AppError::MutexError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erro ao acessar dados".to_string()
            ),
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}
```

### 5. Compilar

```bash
cargo build
```

## Resultado Esperado

- `IntoResponse` implementado para `AppError`
- Compila sem erros
- `AppError` agora pode ser retornado em handlers
- Conversão automática para HTTP response

## Testes

```bash
cargo build  # deve compilar

# Ainda vai ter warnings "unused"
# Vamos usar nas próximas atividades
```

## Erros Comuns

- **"IntoResponse not found":**
  - Esqueceu imports do Axum

- **"json! not found":**
  - Esqueceu `use serde_json::json;`

- **"trait bounds not satisfied":**
  - Erro de sintaxe no `impl IntoResponse`
  - Verifique match arms

## Notas

### Como funciona:

**1. Match no erro:**
```rust
match self {
    AppError::NotFound => (StatusCode::NOT_FOUND, "mensagem"),
    // ...
}
```

**2. Retorna tupla (status, mensagem):**
```rust
let (status, message) = match self { ... };
```

**3. Cria JSON response:**
```rust
let body = Json(json!({
    "error": message
}));
```

**4. Converte para Response:**
```rust
(status, body).into_response()
```

### Mapeamento de erros:

| AppError | HTTP Status | Mensagem |
|----------|-------------|----------|
| NotFound | 404 | "Task não encontrada" |
| ValidationError(msg) | 400 | msg customizada |
| InternalError | 500 | "Erro interno do servidor" |
| MutexError | 500 | "Erro ao acessar dados" |

### Por que não expor "MutexError"?

```rust
AppError::MutexError => (
    StatusCode::INTERNAL_SERVER_ERROR,
    "Erro ao acessar dados".to_string()  // ← genérico
),
```

**Segurança:**
- Não expõe detalhes internos
- Usuário vê mensagem genérica
- Logs devem ter detalhes (implementar depois)

### Exemplo de uso:

```rust
async fn get_task(...) -> Result<impl IntoResponse, AppError> {
    let task = tasks.iter()
        .find(|t| t.id == id)
        .ok_or(AppError::NotFound)?;  // ← retorna 404 automático
        
    Ok(Json(task))
}
```

**Se task não existe:**
```
HTTP/1.1 404 Not Found
{"error":"Task não encontrada"}
```

### Comparação Laravel:

**Laravel:**
```php
abort(404, 'Task não encontrada');
// converte automático em JSON
```

**Rust:**
```rust
Err(AppError::NotFound)?
// IntoResponse converte automático
```

**Similar, mas Rust é type-safe.** ✅

### Próxima atividade:

Refatorar handlers para usar `AppError` em vez de `.unwrap()`.