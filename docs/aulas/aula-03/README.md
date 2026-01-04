# Rust - Aula 03 - API REST

## Atividades desta aula:

- [atividade 01](aula-03-atividade-01.md)
- [atividade 02](aula-03-atividade-02.md)
- [atividade 03](aula-03-atividade-03.md)
- [atividade 04](aula-03-atividade-04.md)

## Resumos com notas do Professor

- [resumo 01](../../resumos/professor/2025-12-31.md)
- [resumo 02](../../resumos/professor/2026-01-01.md)
- [resumo 03](../../resumos/professor/2026-01-02.md)

## Código Final

```rust
use axum::{routing::{get, post, delete, patch}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::extract::{State, Path};

type SharedState = Arc<Mutex<Vec<Task>>>;

// Criando um enum para conseguir retornar get_task() corretamente
#[derive(Serialize)]
#[serde(untagged)] // Aprendi que isso aqui faz com que o json ignore o nome da variante
enum ApiResponse {
    Success(Task),
    Error { message: String },
}

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

#[derive(Deserialize)]
struct UpdateTask {
    title: String,
    done: bool,
}

async fn create_task(
    State(state): State<SharedState>,
    Json(payload): Json<CreateTask>
) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();
    let id = tasks.len() as u32 + 1;

    // Criando nova task
    let new_task = Task {
        id,
        title: payload.title.clone(),
        done: false,
    };

    // Adicionando no vetor
    tasks.push(new_task.clone());

    // Retorno de sucesso
    (StatusCode::CREATED, Json(ApiResponse::Success(new_task.clone())))
}

async fn list_tasks(
    State(state): State<SharedState>
) -> (StatusCode, Json<Vec<Task>>) {
    let tasks = state.lock().unwrap();

    (StatusCode::OK, Json(tasks.clone()))
}

async fn get_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> impl IntoResponse {
    let tasks = state.lock().unwrap();

    // Buscando a task
    let task = match tasks.iter().find(|t| t.id == id) {
        Some(t) => t,
        // Não achou? Erro!
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::Error {
            message: "Task não encontrada".into()
        }))
    };

    // Retornando task
    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}

async fn update_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTask>
) -> impl IntoResponse {
    let mut tasks = state.lock().unwrap();

    // Buscando a task
    let task = match tasks.iter_mut().find(|t| t.id == id) {
        Some(t) => t,
        // Não achou! Erro!
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::Error {
            message: "Task não encontrada".into()
        })),
    };

    // Atualizando os campos
    task.done = payload.done;
    task.title = payload.title.clone();

    // Retornando sucesso
    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}

async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> (StatusCode, Json<String>) { // Aqui não precisa do IntoResponse, porque o retorno vai ser sempre esse
    let mut tasks = state.lock().unwrap();

    let pos = tasks.iter().position(|t| t.id == id);

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
        .route("/tasks/{id}", patch(update_task))
        .route("/tasks/{id}", get(get_task))
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