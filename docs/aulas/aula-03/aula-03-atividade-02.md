# Atividade 02 - Status Code

Para usar o retorno com status code, utilizamos a biblioteca `axum::http::StatusCode`.

## Usos comuns

* `200 OK` - sucesso com body
* `201 Created` - criou recurso
* `204 No Content` - sucesso sem body
* `404 Not Found` - não encontrou
* `400 Bad Request` - dados inválidos
* `500 Internal Server Error` - erro no servidor

## Detalhes Técnicos e Curiosidades

* No Rust, é permitido multiplo retorno declarado na `fn`, como por exemplo `(StatusCode, Json<TaskResponse>)`, o compilador já entende que é um retorno em json, com um status code.
  * Em PHP 8+ eu consigo usar union types, mas não consigo retornar da forma que Rust deixa, teria que retornar uma `array`. 

### Código Atualizado

```rust
use axum::{routing::{get, post, delete}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::http::StatusCode;
use axum::extract::{State, Path};

type SharedState = Arc<Mutex<Vec<Task>>>;

#[derive(Clone, Serialize)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Serialize)]
struct TaskResponse {
    id: u32,
    title: String,
}

async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>
) -> (StatusCode, Json<TaskResponse>) {
    let mut tasks = state.lock().unwrap();
    let id = tasks.len() as u32 + 1;

    tasks.push(Task {
        id,
        title: payload.title.clone(),
        done: false,
    });

    (StatusCode::CREATED, Json(TaskResponse {
        id,
        title: payload.title,
    }))
}

async fn list_tasks(
    State(state): State<SharedState>
) -> (StatusCode, Json<Vec<Task>>) {
    let tasks = state.lock().unwrap();

    (StatusCode::OK, Json((tasks.clone())))
}

// Uma coisa muito interessante no Rus é que conseguimos
// retornos múltiplos, como aqui, por exemplo. Setamos
// StatusCode + Json<String>
async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> (StatusCode, Json<String>) {
    // Ok, aqui eu copiei, é basicamente o vetor de tasks
    let mut tasks = state.lock().unwrap();

    // Aqui vai buscar a posição da task no vetor, só queremos
    // buscar a posição, então não precisa de mut
    let pos = tasks.iter().position(|t| t.id == id);

    // Aqui temos um exemplo do nível de produção, não tem unwrap, mas match.
    // Ele vai usar Some() e None: retorna Some(title) se existe, ou None se não existe,
    // basicamente, o null do PHP/JS, já que aqui não temos null
    match pos { // o pos já foi buscado anteriormente
        Some(index) => { // Existe? retorna Some()
            // remove a task ao mesmo tempo que retorna os valores dela
            let task = tasks.remove(index);
            // Pensando na UX e retornando o título
            (StatusCode::OK, Json(format!("Task '{}' deletada!", task.title)))
        },
        None => { // Não existe, o padrão é None
            // Mensagem de que a task não existe!
            (StatusCode::NOT_FOUND, Json(String::from("Task não encontrada")))
        }
    }
}

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust API! 🦀" }))
        .route("/tasks/{id}", delete(delete_task))
        .route("/tasks", post(create_task))
        .route("/tasks", get(list_tasks))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();

    println!("🚀 Server rodando em http://127.0.0.1:3003");

    axum::serve(listener, app).await.unwrap();
}
```