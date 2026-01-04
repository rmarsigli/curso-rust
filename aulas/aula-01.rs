// # Colinha
// & para emprestar
// mut é mutável, pode alterar
// Vec é uma array/vector
// get retorna imutável
// get_mut() retorna mutável!
// Vec deve ser tratada como uma array comum, começa em 0
// Em rust, não existe null
// get_mut() só funciona filtrando pelo índice
// iter_mut().find() é usado para iterar e filtrar com o conteúdo do índice

struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn add_task(tasks: &mut Vec<Task>, title: String) {
    tasks.push(Task {
        id: tasks.len() as u32 + 1,
        title,
        done: false,
    });
}

fn list_tasks(tasks: &Vec<Task>) {
    for task in tasks {
        println!("[{}] {} - {}",
                 task.id,
                 if task.done { "x" } else { " " },
                 task.title
        );
    }
}

fn mark_done(tasks: &mut Vec<Task>, id: u32) {
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

fn main() -> Result<(), String> {
    let mut tasks: Vec<Task> = Vec::new();

    add_task(&mut tasks, String::from("Aprender Rust"));
    add_task(&mut tasks, String::from("Fazer API REST"));
    add_task(&mut tasks, String::from("Deploy"));

    list_tasks(&tasks);

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
