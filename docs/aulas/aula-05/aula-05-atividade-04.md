# Atividade 04 - Formatar Erros

> **Aula:** 05  
> **Tempo estimado:** 15min

## Objetivo

Criar função helper para formatar erros de validação de forma legível para o usuário.

## Passos

### 1. Abrir src/handlers/tasks.rs

### 2. Adicionar import no topo

```rust
use validator::ValidationErrors;  // ← adiciona esta linha
```

### 3. Adicionar função helper no final do arquivo

```rust
fn format_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();
            format!("{}: {}", field, messages.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ")
}
```

### 4. Atualizar função create_task

**ANTES:**
```rust
if let Err(errors) = payload.validate() {
    return (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::Error {
            message: format!("Validação falhou: {:?}", errors)
        })
    );
}
```

**DEPOIS:**
```rust
if let Err(errors) = payload.validate() {
    return (
        StatusCode::BAD_REQUEST,
        Json(ApiResponse::Error {
            message: format_validation_errors(&errors)
        })
    );
}
```

### 5. Testar

```bash
cargo run
```

## Resultado Esperado

Mensagens de erro mais legíveis:

**Antes:**
```json
{"message":"Validação falhou: ValidationErrors { ... }"}
```

**Depois:**
```json
{"message":"title: Título deve ter entre 1 e 100 caracteres"}
```

## Testes

### Teste 1: Título vazio

```bash
curl -i -X POST http://127.0.0.1:3003/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":""}'
```

**Resultado esperado:**
```
HTTP/1.1 400 Bad Request

{"message":"title: Título deve ter entre 1 e 100 caracteres"}
```

### Teste 2: Múltiplos erros (se tiver mais campos)

Se adicionar validação de email:
```json
{"message":"title: Título deve ter entre 1 e 100 caracteres; email: Email inválido"}
```

## Erros Comuns

- **`ValidationErrors` not found:**
  - Esqueceu: `use validator::ValidationErrors;`

- **"no method `filter_map` found":**
  - Erro de sintaxe na função
  - Copie exatamente como está no código

- **Mensagem ainda aparece como debug:**
  - Não chamou `format_validation_errors(&errors)`
  - Está usando `format!("{:?}", errors)` ainda

## Notas

### Como a função funciona:

1. `errors.field_errors()` → pega erros por campo
2. `.iter()` → itera sobre campos com erro
3. `.map(|(field, errors)| ...)` → formata cada campo
4. `filter_map(|e| e.message.as_ref()...)` → extrai mensagens
5. `.join(", ")` → junta mensagens do mesmo campo
6. `.join("; ")` → junta mensagens de campos diferentes

### Exemplo passo a passo:

**Input:**
```rust
ValidationErrors {
    title: ["Título deve ter entre 1 e 100 caracteres"],
    email: ["Email inválido"]
}
```

**Output:**
```
"title: Título deve ter entre 1 e 100 caracteres; email: Email inválido"
```

### Alternativa simples (se preferir):

```rust
fn format_validation_errors(errors: &ValidationErrors) -> String {
    format!("Erro de validação: {:?}", errors)
}
```

Menos bonito, mas funciona. Use a versão completa para produção.

### Comparação Laravel:

**Laravel:**
```php
// Formata automático:
[
    "title" => ["The title must be between 1 and 100 characters."]
]
```

**Rust:**
```rust
// Precisa formatar manualmente
// Mas tem controle total do formato
```