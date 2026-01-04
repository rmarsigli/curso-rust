# Atividade 01 - Instalar thiserror

> **Aula:** 06  
> **Tempo estimado:** 5min

## Objetivo

Instalar `thiserror` crate para criar erros customizados com derive macros.

## Passos

### 1. Adicionar dependência

```bash
cargo add thiserror
```

### 2. OU editar Cargo.toml manualmente

```toml
[dependencies]
thiserror = "1.0"
# ... resto das dependências
```

### 3. Testar compilação

```bash
cargo build
```

## Resultado Esperado

- `thiserror` instalado
- Projeto compila sem erros
- Pronto para criar enum de erros

## Testes

```bash
cargo build  # deve compilar normalmente
```

## Erros Comuns

- **Versão não encontrada:**
  ```bash
  cargo add thiserror@latest
  ```

- **Network error:**
  - Edite `Cargo.toml` manualmente
  - Ou verifique conexão

## Notas

### O que é thiserror?

Crate que facilita criar erros customizados com derive macros.

**Sem thiserror:**
```rust
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            // ... 20 linhas de boilerplate
        }
    }
}

impl std::error::Error for AppError {}
```

**Com thiserror:**
```rust
#[derive(thiserror::Error)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
}
```

**2 linhas vs 20+.** Muito melhor.

### Alternativas:

- **anyhow:** Mais simples, menos type-safe
  - Bom para aplicações (CLI, scripts)
  - Não recomendado para bibliotecas

- **thiserror:** Mais verboso, type-safe
  - Bom para bibliotecas e APIs
  - **Recomendado para este projeto**

### Por que thiserror?

- ✅ Type-safe (cada erro é tipo diferente)
- ✅ Derive macros (menos código)
- ✅ Padrão da indústria
- ✅ Trabalha bem com `?` operator
- ✅ Bom para APIs REST

### Quando usar anyhow?

- CLI tools
- Scripts
- Protótipos rápidos
- Quando não precisa type-safety

**Para API REST: use thiserror.** ✅