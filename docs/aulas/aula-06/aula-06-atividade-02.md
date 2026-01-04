# Atividade 02 - Criar AppError Enum

> **Aula:** 06  
> **Tempo estimado:** 15min

## Objetivo

Criar enum `AppError` com todos os tipos de erro da API.

## Passos

### 1. Criar arquivo src/error.rs

```bash
touch src/error.rs
```

### 2. Adicionar conteúdo no arquivo

```rust
use thiserror::Error;

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
```

### 3. Adicionar módulo no src/main.rs

No topo do `main.rs`, adicione:

```rust
mod error;  // ← adiciona esta linha

mod models;
mod handlers;
mod routes;
// ... resto do código
```

### 4. Compilar

```bash
cargo build
```

## Resultado Esperado

- Arquivo `src/error.rs` criado
- Enum `AppError` com 4 variantes
- Compila sem erros
- Módulo disponível para uso

## Testes

```bash
cargo build  # deve compilar

# Pode ter warning "unused" - é normal
# Vamos usar nas próximas atividades
```

## Erros Comuns

- **"module `error` not found":**
  - Esqueceu `mod error;` no main.rs

- **"Error trait not found":**
  - Esqueceu `use thiserror::Error;`

- **Syntax error em `#[error(...)]`:**
  - Verifique aspas e parênteses

## Notas

### Explicação de cada variante:

**1. NotFound:**
```rust
#[error("Task não encontrada")]
NotFound,
```
- Usada quando task com ID não existe
- Vai virar HTTP 404

**2. ValidationError:**
```rust
#[error("Erro de validação: {0}")]
ValidationError(String),
```
- `{0}` = placeholder para mensagem
- `String` = armazena mensagem de erro
- Exemplo: `AppError::ValidationError("Título vazio".into())`
- Vai virar HTTP 400

**3. InternalError:**
```rust
#[error("Erro interno do servidor")]
InternalError,
```
- Erro genérico do servidor
- Vai virar HTTP 500

**4. MutexError:**
```rust
#[error("Erro ao travar mutex")]
MutexError,
```
- Quando `.lock()` falha
- Vai virar HTTP 500

### Como usar:

```rust
// Retornar erro:
return Err(AppError::NotFound);

// Com mensagem:
return Err(AppError::ValidationError("Título vazio".into()));
```

### Comparação PHP/Laravel:

**Laravel:**
```php
throw new NotFoundHttpException('Task não encontrada');
throw new ValidationException('Título vazio');
```

**Rust:**
```rust
Err(AppError::NotFound)
Err(AppError::ValidationError("Título vazio".into()))
```

**Diferença:**
- PHP: exceptions (throw/catch)
- Rust: Result<T, E> (type-safe)

### Por que `Debug` e `Error` traits?

```rust
#[derive(Debug, Error)]
```

- **Debug:** permite imprimir erro com `{:?}`
- **Error:** marca como tipo de erro (padrão Rust)
- **thiserror::Error:** gera implementação automática

### Próxima atividade:

Converter `AppError` em HTTP response (IntoResponse trait).