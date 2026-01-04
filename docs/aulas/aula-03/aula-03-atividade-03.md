# Atividade 03 - Médodo `PATCH`

```rust
async fn get_task(
    Path(id): Path<u32>, // id do path da api
    State(state): State<SharedState> // estado compartilhado da task
) -> impl IntoResponse {
    // Padrão em quase todas as fn, listando o vetor de tasks
    let tasks = state.lock().unwrap();

    // Buscando a task
    let task = match tasks.iter().find(|t| t.id == id) {
        // t de task, usando variáveis da forma mais simples possíveil, 
        // diferente do que eu faria em PHP ou JS, mas o "padrão" do
        // Rust é esse
        Some(t) => t,
        // Não achou? Erro!
        None => return (StatusCode::NOT_FOUND, Json(ApiResponse::Error {
            message: "Task não encontrada".into()
        }))
    };

    // Retornando task
    (StatusCode::OK, Json(ApiResponse::Success(task.clone())))
}
```

## Mudanças?

Aqui, nesse método, eu fiz uma refatoração do código e implementei `IntoResponse`. Isso se tornou necessário pois eu queria ter um `get_task()` - que será explicado na [atividade 04](aula-03-atividade-04.md). Para utilizar o `get_task()` da forma que eu queria *(que pudesse retornar tanto a task encontrada, quanto um erro)*, eu pesquisei rapidamente as melhores práticas, e a que mais me agradou para este caso em específico cou a `IntoResponse`, pode não ser a melhor prática no futuro, e irei aprender se não for.

Para isso, eu criei uma `enum`:

```rust
// Criando um enum para conseguir retornar get_task() corretamente
#[derive(Serialize)]
#[serde(untagged)] // Aprendi que isso aqui faz com que o json ignore o nome da variante
enum ApiResponse { // Ela vai substituir a TaskResponse no nosso projeto inteiro
    Success(Task), // Se deu certo, retorna json da task
    Error { message: String }, // se deu erro retorna string do erro
}
```

Esse `enum` é muito interessante, por que eu mantenho o `StatusCode` nos dois retornos, sem a necessidade de declarar eles aqui.

O retorno esperado é uma array json com as tasks:

```json
[
    {
        "id": 1,
        "title": "Criado via Postman",
        "done": false
    }
]
```

