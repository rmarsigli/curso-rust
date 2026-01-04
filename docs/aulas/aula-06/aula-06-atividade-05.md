# Atividade 05 - Operador ? (Question Mark)

> **Aula:** 06  
> **Tempo estimado:** 20min

## Objetivo

Entender profundamente como funciona o operador `?` e quando usá-lo.

## O que é o operador ?

**Sintaxe:**
```rust
let value = something_that_returns_result()?;
//                                        ↑ magic!
```

**O que faz:**
1. Se `Result` for `Ok(x)` → desembrulha e retorna `x`
2. Se `Result` for `Err(e)` → retorna erro imediatamente da função

**É açúcar sintático para:**
```rust
let value = match something_that_returns_result() {
    Ok(x) => x,
    Err(e) => return Err(e),
};
```

## Exemplos

### Exemplo 1: Básico

**Sem ?:**
```rust
fn get_task(id: u32) -> Result<Task, AppError> {
    let tasks = match state.lock() {
        Ok(t) => t,
        Err(_) => return Err(AppError::MutexError),
    };
    
    let task = match tasks.iter().find(|t| t.id == id) {
        Some(t) => t,
        None => return Err(AppError::NotFound),
    };
    
    Ok(task.clone())
}
```

**Com ?:**
```rust
fn get_task(id: u32) -> Result<Task, AppError> {
    let tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;
    
    let task = tasks.iter()
        .find(|t| t.id == id)
        .ok_or(AppError::NotFound)?;
    
    Ok(task.clone())
}
```

**11 linhas → 7 linhas. Mais limpo.** ✅

### Exemplo 2: Múltiplos erros

```rust
async fn complex_operation() -> Result<String, AppError> {
    let tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;  // ← pode falhar aqui
    
    let task = tasks.iter()
        .find(|t| t.id == 1)
        .ok_or(AppError::NotFound)?;  // ← ou aqui
    
    validate_task(&task)?;  // ← ou aqui
    
    Ok(task.title.clone())  // ← ou sucesso
}
```

**Se qualquer `?` falhar, retorna erro imediatamente.**

### Exemplo 3: Encadeamento

```rust
async fn chain_example() -> Result<Task, AppError> {
    // Cada ? pode falhar:
    let tasks = state.lock().map_err(|_| AppError::MutexError)?;
    let task = tasks.get(0).ok_or(AppError::NotFound)?;
    let validated = validate(task)?;
    let processed = process(validated)?;
    
    Ok(processed)
}
```

**Leia de cima pra baixo. Se algo falhar, para.**

## .map_err() - Converter Erros

### Por que precisa?

```rust
state.lock()  // retorna Result<T, PoisonError>
```

Mas nossa função retorna `Result<T, AppError>`.

**Tipos incompatíveis!** ❌

### Solução: .map_err()

```rust
state.lock()
    .map_err(|_| AppError::MutexError)?
//  ^^^^^^^^ converte PoisonError → AppError
```

**Agora é compatível.** ✅

### Template:

```rust
operacao_externa()
    .map_err(|erro_externo| AppError::MeuErro)?
//  ^^^^^^^^ transforma tipo de erro
```

## .ok_or() - Option → Result

### Problema:

```rust
tasks.iter().find(|t| t.id == id)  // retorna Option<&Task>
```

Mas precisamos de `Result<&Task, AppError>`.

### Solução: .ok_or()

```rust
tasks.iter()
    .find(|t| t.id == id)
    .ok_or(AppError::NotFound)?
//  ^^^^^ converte None → Err(AppError::NotFound)
```

**Option → Result → pode usar ?** ✅

### Como funciona:

```rust
// Se Some(x):
Some(task).ok_or(AppError::NotFound)  // → Ok(task)

// Se None:
None.ok_or(AppError::NotFound)  // → Err(AppError::NotFound)
```

## Quando NÃO usar ?

### 1. Quando precisa tratar erro localmente

**Errado:**
```rust
let task = tasks.find(...)?;  // ← retorna erro da função inteira
println!("Não encontrou, mas continuo...");  // nunca executa
```

**Certo:**
```rust
let task = match tasks.find(...) {
    Some(t) => t,
    None => {
        println!("Não encontrou, usando default");
        &default_task  // continua executando
    }
};
```

### 2. Quando quer logar erro antes de propagar

**Errado:**
```rust
let tasks = state.lock()?;  // ← retorna sem logar
```

**Certo:**
```rust
let tasks = state.lock()
    .map_err(|e| {
        eprintln!("Erro ao travar mutex: {:?}", e);  // loga
        AppError::MutexError  // retorna erro
    })?;
```

### 3. Em funções que não retornam Result

```rust
fn main() {  // ← não retorna Result
    let x = something()?;  // ❌ ERRO! main não retorna Result
}
```

**Correção:**
```rust
#[tokio::main]
async fn main() -> Result<(), AppError> {  // ← agora retorna Result
    let x = something()?;  // ✅ OK
    Ok(())
}
```

## Exercício Prático

### Refatore este código:

**Antes:**
```rust
async fn create_and_get(title: String) -> Result<Task, AppError> {
    let mut tasks = match state.lock() {
        Ok(t) => t,
        Err(_) => return Err(AppError::MutexError),
    };
    
    if title.is_empty() {
        return Err(AppError::ValidationError("Título vazio".into()));
    }
    
    let id = tasks.len() as u32 + 1;
    
    let task = Task { id, title, done: false };
    tasks.push(task.clone());
    
    Ok(task)
}
```

**Depois (com ?):**
```rust
async fn create_and_get(title: String) -> Result<Task, AppError> {
    let mut tasks = state.lock()
        .map_err(|_| AppError::MutexError)?;
    
    if title.is_empty() {
        return Err(AppError::ValidationError("Título vazio".into()));
    }
    
    let id = tasks.len() as u32 + 1;
    let task = Task { id, title, done: false };
    tasks.push(task.clone());
    
    Ok(task)
}
```

**Menos 4 linhas. Mais limpo.** ✅

## Notas

### Comparação com outras linguagens:

**JavaScript (async/await):**
```javascript
const task = await getTask();  // throw se falhar
```

**PHP:**
```php
$task = getTask();  // throw exception se falhar
```

**Rust:**
```rust
let task = get_task()?;  // Err se falhar (compile-time safe)
```

**Rust é explícito, mas seguro.** ✅

### Regra de ouro:

**Use ? quando:**
- ✅ Função retorna `Result<T, E>`
- ✅ Quer propagar erro pra cima
- ✅ Não precisa tratar erro localmente

**Use match quando:**
- ✅ Precisa tratar erro específico
- ✅ Quer logar antes de retornar
- ✅ Quer valor default se falhar

### Próxima atividade:

Testar toda a API com error handling profissional.