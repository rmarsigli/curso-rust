# Rust - Aula 01

## Conceitos que aprendi

* Use `&` para emprestar a variável
* `mut` é um mutável, quando não usamos, as variáveis são imutáveis
* `Vec` é um `vetor`, que conheço como `array`. Assim com oas arrays em outras linguagens de programação, elas começam no index 0

### Funções nativas do Rust

* `get()` retorna um valor imutável
* `get_mut()` funciona igual `get()`, mas retorna o valor mutável
* `iter_mut().find()` é usado para iterar e filtrar com o conteúdo do índice

### Sobre o `null`

Em Rust, não existe `null`, usamos `Option<T>`, que é nativo da própria linguagem. Veja o exemplo abaixo:

```rust
enum Option<T> {
    Some(T),  // tem valor
    None,     // não tem valor
}
```

### Sobre o error handling

Em, para `exceptions`, usamos `Result<T, E>`:

```rust
enum Result<T,E> {
    Some(T),
    Error(R)
}
```

## Resultado

Este é o arquivo final da aula:

```rust
// struct define estrutura de dados (como class sem métodos)
// type é alias: type TaskId = u32;
struct Task {
    id: u32,
    title: String,
    done: bool,
}

// fn declara função, tipagem é obrigatória e verificada em compile-time,
// você percebe que tasks empresta a struct Task de forma mutável, podendo ser alterada
// title vai ser string, sempre, aqui usamos String, com maiúscula
fn add_task(tasks: &mut Vec<Task>, title: String) {
    tasks.push(Task {
        id: tasks.len() as u32 + 1,
        title,
        done: false,
    });
}

// Temos que emperstar sempre com &, pois quem
// controla a memória somos nós!
// Nota: aqui você percebe que não é mutável, pois só vamos
// iterar e imprimit valores, não alterar eles
fn list_tasks(tasks: &Vec<Task>) {
    // A iteração é elegante, sem parênteses
    for task in tasks {
        println!("[{}] {} - {}",
                 task.id,
                 if task.done { "x" } else { " " },
                 task.title
        );
    }
}

// Aqui é mutável! Pois vamos alterar task.done
// Então essa função pede a task e o id
fn mark_done(tasks: &mut Vec<Task>, id: u32) {
    // Essa iteração tem uma ótima tradução:
    // SE (if let) conseguir pegar (Some) uma task mutável (task) 
    // do vetor (tasks.get_mut) no índice (id - 1 convertido pra 
    // usize), ENTÃO executa o bloco 
    if let Some(task) = tasks.get_mut((id - 1) as usize) {
        task.done = true;
    }
}

// Aqui eu tipo o retorno Result<(), String>,
// que pode ser tanto vazio (deu certo), quanto string (o erro)
fn remove_task(tasks: &mut Vec<Task>, id: u32) -> Result<(), String> {
    // Apenas remover a task não é saudável, eu preciso VERIFICAR se a task existe
    // Rust não valida lógica de negócio,
    // então preciso verificar índice manualmente

    // id do index
    let index = (id - 1) as usize;

    // verifica se o index é maior que número de tasks
    if index >= tasks.len() {
        return Err(format!("Task {} não existe", id));
    }

    // Já que está ok, remove o index!
    tasks.remove(index);
    Ok(())
}

// Função principal, que roda automaticamente no build
fn main() -> Result<(), String> {
    // Cria a variável de forma mutável
    // Não preciso emprestar aqui, pois estou criando ela pela
    // primeira vez
    let mut tasks: Vec<Task> = Vec::new();

    // vou adicionando as tasks com essa função, como
    // a array já existe, tenho que emprestar
    add_task(&mut tasks, String::from("Aprender Rust"));
    add_task(&mut tasks, String::from("Fazer API REST"));
    add_task(&mut tasks, String::from("Deploy"));

    // Printando tasks
    list_tasks(&tasks);

    // Marcar uma como concluída
    mark_done(&mut tasks, 1);

    // unwrap() vai crashar no runtime se der erro
    // em produção, match é uma forma melhor de tratar o erro diretamente
    // e sem rodeios, já retornando o erro para uma
    // verificação melhor
    match remove_task(&mut tasks, 2) {
        Ok(()) => println!("Task Removida!"),
        Err(e) => println!("Erro: {}", e),
    }

    // é macro com formatting + newline automático, igual o do C
    println!("\nDepois de modificar:");

    // Listar as tasks atualizadas
    list_tasks(&tasks);

    Ok(())
}
```

### Sobre o `u32` e `as usize`

`Vec` usa `usize` como tipo de índice porque:

* `usize` = tamanho da arquitetura (32 ou 64 bits)
* Arrays/Vecs precisam de índices do tamanho da memória do sistema
* `u32` é sempre 32 bits (pode ser pequeno demais em sistemas 64-bit)

## Dúvidas para revisar

* Por que `usize` especificamente? *(entendo conversão, mas não o motivo)*
* Diferença entre `Option<T>` e `Result<T, E>` na prática
* Quando usar `unwrap()` vs `match` vs `?`
