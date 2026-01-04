# Atividade 01 - Instalar Validator

> **Aula:** 05  
> **Tempo estimado:** 5min

## Objetivo

Adicionar a crate `validator` ao projeto para validação de structs.

## Passos

### 1. Adicionar dependência via terminal

```bash
cargo add validator --features derive
cargo add serde_json  # para formatar erros bonitos
```

### 2. OU editar Cargo.toml manualmente

Abra `Cargo.toml` e adicione:

```toml
[dependencies]
validator = { version = "0.18", features = ["derive"] }
serde_json = "1.0"
# ... resto das dependências existentes
```

### 3. Testar compilação

```bash
cargo build
```

## Resultado Esperado

- `validator` instalado com sucesso
- `serde_json` instalado
- Projeto compila sem erros
- Pronto para adicionar validações

## Testes

```bash
# Verificar se instalou
cargo build

# Deve compilar sem erros
# Pode ter warnings de "unused", é normal
```

## Erros Comuns

- **Erro de versão:** Se `0.18` não funcionar, use versão mais recente:
  ```bash
  cargo add validator@latest --features derive
  ```

- **Feature não encontrada:** Certifique-se de incluir `features = ["derive"]`
  - Sem essa feature, `#[derive(Validate)]` não funciona

- **Network error:** Se `cargo add` falhar, edite `Cargo.toml` manualmente

## Notas

- `validator` é a crate padrão da indústria para validação em Rust
- `features = ["derive"]` habilita macros `#[derive(Validate)]`
- `serde_json` será usado para formatar erros de validação de forma legível
- Essa crate é mantida ativamente e muito usada em produção