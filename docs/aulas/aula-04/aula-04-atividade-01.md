# Rust - Aula 04 - Atividade 01

# Atividade 01 - Entender Os Módulos

## O que são módulos?

**PHP/Laravel:**

```php
// app/Models/Task.php
namespace App\Models;
class Task { }

// app/Http/Controllers/TaskController.php
namespace App\Http\Controllers;
use App\Models\Task;
class TaskController { }
```

**Rust equivalente:**

```rust
// src/models/task.rs
pub struct Task { }

// src/handlers/tasks.rs
use crate::models::task::Task;
pub async fn create_task() { }
```

**Diferenças:**

| PHP                 | Rust               |
| ------------------- | ------------------ |
| `namespace`         | `mod`              |
| `use`               | `use`              |
| Autoload (Composer) | Manual (mod.rs)    |
| Public por padrão   | Private por padrão |

## Estrutura de módulos Rust



```rust
// src/main.rs
mod models;  // declara módulo
mod handlers;
mod routes;

use models::task::Task;  // importa

fn main() {
    let task = Task { ... };
}
```

```rust
// src/models/mod.rs
pub mod task;  // expõe submódulo
```

```rust
// src/models/task.rs
pub struct Task {  // pub = público
    pub id: u32,
    pub title: String,
}
```

## Palavras-chave

* **`mod`** - declara módulo
* **`pub`** - torna público (default é privado)
* **`use`** - importa
* **`crate::`** - raiz do projeto
* **`super::`** - módulo pai
* **`self::`** - módulo atual
