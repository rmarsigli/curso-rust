# Atividade 05 - Validar UpdateTask

> **Aula:** 05  
> **Tempo estimado:** 10min

## Objetivo

Adicionar validação no struct `UpdateTask` (mesma validação do `CreateTask`).

## Passos

### 1. Abrir src/models/task.rs

### 2. Atualizar struct UpdateTask

**ANTES:**
```rust
#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: String,
    pub done: bool,
}
```

**DEPOIS:**
```rust
#[derive(Deserialize, Validate)]  // ← adiciona Validate
pub struct UpdateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub done: bool,  // ← não precisa validação (bool sempre válido)
}
```

### 3. Abrir src/handlers/tasks.rs

### 4. Atualizar função update_task

Adiciona validação no início da função:

```rust
pub async fn update_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTask>
) -> impl IntoResponse {
    // VALIDA
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::Error {
                message: format_validation_errors(&errors)
            })
        );
    }

    let mut tasks = state.lock().unwrap();

    let task = match tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::Error {
            message: "Task não encontrada".into()
        })),
    };

    task.done = payload.done;
    task.title = payload.title.clone();

    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}
```

### 5. Testar

```bash
cargo run
```

## Resultado Esperado

- PATCH com título vazio retorna 400
- PATCH com título >100 chars retorna 400
- PATCH com título válido funciona
- Campo `done` não é validado (bool sempre válido)

## Testes

### Teste 1: Criar task primeiro

```bash
curl -X POST http://127.0.0.1:3003/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Task original"}'
```

### Teste 2: Update com título vazio (deve falhar)

```bash
curl -i -X PATCH http://127.0.0.1:3003/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"","done":true}'
```

**Resultado esperado:**
```
HTTP/1.1 400 Bad Request

{"message":"title: Título deve ter entre 1 e 100 caracteres"}
```

### Teste 3: Update válido (deve funcionar)

```bash
curl -i -X PATCH http://127.0.0.1:3003/tasks/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Task atualizada","done":true}'
```

**Resultado esperado:**
```
HTTP/1.1 200 OK

{"id":1,"title":"Task atualizada","done":true}
```

## Erros Comuns

- **Validação não funciona:**
  - Esqueceu `#[derive(Validate)]` no struct
  - Esqueceu `payload.validate()` no handler

- **Erro de import:**
  - `Validate` já deve estar importado da atividade anterior
  - Caso não esteja: `use validator::Validate;`

- **ValidationErrors not found:**
  - Já deve estar importado da atividade 04

## Notas

### Por que `done` não precisa validação?

```rust
pub done: bool,  // só aceita true ou false
```

- `bool` só pode ser `true` ou `false`
- JSON inválido (`"done": "sim"`) já é rejeitado por Serde
- Não precisa de `#[validate(...)]`

### Se quisesse validar `done`:

```rust
#[validate(custom(function = "validate_done"))]
pub done: bool,

fn validate_done(value: &bool) -> Result<(), ValidationError> {
    if !value {
        return Err(ValidationError::new("must_be_true"));
    }
    Ok(())
}
```

Mas isso força `done` sempre ser `true`, não faz sentido.

### Validações diferentes por struct:

```rust
// CreateTask: done não existe (sempre false ao criar)
#[derive(Validate)]
struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    title: String,
}

// UpdateTask: done existe e pode ser alterado
#[derive(Validate)]
struct UpdateTask {
    #[validate(length(min = 1, max = 100))]
    title: String,
    done: bool,  // sem validação
}
```

Isso é correto e idiomático.