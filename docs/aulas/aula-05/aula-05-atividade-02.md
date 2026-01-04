# Atividade 02 - Validar CreateTask

> **Aula:** 05  
> **Tempo estimado:** 10min

## Objetivo

Adicionar validação no struct `CreateTask` para garantir que título tem entre 1 e 100 caracteres.

## Passos

### 1. Abrir src/models/task.rs

### 2. Adicionar import no topo

```rust
use validator::Validate;  // ← adiciona esta linha
```

### 3. Atualizar struct CreateTask

**ANTES:**
```rust
#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}
```

**DEPOIS:**
```rust
#[derive(Deserialize, Validate)]  // ← adiciona Validate
pub struct CreateTask {
    #[validate(length(min = 1, max = 100, message = "Título deve ter entre 1 e 100 caracteres"))]
    pub title: String,
}
```

### 4. Compilar para testar

```bash
cargo build
```

## Resultado Esperado

- Struct `CreateTask` tem validação configurada
- Código compila sem erros
- Validação ainda NÃO é executada (próxima atividade)

## Testes

```bash
# Deve compilar normalmente
cargo build

# Pode ter warning "Validate never constructed" - é normal
# A validação será usada na próxima atividade
```

## Erros Comuns

- **`Validate` not found:**
  - Esqueceu de adicionar `use validator::Validate;`
  - Ou `validator` não foi instalado corretamente

- **Syntax error em `#[validate(...)]`:**
  - Verifique vírgulas e parênteses
  - Formato correto: `#[validate(length(min = 1, max = 100))]`

- **"expected `,` found `message`":**
  - Está faltando vírgula entre parâmetros
  - Correto: `min = 1, max = 100, message = "..."`

## Notas

### O que cada parte faz:

- `#[derive(Validate)]` → habilita validação nesta struct
- `#[validate(...)]` → regras específicas do campo
- `length(min = 1, max = 100)` → tamanho entre 1 e 100 chars
- `message = "..."` → mensagem customizada de erro (opcional)

### Comparação:

**PHP/Laravel:**
```php
'title' => 'required|min:1|max:100'
```

**Rust/Validator:**
```rust
#[validate(length(min = 1, max = 100))]
```

### Importante:

- Validação é **declarativa** (annotation na struct)
- Validação ainda **não é executada** - só declarada
- Próxima atividade: executar validação no handler