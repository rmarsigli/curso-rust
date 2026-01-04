# Aula 05 - Validação

> **Data:** 2026-01-04  
> **Duração:** 1h-2h 
> **Dificuldade:** Média

## Atividades

- [Atividade 1](./aula-05-atividade-01.md)
- [Atividade 2](./aula-05-atividade-02.md)
- [Atividade 3](./aula-05-atividade-03.md)
- [Atividade 4](./aula-05-atividade-04.md)
- [Atividade 5](./aula-05-atividade-05.md)
- [Atividade 6](./aula-05-atividade-06.md)

## Objetivos

Adicionar validação profissional nas suas structs.

Antes:
```rust
struct CreateTask {
    title: String,  // aceita string vazia
}
```

Depois:
```rust
#[derive(Validate)]
struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    title: String,  // valida 1-100 chars
}
```

## Conceitos Novos

- Validator crate (validação declarativa)
- Derive macro Validate
- Validation errors
- Error handling idiomático
- Custom validators (se precisar)

## Código

```rust
// Em breve...
```

## O Que Aprendi

- Aprendizado 1
- Aprendizado 2

## Dúvidas Pendentes

- Dúvida 1

## Notas

Observações gerais