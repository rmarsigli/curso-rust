# Rust - Aula 04 - Atividade 03

# Atividade 03 - Mover Models

No arquivo `src/models/task.rs`, cole **apenas** as structs:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: String,
    pub done: bool,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Success(Task),
    Error { message: String },
}
```

> **Nota importante:** todo `struct/enum` precisa de `pub`.

Arquivo: `src/models/mod.rs`

```rust
pub mod task;
```

Isso expõe o módulo task pra fora.

## Teste parcial:

No main.rs, adiciona no topo:

```rust
mod models;

use models::task::{Task, CreateTask, UpdateTask, ApiResponse};
```

Compile em seguida:

```rust
cargo build
```

Deve compilar sem erro (mas vai ter warnings de *"unused"*).
