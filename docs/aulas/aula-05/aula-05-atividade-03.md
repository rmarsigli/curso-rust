# Atividade 03 - Aplicar Validação no Handler

> **Aula:** 05  
> **Tempo estimado:** 15min

## Objetivo

Executar validação no handler `create_task` antes de criar a task.

## Passos

### 1. Abrir src/handlers/tasks.rs

### 2. Adicionar import no topo

```rust
use validator::Validate;  // ← adiciona esta linha
```

### 3. Atualizar função create_task

**ANTES:**
```rust
pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>
) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();
    let id = tasks.len() as u32 + 1;
    // ... resto do código
}
```

**DEPOIS:**
```rust
pub async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>
) -> impl IntoResponse {
    // VALIDA ANTES DE USAR
    if let Err(errors) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::Error {
                message: format!("Validação falhou: {:?}", errors)
            })
        );
    }

    let mut tasks = state.lock().unwrap();
    let id = tasks.len() as u32 + 1;

    let new_task = Task {
        id,
        title: payload.title.clone(),
        done: false,
    };

    tasks.push(new_task.clone());

    (StatusCode::CREATED, Json(ApiResponse::Success(new_task.clone())))
}
```

### 4. Testar

```bash
cargo run
```

## Resultado Esperado

- Títulos vazios retornam erro 400
- Títulos com >100 chars retornam erro 400
- Títulos válidos (1-100 chars) funcionam normalmente

## Testes

### Teste 1: Título vazio (deve falhar)

```bash
curl -i -X POST http://127.0.0.1:3003/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":""}'
```

**Resultado esperado:**
```
HTTP/1.1 400 Bad Request

{"message":"Validação falhou: ..."}
```

### Teste 2: Título muito longo (deve falhar)

```bash
# Gera string com 101 caracteres
curl -i -X POST http://127.0.0.1:3003/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}'
```

**Resultado esperado:**
```
HTTP/1.1 400 Bad Request

{"message":"Validação falhou: ..."}
```

### Teste 3: Título válido (deve funcionar)

```bash
curl -i -X POST http://127.0.0.1:3003/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Task válida"}'
```

**Resultado esperado:**
```
HTTP/1.1 201 Created

{"id":1,"title":"Task válida","done":false}
```

## Erros Comuns

- **`Validate` not found:**
  - Esqueceu import: `use validator::Validate;`

- **"ApiResponse not found":**
  - Já deveria estar importado da aula anterior
  - Verifique imports no topo do arquivo

- **Erro 500 em vez de 400:**
  - Validação pode estar falhando com panic
  - Verifique se `payload.validate()` está correto

## Notas

### O que acontece:

1. `payload.validate()` retorna `Result<(), ValidationErrors>`
2. Se houver erro (`Err`):
   - Retorna 400 Bad Request
   - Com mensagem de erro
   - NÃO cria a task
3. Se OK (`Ok`):
   - Continua normalmente
   - Cria a task

### Comparação Laravel:

**Laravel:**
```php
$validated = $request->validate([
    'title' => 'required|min:1|max:100',
]);
// Se falhar, retorna 422 automático
```

**Rust:**
```rust
if let Err(errors) = payload.validate() {
    return (StatusCode::BAD_REQUEST, ...);
}
// Explícito, mas type-safe
```

### Próxima atividade:

Melhorar formatação das mensagens de erro (está saindo como debug `{:?}`)