# Rust - Aula 04 - Módulos

Nessa aula, vamos separar seu `main.rs` gigante em módulos organizados. Vamos continuar o código da [aula 03](../aula-03/README.md), e melhorar nossa API. Na aula 03 nosso `main.rs` contém toda a lógica de negócio, agora nós vamos aprender separar da melhor forma possível.

## Atividades

- [Atividade 1](./aula-04-atividade-01.md)
- [Atividade 2](./aula-04-atividade-02.md)
- [Atividade 3](./aula-04-atividade-03.md)
- [Atividade 4](./aula-04-atividade-04.md)
- [Atividade 5](./aula-04-atividade-05.md)
- [Atividade 6](./aula-04-atividade-06.md)
- [Atividade 7](./aula-04-atividade-07.md)

## Estrutura

**Antes**, na aula anterior:

```shellscript
src/
└── main.rs (300+ linhas - TUDO junto)
```

**Depois** desta aula:

```shellscript
src/
├── main.rs (30 linhas - só init)
├── models/
│   ├── mod.rs
│   └── task.rs
├── handlers/
│   ├── mod.rs
│   └── tasks.rs
└── routes/
    ├── mod.rs
    └── tasks.rs
```

## Conceitos Novos

* **Sistema de módulos** (`mod`, `pub`, `use`)
* **mod.rs** (arquivo especial de módulo)
* **Visibilidade** (public vs private)
* **Importação** (crate, super, self)

## Atividades

7 Atividades

## Conceitos Aprendidos

* Módulos (`mod`, `pub`)
* `mod.rs` (expor submódulos)
* Visibilidade (`pub` vs `private`)
* Importação (use `crate::`)
* Separação de responsabilidades
* Estrutura profissional

## Desafios Esperados

1. **Erro "cannot find...":**
   1. Esqueceu pub em struct/função
   2. Esqueceu mod no mod.rs
   3. Import path errado
   **Solução:** Leia erro com atenção, ele diz exatamente o que falta.
2. **"circular dependency":**
   1. Módulo A importa B, B importa A
   2. Evite imports circulares
3. **"private struct...":**
   1. Esqueceu pub struct
   2. Adiciona pub em tudo que precisa ser acessado

## Quando Completar

Você vai ter:

* Código organizado profissionalmente
* main.rs limpo (30 linhas)
* Módulos separados por responsabilidade
* **Estrutura escalável** (adicionar novos módulos é fácil)
