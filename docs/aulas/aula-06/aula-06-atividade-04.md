# Atividade 04 - Refatorar Handlers

> **Aula:** 06  
> **Tempo estimado:** 30min

## Objetivo

Refatorar todos os handlers para usar `AppError` em vez de `.unwrap()` e retornos diretos.

## Passos

### 1. Abrir src/handlers/tasks.rs

### 2. Adicionar import no topo

```rust
use crate::error::AppError;  // ← adiciona esta linha
```

### 3. Refatorar get_task

**ANTES:**
```rust
pub async fn get_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> impl IntoResponse {
    let tasks = state.lock().unwrap();  // ❌ unwrap

    let task = match tasks.iter().find(|t| t.id == id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::Error {
            message: "Task não encontrada".into()
        }))
    };

    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}
```

**DEPOIS:**
```rust
pub async fn get_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> Result<Json<Task>, AppError> {  // ✅ Result
    let tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;  // ✅ não usa unwrap

    let task = tasks.iter()
        .find(|t| t.id == id)
        .ok_or(AppError::NotFound)?;  // ✅ retorna erro se não achar

    Ok(Json(task.clone()))
}
```

### 4. Refatorar delete_task

**ANTES:**
```rust
pub async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> (StatusCode, Json<String>) {
    let mut tasks = state.lock().unwrap();  // ❌ unwrap

    let pos = tasks.iter().position(|t| t.id == id);

    match pos {
        Some(index) => {
            let task = tasks.remove(index);
            (StatusCode::OK, Json(format!("Task '{}' deletada!", task.title)))
        },
        None => {
            (StatusCode::NOT_FOUND, Json(String::from("Task não encontrada")))
        }
    }
}
```

**DEPOIS:**
```rust
pub async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> Result<Json<String>, AppError> {  // ✅ Result
    let mut tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;  // ✅ não usa unwrap

    let pos = tasks.iter()
        .position(|t| t.id == id)
        .ok_or(AppError::NotFound)?;  // ✅ retorna erro se não achar

    let task = tasks.remove(pos);
    Ok(Json(format!("Task '{}' deletada!", task.title)))
}
```

### 5. Refatorar create_task

**ANTES:**
```rust
pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::Error {
                message: format_validation_errors(&errors)
            })
        );
    }

    let mut tasks = state.lock().unwrap();  // ❌ unwrap
    // ...
}
```

**DEPOIS:**
```rust
pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>
) -> Result<(StatusCode, Json<Task>), AppError> {  // ✅ Result
    // Validação
    if let Err(errors) = payload.validate() {
        return Err(AppError::ValidationError(
            format_validation_errors(&errors)
        ));
    }

    let mut tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;  // ✅ não usa unwrap

    let id = tasks.len() as u32 + 1;

    let new_task = Task {
        id,
        title: payload.title.clone(),
        done: false,
    };

    tasks.push(new_task.clone());

    Ok((StatusCode::CREATED, Json(new_task)))
}
```

### 6. Refatorar update_task

**ANTES:**
```rust
pub async fn update_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTask>
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::Error {
                message: format_validation_errors(&errors)
            })
        );
    }

    let mut tasks = state.lock().unwrap();  // ❌ unwrap

    let task = match tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::Error {
            message: "Task não encontrada".into()
        })),
    };
    // ...
}
```

**DEPOIS:**
```rust
pub async fn update_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTask>
) -> Result<Json<Task>, AppError> {  // ✅ Result
    // Validação
    if let Err(errors) = payload.validate() {
        return Err(AppError::ValidationError(
            format_validation_errors(&errors)
        ));
    }

    let mut tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;  // ✅ não usa unwrap

    let task = tasks.iter_mut()
        .find(|t| t.id == id)
        .ok_or(AppError::NotFound)?;  // ✅ retorna erro se não achar

    task.done = payload.done;
    task.title = payload.title.clone();

    Ok(Json(task.clone()))
}
```

### 7. Refatorar list_tasks

**ANTES:**
```rust
pub async fn list_tasks(
    State(state): State<SharedState>
) -> (StatusCode, Json<Vec<Task>>) {
    let tasks = state.lock().unwrap();  // ❌ unwrap
    (StatusCode::OK, Json(tasks.clone()))
}
```

**DEPOIS:**
```rust
pub async fn list_tasks(
    State(state): State<SharedState>
) -> Result<Json<Vec<Task>>, AppError> {  // ✅ Result
    let tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;  // ✅ não usa unwrap
    
    Ok(Json(tasks.clone()))
}
```

### 8. Compilar

```bash
cargo build
```

## Resultado Esperado

- Todos handlers usam `Result<T, AppError>`
- Nenhum `.unwrap()` nos handlers
- Compila sem erros
- Erros são tratados de forma profissional

## Testes

```bash
# Compilar
cargo build

# Rodar
cargo run

# Testar endpoints (devem funcionar igual antes)
curl http://127.0.0.1:3003/tasks
```

## Erros Comuns

- **"AppError not found":**
  - Esqueceu `use crate::error::AppError;`

- **"mismatched types":**
  - Tipo de retorno errado
  - Verifique `Result<T, AppError>`

- **"ApiResponse not found":**
  - Pode remover ApiResponse agora
  - Não precisamos mais (AppError substitui)

- **"cannot move out of dereference":**
  - Faltou `.clone()` em algum lugar
  - Verifique `task.clone()`

## Notas

### Mudanças principais:

**1. Tipo de retorno:**
```rust
// Antes:
-> impl IntoResponse

// Depois:
-> Result<T, AppError>
```

**2. .unwrap() → .map_err()?:**
```rust
// Antes:
let tasks = state.lock().unwrap();

// Depois:
let tasks = state.lock()
    .map_err(|_| AppError::MutexError)?;
```

**3. match → .ok_or()?:**
```rust
// Antes:
match tasks.iter().find(...) {
    Some(t) => t,
    None => return error
}

// Depois:
tasks.iter()
    .find(...)
    .ok_or(AppError::NotFound)?
```

**4. Return direto → Err():**
```rust
// Antes:
return (StatusCode::BAD_REQUEST, ...)

// Depois:
return Err(AppError::ValidationError(...))
```

### Operador ?:

```rust
.ok_or(AppError::NotFound)?
//                        ↑ magic!
```

**O que faz:**
- Se `None` → retorna `Err(AppError::NotFound)`
- Se `Some(x)` → desembrulha e continua

**Equivalente a:**
```rust
match value {
    Some(x) => x,
    None => return Err(AppError::NotFound),
}
```

**Muito mais limpo!** ✅

### Por que remover ApiResponse?

**Antes (Aula 05):**
```rust
enum ApiResponse {
    Success(Task),
    Error { message: String },
}
```

**Agora (Aula 06):**
- Sucesso: `Ok(Json(task))`
- Erro: `Err(AppError::...)`

**Result<T, E> substituiu ApiResponse.** Mais idiomático.

### Próxima atividade:

Explorar operador `?` em mais detalhes.