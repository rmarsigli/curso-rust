# Atividade 01 - Método `DELETE`

Aqui o objetivo é deletar uma task via chamada da api, com `curl -X DELETE http://127.0.0.1:3004/tasks/1`. Para isso preciso aprender `Path params` no **Axum**.

O código da atividade é esse:

```rust
use axum::extract::Path;

async fn delete_task(
    Path(id): Path<u32>,  // ← extrai :id da URL
    State(state): State<SharedState>
) -> /* ??? */ {
    // seu código aqui
}
```

Como que ficou?

```rust
// Nos imports
use axum::{routing::{get, post, delete}, Router, Json};

// A função de delete
async fn delete_task(
    Path(id): Path<u32>,
    State(state): State<SharedState>
) -> Json<String> {
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
            Json(format!("Task '{}' deletada!", task.title))
        },
        None => { // Não existe, o padrão é None
            // Mensagem de que a task não existe!
            Json(String::from("Task não encontrada"))
        }
    }
}

// A rota de exclusão
...
.route("/tasks/{id} ", delete(delete_task))
...
```

* `.position()` retorna `Option<usize>` (índice)
* `.remove(index)` remove **e também** retorna a task removida
  * Assim temos acesso a `task.title` pra mensagem de retorno

### Testando

```shellscript
# Primeiro, rodei o comando para criar uma task
curl -X POST http://127.0.0.1:3003/tasks -H "Content-Type: application/json" -d '{"title":"Task 1"}'

# Obtive o resultado correto
{"id":1,"title":"Task 1"}

# Agora, vou apagar a task de id 1
curl -X DELETE http://127.0.0.1:3003/tasks/1

# Resposta
"Task 'Task 1' deletada!"
```