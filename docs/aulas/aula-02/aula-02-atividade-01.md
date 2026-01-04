# Rust - Aula 02 - Testes

## Removendo `Arc<>`

Testando:

```rust
use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::extract::State;

// E se eu deixar de usar o Arc?
// type SharedState = Arc<Mutex<Vec<Task>>>;
type SharedState = Mutex<Vec<Task>>;
```

O resultado no console vai ser esse:

```shellscript
rafhael@rafha:~/www/html/rust/curso-rust$ cargo build
   Compiling curso-rust v0.1.0 (/home/rafhael/www/html/rust/curso-rust)
error[E0308]: mismatched types
  --> src/main.rs:48:30
   |
48 |     let state: SharedState = Arc::new(Mutex::new(Vec::new()));
   |                -----------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<Vec<Task>>`, found `Arc<Mutex<Vec<_>>>`
   |                |
   |                expected due to this
   |
   = note: expected struct `std::sync::Mutex<Vec<Task>>`
              found struct `Arc<std::sync::Mutex<Vec<_>>>`
```

Além desse erro, ele desencadeia diversos outros erros no console, que acontecem pela simples falta do `Arc<>`.

Por que esse erro acontece?

* Sem o **Arc**, *não consigo* compartilhar a referência manualmente entre os handlers do código (GET, POST, DELETE, etc.)
* Cada handler tentaria ter ownership exclusivo do `Vec<>`
* Por padrão, Rust **não permite** ownership compartilhado
* `Arc<>` permite múltiplas referências
* `Mutex<>` é a camada de proteção que evita modificação simultânea
* Recapitulando o nome, **Arc** significa *Atomic Reference Counting*

## Removendo o `Mutex<>`

Nós já entendemos a necessidade do encapsulamento do `<Arc<Mutex<...>>`, mas e se o `Arc<>` continuar, mas o `Mutex<>` sair?

```rust
use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use axum::extract::State;

// E se eu deixar de usar o Mutex?
// type SharedState = Arc<Mutex<Vec<Task>>>;
type SharedState = Arc<Vec<Task>>;
```

O resultado no console vai ser esse:

```shellscript
rafhael@rafha:~/www/html/rust/curso-rust$ cargo build
   Compiling curso-rust v0.1.0 (/home/rafhael/www/html/rust/curso-rust)
error[E0599]: no method named `lock` found for struct `Arc<Vec<Task>>` in the current scope
  --> src/main.rs:31:27
   |
31 |     let mut tasks = state.lock().unwrap();
   |                           ^^^^ method not found in `Arc<Vec<Task>>`

error[E0308]: mismatched types
  --> src/main.rs:48:39
   |
48 |     let state: SharedState = Arc::new(Mutex::new(Vec::new()));
   |                              -------- ^^^^^^^^^^^^^^^^^^^^^^ expected `Vec<Task>`, found `Mutex<Vec<_>>`
   |                              |
   |                              arguments to this function are incorrect
   |
   = note: expected struct `Vec<Task>`
              found struct `std::sync::Mutex<Vec<_>>`
note: associated function defined here
  --> /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/alloc/src/sync.rs:417:12

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `curso-rust` (bin "curso-rust") due to 2 previous errors
```

Aqui dá os erros que já esperamos, o resto do código espera que o `Mutex<>` esteja sendo usado, então precisamos de um teste mais profundo (a seguir).

## Teste profundo

Ah, mas e se removermos essas referências do código??

```rust
// Removendo referências, deixando o código assim
let mut tasks = state.unwrap();
let state: SharedState = Arc::new(Vec::new());
```

O state funciona, mas ele não consegue compilar com o `unwrap()`:

```shellscript
error[E0599]: no method named `unwrap` found for struct `Arc<Vec<Task>>` in the current scope
  --> src/main.rs:31:27
   |
31 |     let mut tasks = state.unwrap();
   |                           ^^^^^^
   |
help: there is a method `swap` with a similar name, but with different arguments
  --> /rustc/ded5c06cf21d2b93bffd5d884aa6e96934ee4234/library/core/src/slice/mod.rs:901:5
```

* `Arc<Vec<T>>` não tem `.unwrap()` porque não é **Option** nem **Result**
  * **Nota importante:** `.unwrap()` só funciona com **Option** *(o null do Rust)* ou **Result** *(o Error Handler do Rust)*
* Para funcionar, deveria ser `let tasks = &*state;`. Mas isso é perigoso, pois não tem a camada de segurança do `Mutex<>`

### Conclusões importantes sobre `Arc<>` e `Mutex<>`

* Sem Mutex, **não existe método pra modificar** o Vec dentro do Arc
* Arc só permite **leitura compartilhada**, não **escrita compartilhada**
  * E é exatamente por isso que `Mutex<>` não está imbutido no `Arc<>`, pois se for apenas para leitura (uma API de leitura apenas, eu tenho APIs assim, então é um uso de caso real), não há necessidade nenhuma do `Mutex<>`
* Os dois juntos - praticamente sempre - é pelo motivo básico de:
  * `Arc<>` é compartilhar
  * `Mutex<>` é modificar com segurança

Eu poderia fazer esse código funcionar *parcialmente*, mas **sem conseguir adicionar tasks de verdade:**

```rust
// importando os packages
use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::{Arc};
use axum::extract::State;

type SharedState = Arc<Vec<Task>>;

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
    let tasks = &*state;
    let id = tasks.len() as u32 + 1;

    Json(TaskResponse {
        id,
        title: payload.title,
    })
}

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Vec::new());

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust API! 🦀" }))
        .route("/tasks", post(create_task))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();

    // Printa no terminal que o server tá rodando
    println!("🚀 Server rodando em http://127.0.0.1:3003");

    // Servindo o framework web
    axum::serve(listener, app).await.unwrap();
}
```

**Por que não funciona completamente?**

* `&*state` é referência imutável
* Não posso fazer `.push()`
* Só consigo **ler**, não **modificar**
* **Por isso Arc + Mutex sempre juntos em APIs de escrita**

#### Adicionando o método `GET` com `/tasks`

```rust
async fn list_tasks(
    State(state): State<SharedState>
) -> Json<Vec<Task>> {
    let tasks = state.lock().unwrap();
    Json(tasks.clone());
}
```

Do jeito que está aí, vamos ter o erro:

```shellscript
error[E0308]: mismatched types
  --> src/main.rs:59:22
   |
59 |   ) -> Json<Vec<Task>> {
   |  ______________________^
...  |
68 | |     Json(tasks.clone());
   | |                        - help: remove this semicolon to return this value
69 | | }
   | |_^ expected `Json<V    Building [=========================> ] 74/76: curso-rust(bin test), curso-rust(bin)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               ec<Task>>`, found `()`
   |
   = note: expected struct `Json<Vec<Task>>`
           found unit type `()`
```

Esse é um erro comum de quem vem de outra linguagem, como eu. O retorno **não tem** ponto e vírgula. Deve ser assim:

```rust
Json(tasks.clone())
```

Como não precisamos usar return nas funções de Rust (algo incrível), é a falta do ponto e vírgula que vai fazer o interpretador *"entender"* que estamos returnando.

Agora eu só preciso adicionar no router:

```rust
let app = Router::new()
  .route("/", get(|| async { "Hello Rust API! 🦀" }))
  .route("/tasks", post(create_task))
  .route("/tasks", get(list_tasks))
  .with_state(state);
```

## Por Que `.clone()`?

Vamos testar build sem `.clone()`? Eu tenho o seguinte erro:

```shellscript
error[E0382]: use of moved value: `payload.title`
  --> src/main.rs:41:16
   |
35 |         title: payload.title,
   |                ------------- value moved here
...
41 |         title: payload.title,
   |                ^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `payload.title` has type `std::string::String`, which does not implement the `Copy` trait
```

### **Por que precisa clonar?**

Por causa do **Ownership&#x20;**(e não pela trait `copy()`, como parece ser)**:**

```rust
tasks.push(Task {
    title: payload.title,  // ← MOVE payload.title pra dentro do Task
});

Json(TaskResponse {
    title: payload.title,  // ← ERRO: já foi movido acima!
})
```

* `payload.title` é movido pro `Task`
* `payload.title` não existe mais
* Ownership move o valor *(transfere "dono")*
* Após mover, valor original não existe mais
* Não pode usar de novo *(compile-time garante isso)*

> **Lembrete sobre a memória em Rust:** Rust gerencia memória automaticamente via ownership, mas em compile-time *(não precisa de Garbage Collector em runtime)*

A solução então:

```shellscript
title: payload.title.clone(),  // Clona antes de mover
```

Assim, o clone vai pro `Task`, e o original fica para o `TaskResponse`.

#### Entendendo melhor o Ownership, o uso de `copy()` e Copy vs Clone

Copy e Clone são **Traits Diferentes**.

**Tipos com Copy trait (cópia automática)**

* `u32`, `i32`, `bool`, `char` - TÊM Copy
* São copiados automaticamente (implícito)
* Pequenos, na stack, baratos de copiar

**Tipos sem Copy, mas com Clone (cópia explícita)**

* `String`, `Vec`, `HashMap` - NÃO TÊM Copy
* Precisam de `.clone()` explícito
* Grandes, heap-allocated, podem ser caros

**Exemplo:**

```rust
// Copy automático:
let x: u32 = 5;
let y = x;  // copia automático
println!("{}", x);  // funciona!

// Clone explícito:
let x = String::from("hi");
let y = x.clone();  // .clone() necessário
println!("{}", x);  // funciona!

// Sem clone = move:
let x = String::from("hi");
let y = x;  // MOVE (não copia)
println!("{}", x);  // ERRO! ❌
```

Portanto, copiar dados que podem ter valores altos automaticamente seria caro, e iria contra toda a proposta do Rust. Com esses campos, ele te força a decidir se 1) move ou 2) clona explicitamente.

### **O que aconteceria se não clonasse?**

Como descrito no código acima, não compila

### **Tem impacto de performance?**

Sim, o clone vai copiar os dados. Mas aqui usamos quando é necessário - e não sempre, que é o padrão de muitas linguagens de programação. Existe alternativa? Sim, mas vou aprender mais pra frente.

### `Clone` é má prática?

Como quase tudo no mundo moderno há polarização de pensamentos antagônicos que não buscam um meio termo.

* **O team Clone é pragmático:** às vezes clone é a solução mais simples; trazer otimização prematura é raiz do mal; clone claro > código complexo sem clone
* **O team NoClone é mais reacionário:** `Clone` tem custo de performance, bom código Rust evita clones; use referências quando possível

> Me conhecendo, eu provavelmente em muitos casos adotaria o modo reacionário, mas entendendo que `clone()` não é errado. É como usar Spatie Data + Value Objects no Laravel, e entender que não é errado - em momento algum - usar Requests padrão do Laravel.

No mundo real, na prática. Colone não é má prática quando:

* Você precisa do dado em 2 ou mais lugares
* Dados são pequenos (strings curtas, structs pequenas)
* Alternativa seria muito mais complexa
* Nesse ponto, a performance não é crítica

E é sim, má prática quando:

* Clonar Vec gigante em loop
* Clonar para burlar o ownership (*"é complexo demais pra mim, mete o clone aí"*)
* Clonar quando referência resolveria
* Clonarem hot path *(código executado milhões de vezes)*

## Código Final

Aqui está p código final dessa aula:

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

async fn list_tasks(
    State(state): State<SharedState>
) -> Json<Vec<Task>> {
    let tasks = state.lock().unwrap();
    Json(tasks.clone())
}

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust API! 🦀" }))
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

Aqui está o código de estudo, com todas as observações:

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

// Adicionando função de criar tasks
// Vamos resumir aqui pra ver se eu entendi:
// async = assíncrono padrão
// fn = function
// dentro de function() temos o que passa dentro da função (State), e a sua tipagem,
// -> Json<Vec<Task>> = retorna um Vec de Tasks, envolvido em Json (pra serializar automaticamente)
// ---
// State (o estado) importamos de uma lib externa, state é o nome da variável,
// e o tipo dela, é SharedState (que definimos lá em cima) que segue o formato
// de State(), tanto o `{nome da var}:{tipo da var}` vão estar formatados como State
// quer, por isso usamos duas vezes
async fn list_tasks(
    State(state): State<SharedState>
) -> Json<Vec<Task>> {
    // .lock() -> pq estamos usando Mutex, estamos travando ela
    // para usar (e ninguém mais, claro que só agora né, resolveu = liberou)
    // .unwrap() -> debugar erro no ambiente local
    // Meu IDE RustRover já marca tasks como `:MutexGuard<Vec<Task>>`, então dá pra ver que está
    // funcionando 100%
    let tasks = state.lock().unwrap();

    // Isso é incrível, não preciso de retorno, Rust já retorna a última linha da função
    Json(tasks.clone())
}

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust API! 🦀" }))
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

## Descobertas Importantes

### Return Implícito

Rust não precisa de `return`. Última linha SEM ponto-e-vírgula = retorno automático.

```rust
fn soma(a: i32, b: i32) -> i32 {
    a + b  // SEM ; = retorna
}
```

### MutexGuard

`state.lock()` retorna `MutexGuard<Vec<Task>>`.

É um "token de acesso"

* Enquanto existe, você tem acesso exclusivo
* Quando sai de escopo, destrava automático

## Dúvidas Pendentes (revisar depois)

* Copy trait vs Clone trait - diferença exata?
* Como Rust gerencia memória em compile-time?
* Quando usar referência vs clone?
