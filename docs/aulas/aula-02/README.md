# Aula 02 - Básico de API

Nessa aula vou entender o básico de API.

***

Código usado:

```rust
use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::extract::State;

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
) -> Json<TaskResponse> {
    let mut tasks = state.lock().unwrap();
    let id = tasks.len() as u32 + 1;

    tasks.push(Task {
        id,
        title: payload.title.clone(),
        done: false,
    });

    Json(TaskResponse {
        id,
        title: payload.title,
    })
}

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust API! 🦀" }))
        .route("/tasks", post(create_task))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();

    println!("🚀 Server rodando em http://127.0.0.1:3003");

    axum::serve(listener, app).await.unwrap();
}
```

***

## Sobre o Async

Veja abaixo um exemplo de código normal (síncrino):

```rust
fn buscar_dados() {
    let dados = database.query(); // BLOQUEIA aqui 500ms
    processar(dados);
}
```

E aqui um código com async:

```rust
async fn buscar_dados() {
    let dados = database.query().await; // NÃO bloqueia
    processar(dados);
}
```

Nesse caso, a CPU **não espera** terminar, ela vai fazer outras coisas. É o padrão em outras linguagens, mas resumindo de forma simples:

* **Síncrono vai:** esperar a sua vez, ficar parado olhando pro teto, não vai atender mais ninguém até terminar
* **Asíncrono vai:** Pegar esse pedido na mesa 1, pegar outro pedido na mesa 2, pegar outro pedido na mesa 3, voltar quando algum estiver pronto

> ***Nota:****&#x20;Async não faz nada, precisamos de runtime para executar, por isso usamos o pacote&#x20;****Tokio****.*

### É aqui que entra o Tokio

Tokio é o motor que executa o código async, precisamos dele para fazer tudo isso acontecer. `async fn` é a receita de bolo, e Tokio é o forno que vai assar esse bolo. Sem Tokio, o `async fn` não vai fazer nada.

Exemplo prático:

```rust
#[tokio::main]  // ← Essa macro cria o runtime
async fn main() {
    // Tokio gerencia:
    //   1. Thread pool
    //   2. Event loop
    //   3. Scheduler de tasks
    //   4. I/O não bloqueante
}
```

Em poucas palavras, `#[tokio::main]` diz *"Tokio, cria infraestrutura pra rodar async aqui"*. Quer entender melhor? por baixo dos panos, o código abaixo:

```rust
#[tokio::main]
async fn main() { ... }
```

Vira esse código:

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // seu código async aqui
    });
}
```

Viu? Até que Rust tem seus atalhos! 🤣

## Axum

> Axum é o equivalente ao **Laravel ou Express** do Rust *(mas muito mais simples)*. Feito pelo time do Tokio. É construído em cima de Tokio.

### Por que usar o Axum?

Existe outras alternativas, como Actix-web (mais rápido, API chata), Rocket (fácil, mas macros demais), Warp (potente, curva de aprendizado alta).

Mas Axum é equilíbrio perfeito, ele é **rápido**, **simples** e **type-safe**.

```rust
Router::new() // Cria router vazio
    .route("/", get(handler)) // GET /
    .route("/users", post(criar)) // POST /users
```

## O que é `Arc<Mutex<Vec<Task>>>`?

Olhe o código, camada por camada:

```rust
type SharedState = Arc<Mutex<Vec<Task>>>;
//                 ^^^ ^^^^^ ^^^^^^^^
//                 |   |     └─ Vector de Tasks
//                 |   └─ Trava (lock) pra thread safety
//                 └─ Contador de referências pra compartilhar
```

* **`Vec<Task>`****:** é basicamente uma lista de tasks (Vector de Task)
* **`Mutex<Vec<Task>>`****:** exclusão mútua (Mutual Exclusion)

Sem o Mutex:

1. **Thread 1:** lê tasks\[0]
2. **Thread 2:** escreve tasks\[0] ← RACECONDITION
3. **Thread 1:** usa valor antigo ← BUG

O mutex muda isso (como o próprio nome já insinua), ele quer dizer *"só uma pessoa por vez pode mexer nisso"*.

```rust
let mut tasks = state.lock().unwrap(); // TRAVA
// Só UMA thread pode acessar agora
tasks.push(...); // seguro
// Quando tasks sai de escopo, DESTRAVA automaticamente
```

### E o Arc?

> **Arc** diz *"várias pessoas podem ter referência, mas só uma mexe por vez (via Mutex)"*

Arc é Atomic Reference Counting, qual o problema? sem ele, você precisa compartilhar tasks entre:

* Handler de POST /tasks
* Handler de GET /tasks (futuro)
* Handler de DELETE /tasks (futuro)

Mas, no Rust, como compartilhar? Primeiro, o que **não pode**:

```rust
let tasks = Vec::new();
```

A solução, com Arc:

```rust
let state = Arc::new(Mutex::new(Vec::new()));
let clone1 = Arc::clone(&state); // contador += 1
let clone2 = Arc::clone(&state); // contador += 1
// Todos apontam pro MESMO dado
// Quando todos saem de escopo, libera memória
```

## Sobre `#[derive(Clone, Serialize)]`

> Derive diz **"Ei Rust, gera código automaticamente pra mim"**

### Sobre o Clone:

```rust
#[derive(Clone)]
struct Task { ... }

// Rust gera isso automaticamente:
impl Clone for Task {
    fn clone(&self) -> Self {
        Task {
            id: self.id,
            title: self.title.clone(),
            done: self.done,
        }
    }
}
```

Mas, por que precisa?

```rust
title: payload.title.clone(), // ← sem derive(Clone), não compila
```

### Sobre o Serialize

```rust
#[derive(Serialize)]
struct Task { ... }

// Rust gera código que converte Task → JSON
```

E por que é necessário?

```rust
Json(TaskResponse { ... }) // ← Axum converte pra JSON
// Sem Serialize, não sabe como
```

## Sobre o `State(state): State<SharedState>`

> É o pattern matching no argumento da função.

O que o axum faz?

```rust
async fn create_task(
    State(state): State<SharedState>, // ← extrai state de dentro de State
    Json(payload): Json<CreateTask> // ← extrai payload de dentro de Json
)

// é equivalente a:

async fn create_task(wrapper: State<SharedState>, json_wrapper: Json<CreateTask>) {
    let state = wrapper.0; // extrai manualmente
    let payload = json_wrapper.0; // extrai manualmente
}
```

Pattern matching faz isso automaticamente:

```rust
State(state)  // "pega o que tá dentro de State e chama de state"
Json(payload) // "pega o que tá dentro de Json e chama de payload"
```

## Sobre `state.lock().unwrap()`

```rust
let mut tasks = state.lock().unwrap();
```

O Que Acontece aqui:

* state.lock():
  * Tenta pegar o lock do Mutex
  * Se outra thread tem lock, ESPERA até liberar
  * Retorna `Result<MutexGuard, PoisonError>`
* unwrap():
  * Se lock deu certo → extrai MutexGuard
  * Se lock deu erro → PANIC (crash)
* MutexGuard:
  * É um "token de acesso"
  * Enquanto existe, você tem acesso exclusivo
  * Quando sai de escopo, destrava automaticamente

```rust
{
    let mut tasks = state.lock().unwrap(); // TRAVA
    tasks.push(...); // usa
} // MutexGuard destruído aqui → DESTRAVA automático
```

## Sobre  `.with_state(state)`

````rust
let app = Router::new()
    .route("/tasks", post(create_task))
    .with_state(state); // ← injeta estado no router
```

**O que faz:**
- Guarda `state` dentro do Router
- Quando request chega em `/tasks`:
  - Axum **clona** o `Arc` (barato, só incrementa contador)
  - Passa pro handler via `State(state)`

**Sem .with_state():**
```
error: handler requires `State<SharedState>` but router has no state
````

## Sobre tomadas de decisão do uso do `unwrap`

Essa é uma **questão pertinente**. unwrap() é usado apenas, exclusivamente, em desenvolvimento, para escrever rápido e validar o código. Mas para produção ele não é usado, em nenhum caso, nós usamos `expect()` ou *(mais completo, só que mais verboso)* `match`. Veja os exemplos:

Usando `unwrap()`:

```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
    .await
    .unwrap(); // ← PROBLEMA EM PRODUÇÃO!
```

Usando `expect()`:

```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
    .await
    .expect("Falha ao bindar porta 3003"); // Melhor que unwrap
```

usando `match`:

```rust
let listener = match tokio::net::TcpListener::bind("127.0.0.1:3003").await {
    Ok(l) => l,
    Err(e) => {
        eprintln!("Erro ao bindar: {}", e);
        std::process::exit(1);
    }
}; // Enorme, repetitivo, mas completo!
```

**Pergunta comum:** se `unrwap` vai ter que ser retaforado, por que raios usar então? em protótipos, ele vai funcionar rápido, economizando dezenas ou centenas de linhas de código, mas com ciência de que vai ter que **refatorar tudo** depois.

***

## Conceitos que ainda NÃO domino completamente

### Thread Safety

* Entendi a teoria (Mutex = lock, Arc = compartilhamento)
* **NÃO testei na prática:** o que acontece se REMOVER Mutex? E se remover Arc?
* Preciso quebrar o código de propósito pra entender

### MutexGuard

* Sei que existe e destrava automático
* **NÃO entendo:** por que é tipo especial? Por que não é só bool?

### Pattern Matching em Funções

* Entendi `State(state): State<SharedState>`
* **NÃO domino:** quando/onde mais posso usar isso?

### Lifecycle de Dados

* Sei que Vec tá em memória
* **NÃO entendo:** quando exatamente é liberada? Quando Arc conta chega a zero?

**Próxima aula:** testar quebrando o código de propósito

***

## Experimentos e Testes

[Aula 02 - Testes](./aula-02-atividade-01.md)
