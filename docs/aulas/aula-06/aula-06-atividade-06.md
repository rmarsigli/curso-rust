# Aula 06 - Error Handling Profissional

> **Data:** 2026-01-06  
> **Duração:** 1.5h-2h  
> **Dificuldade:** Média/Difícil

## Atividades

- [Atividade 1](./aula-06-atividade-01.md) - Instalar thiserror
- [Atividade 2](./aula-06-atividade-02.md) - Criar AppError Enum
- [Atividade 3](./aula-06-atividade-03.md) - Implementar IntoResponse
- [Atividade 4](./aula-06-atividade-04.md) - Refatorar Handlers
- [Atividade 5](./aula-06-atividade-05.md) - Usar Operador ?
- [Atividade 6](./aula-06-atividade-06.md) - Testar Error Handling

## Objetivos

Trocar `.unwrap()` por error handling profissional e idiomático.

**Antes:**
```rust
let mut tasks = state.lock().unwrap();  // ❌ panic em produção
```

**Depois:**
```rust
let mut tasks = state.lock()
    .map_err(|_| AppError::InternalError)?;  // ✅ retorna erro HTTP
```

## Conceitos Novos

- `thiserror` crate (derive macros para erros)
- Enum de erros customizado (`AppError`)
- `IntoResponse` trait (converter erro em HTTP response)
- Operador `?` (propagar erros)
- `Result<T, AppError>` como tipo de retorno
- Error handling sem `.unwrap()`

## Problema do .unwrap()

### Código atual (Aula 05):

```rust
async fn create_task(...) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();  // ❌ PANIC se falhar
    // ...
}
```

### O que acontece se falhar:

```
thread 'tokio-runtime-worker' panicked at src/handlers/tasks.rs:15:45
```

**Servidor CRASHA. API fica fora do ar.** 🔥

### Solução profissional:

```rust
async fn create_task(...) -> Result<impl IntoResponse, AppError> {
    let mut tasks = state.lock()
        .map_err(|_| AppError::InternalError)?;  // ✅ retorna 500
    // ...
}
```

**Servidor NÃO crasha. Retorna erro HTTP 500.** ✅

## Tipos de Erro

### AppError enum (vamos criar):

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Task não encontrada")]
    NotFound,
    
    #[error("Erro de validação: {0}")]
    ValidationError(String),
    
    #[error("Erro interno do servidor")]
    InternalError,
}
```

### Conversão automática para HTTP:

- `AppError::NotFound` → 404 Not Found
- `AppError::ValidationError` → 400 Bad Request
- `AppError::InternalError` → 500 Internal Server Error

## Comparação com Laravel

**Laravel:**
```php
// Laravel faz automático
$task = Task::findOrFail($id);  // throw 404 se não achar

// Ou manual:
if (!$task) {
    abort(404, 'Task não encontrada');
}
```

**Rust (depois desta aula):**
```rust
// Rust precisa ser explícito
let task = tasks.iter()
    .find(|t| t.id == id)
    .ok_or(AppError::NotFound)?;
```

**Diferenças:**
- Laravel: exceptions (runtime)
- Rust: Result<T, E> (compile-time)
- Laravel: implícito e mágico
- Rust: explícito e type-safe

## O Que Aprendi

- Error handling sem panic
- thiserror para derive macros
- Enum de erros customizado
- IntoResponse para converter erros
- Operador ? para propagar
- Result<T, E> como retorno

## Dúvidas Pendentes

- Como criar erros mais complexos?
- Como logar erros sem expor detalhes?
- Como combinar diferentes tipos de erro?
- Performance de error handling vs .unwrap()?

## Notas

- **NUNCA use .unwrap() em produção**
- `thiserror` é padrão da indústria
- `anyhow` é alternativa (mais simples, menos type-safe)
- Error handling em Rust é MUITO melhor que try/catch
- Compilador força você a lidar com erros