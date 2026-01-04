# Aula 05 - Validação

> **Data:** 2026-01-05  
> **Duração:** 1h-1.5h  
> **Dificuldade:** Fácil/Média

## Atividades

- [Atividade 1](./aula-05-atividade-01.md) - Instalar Validator
- [Atividade 2](./aula-05-atividade-02.md) - Validar CreateTask
- [Atividade 3](./aula-05-atividade-03.md) - Aplicar Validação no Handler
- [Atividade 4](./aula-05-atividade-04.md) - Formatar Erros
- [Atividade 5](./aula-05-atividade-05.md) - Validar UpdateTask
- [Atividade 6](./aula-05-atividade-06.md) - Testar Validação

## Objetivos

Adicionar validação profissional nas suas structs.

**Antes:**
```rust
struct CreateTask {
    title: String,  // aceita string vazia ❌
}
```

**Depois:**
```rust
#[derive(Validate)]
struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    title: String,  // valida 1-100 chars ✅
}
```

## Conceitos Novos

- Validator crate (validação declarativa)
- Derive macro Validate
- Validation errors
- Error handling idiomático
- Helper functions para formatação

## Validações Disponíveis

### Strings
- `length(min = X, max = Y)` - tamanho
- `email` - email válido
- `url` - URL válida
- `regex(path = "PATTERN")` - regex custom
- `contains = "texto"` - contém substring

### Números
- `range(min = X, max = Y)` - range numérico
- `custom(function = "fn_name")` - função custom

### Múltiplas
```rust
#[validate(length(min = 1), email)]  // título E email
```

## Comparação com Laravel

**Laravel:**
```php
$request->validate([
    'title' => 'required|min:1|max:100',
    'email' => 'required|email',
]);
```

**Rust (validator):**
```rust
#[derive(Validate)]
struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    title: String,
    
    #[validate(email)]
    email: String,
}
```

**Diferenças:**
- Laravel: validação em runtime (controller)
- Rust: validação declarada na struct
- Laravel: string-based rules
- Rust: type-safe annotations

## O Que Aprendi

- Validação declarativa com derive macros
- Error handling com ValidationErrors
- Formatação de erros para usuário
- Status codes corretos (400 Bad Request)

## Dúvidas Pendentes

- Como criar validadores customizados?
- Como validar structs aninhadas?
- Performance de validação vs validação manual?

## Notas

- `validator` é padrão da indústria
- Mensagens de erro podem ser customizadas
- Validação acontece antes de salvar dados
- Similar a Form Requests do Laravel