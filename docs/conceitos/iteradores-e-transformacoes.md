# Iteradores e Transformações de Dados

> **Conceito:** Como transformar coleções de dados usando iteradores, map, filter_map, collect e join  
> **Nível:** Intermediário  
> **Relacionado:** Aula 05 (Validação - `format_validation_errors`)

## Problema Real

Você tem erros de validação assim:

```rust
ValidationErrors {
    title: [ValidationError { message: Some("Título inválido") }],
    email: [ValidationError { message: Some("Email inválido") }]
}
```

E precisa transformar em string legível:

```
"title: Título inválido; email: Email inválido"
```

**Como fazer isso em Rust?**

---

## Conceito: Pipeline de Transformação

Em Rust, transformamos dados através de um **pipeline**:

```
dados → .iter() → .map() → .filter() → .collect()
```

Cada operação transforma dados **sem modificar o original** (imutável).

---

## Operações Básicas

### 1. `.iter()` - Criar Iterador

```rust
let numbers = vec![1, 2, 3];
let iter = numbers.iter();  // iterador sobre &1, &2, &3
```

**O que faz:**
- Cria iterador sobre referências (`&T`)
- NÃO consome o Vec original
- Pode iterar múltiplas vezes

**Comparação PHP:**
```php
$numbers = [1, 2, 3];
foreach ($numbers as $n) { ... }  // similar
```

---

### 2. `.map()` - Transformar Cada Item

```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|n| n * 2)  // transforma cada item
    .collect();

// doubled = [2, 4, 6]
```

**O que faz:**
- Aplica função em cada item
- Retorna novo iterador transformado
- **Lazy** (só executa quando chamar `.collect()`)

**Comparação PHP:**
```php
$numbers = [1, 2, 3];
$doubled = array_map(fn($n) => $n * 2, $numbers);
// [2, 4, 6]
```

**Comparação JavaScript:**
```javascript
const numbers = [1, 2, 3];
const doubled = numbers.map(n => n * 2);
// [2, 4, 6]
```

---

### 3. `.filter()` - Filtrar Items

```rust
let numbers = vec![1, 2, 3, 4, 5];
let evens: Vec<&i32> = numbers
    .iter()
    .filter(|n| *n % 2 == 0)  // só pares
    .collect();

// evens = [&2, &4]
```

**O que faz:**
- Mantém apenas items que passam teste
- Retorna iterador filtrado

**Comparação PHP:**
```php
$numbers = [1, 2, 3, 4, 5];
$evens = array_filter($numbers, fn($n) => $n % 2 == 0);
// [2, 4]
```

---

### 4. `.filter_map()` - Transformar E Filtrar

**O mais confuso, mas super útil!**

```rust
let strings = vec!["1", "dois", "3", "quatro"];

let numbers: Vec<i32> = strings
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())  // tenta parsear, ignora erros
    .collect();

// numbers = [1, 3]  (ignora "dois" e "quatro")
```

**O que faz:**
- Aplica função que retorna `Option<T>`
- Mantém apenas `Some(value)`
- Descarta `None`
- **Combina `.map()` + `.filter()` em um!**

**Equivalente verboso:**
```rust
let numbers: Vec<i32> = strings
    .iter()
    .map(|s| s.parse::<i32>().ok())     // Vec<Option<i32>>
    .filter(|opt| opt.is_some())        // só Some
    .map(|opt| opt.unwrap())            // extrai valor
    .collect();
```

**Comparação PHP (não tem equivalente direto):**
```php
$strings = ["1", "dois", "3", "quatro"];
$numbers = array_filter(
    array_map(fn($s) => is_numeric($s) ? (int)$s : null, $strings),
    fn($n) => $n !== null
);
// [1, 3]
```

---

### 5. `.collect()` - Materializar Resultado

```rust
let numbers = vec![1, 2, 3];

// Até aqui, nada foi executado (lazy):
let iter = numbers.iter().map(|n| n * 2);

// Agora sim executa tudo:
let result: Vec<i32> = iter.collect();
```

**O que faz:**
- Executa todo o pipeline
- Coleta resultados em coleção
- Pode coletar em: `Vec`, `HashMap`, `String`, etc.

**Importante:** Precisa anotar tipo (`Vec<i32>`) OU usar turbofish (`::<Vec<_>>()`).

---

## Caso Real: `format_validation_errors()`

Agora vamos analisar a função da Aula 05:

```rust
fn format_validation_errors(errors: &ValidationErrors) -> String {
    errors
        .field_errors()           // HashMap<&str, Vec<ValidationError>>
        .iter()                   // iterador sobre (&str, &Vec<...>)
        .map(|(field, errors)| {  // para cada campo:
            let messages: Vec<String> = errors
                .iter()           // itera sobre erros do campo
                .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                .collect();       // coleta mensagens
            format!("{}: {}", field, messages.join(", "))
        })
        .collect::<Vec<_>>()      // coleta campos formatados
        .join("; ")               // junta com ";"
}
```

### Passo a passo com exemplo:

**Input:**
```rust
ValidationErrors {
    title: [
        ValidationError { message: Some("Título inválido") },
        ValidationError { message: Some("Mín 4 chars") }
    ],
    email: [
        ValidationError { message: Some("Email inválido") }
    ]
}
```

**Execução:**

**1. `.field_errors().iter()`**
```rust
[
    ("title", [ValidationError {...}, ValidationError {...}]),
    ("email", [ValidationError {...}])
]
```

**2. `.map(|(field, errors)| ...)`**

Para `("title", [...])``:

```rust
// 2a. errors.iter()
[ValidationError {...}, ValidationError {...}]

// 2b. .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
// ValidationError { message: Some("Título inválido") }
//   → e.message.as_ref() = Some(&"Título inválido")
//   → .map(|m| m.to_string()) = Some("Título inválido")
//   → filter_map mantém = "Título inválido"

["Título inválido", "Mín 4 chars"]

// 2c. .collect() em Vec<String>
messages = ["Título inválido", "Mín 4 chars"]

// 2d. messages.join(", ")
"Título inválido, Mín 4 chars"

// 2e. format!("{}: {}", field, ...)
"title: Título inválido, Mín 4 chars"
```

Para `("email", [...])`:
```rust
"email: Email inválido"
```

**3. `.collect::<Vec<_>>()`**
```rust
[
    "title: Título inválido, Mín 4 chars",
    "email: Email inválido"
]
```

**4. `.join("; ")`**
```rust
"title: Título inválido, Mín 4 chars; email: Email inválido"
```

---

## Por que `.filter_map()` aqui?

```rust
.filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
```

**Porque `message` é `Option<Cow<'static, str>>`:**

```rust
struct ValidationError {
    message: Option<Cow<'static, str>>,  // pode ser None!
    // ...
}
```

**Se `message` for `None`:**
- `e.message.as_ref()` = `None`
- `.map(...)` = `None`
- `filter_map` descarta `None`

**Se `message` for `Some("texto")`:**
- `e.message.as_ref()` = `Some(&"texto")`
- `.map(|m| m.to_string())` = `Some("texto".to_string())`
- `filter_map` mantém `"texto"`

**Resultado:** Só mensagens não-vazias aparecem no output.

---

## Outros Métodos Úteis

### `.join()` - Juntar Strings

```rust
let parts = vec!["um", "dois", "três"];
let result = parts.join(", ");
// "um, dois, três"
```

**Comparação PHP:**
```php
implode(", ", ["um", "dois", "três"]);
// "um, dois, três"
```

---

### `.fold()` - Reduzir a Valor Único

```rust
let numbers = vec![1, 2, 3, 4];
let sum = numbers.iter().fold(0, |acc, n| acc + n);
// sum = 10
```

**Comparação PHP:**
```php
array_reduce([1, 2, 3, 4], fn($acc, $n) => $acc + $n, 0);
// 10
```

**Comparação JavaScript:**
```javascript
[1, 2, 3, 4].reduce((acc, n) => acc + n, 0);
// 10
```

---

### `.find()` - Encontrar Primeiro

```rust
let numbers = vec![1, 2, 3, 4];
let first_even = numbers.iter().find(|n| *n % 2 == 0);
// Some(&2)
```

**Comparação PHP:**
```php
// Não tem find direto, precisa usar array_filter + reset
$first_even = reset(array_filter([1,2,3,4], fn($n) => $n % 2 == 0));
// 2
```

---

## Lazy vs Eager

**Rust iterators são LAZY:**

```rust
let numbers = vec![1, 2, 3];

// NÃO executa ainda:
let doubled = numbers.iter().map(|n| {
    println!("Dobrando {}", n);
    n * 2
});

// Agora sim executa:
let result: Vec<i32> = doubled.collect();
// Imprime:
// Dobrando 1
// Dobrando 2
// Dobrando 3
```

**PHP/JavaScript são EAGER:**

```php
$numbers = [1, 2, 3];
$doubled = array_map(function($n) {
    echo "Dobrando $n\n";  // executa IMEDIATAMENTE
    return $n * 2;
}, $numbers);
// Imprime imediatamente
```

**Vantagem Lazy:**
- Só executa quando necessário
- Pode parar cedo (`.take()`, `.find()`)
- Mais eficiente

---

## Encadeamento (Chaining)

**Rust ama encadear operações:**

```rust
let result: Vec<String> = vec!["1", "2", "abc", "3"]
    .iter()                              // iterador
    .filter_map(|s| s.parse::<i32>().ok())  // parseia
    .filter(|n| n % 2 == 0)              // só pares
    .map(|n| format!("Número: {}", n))   // formata
    .collect();                          // materializa

// ["Número: 2"]
```

**Equivalente PHP (verboso):**
```php
$input = ["1", "2", "abc", "3"];
$parsed = array_filter(array_map(fn($s) => is_numeric($s) ? (int)$s : null, $input));
$evens = array_filter($parsed, fn($n) => $n % 2 == 0);
$result = array_map(fn($n) => "Número: $n", $evens);
// ["Número: 2"]
```

**Rust: 6 linhas, legível**  
**PHP: 4 linhas, confuso**

---

## Exercícios

### Exercício 1: Dobrar números pares

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// Dobrar só os pares:
let result: Vec<i32> = numbers
    .iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * 2)
    .collect();

// result = [4, 8, 12]
```

---

### Exercício 2: Extrair nomes de erros

```rust
struct Error {
    code: i32,
    message: Option<String>,
}

let errors = vec![
    Error { code: 404, message: Some("Not found".into()) },
    Error { code: 500, message: None },
    Error { code: 400, message: Some("Bad request".into()) },
];

let messages: Vec<String> = errors
    .iter()
    .filter_map(|e| e.message.as_ref().cloned())
    .collect();

// messages = ["Not found", "Bad request"]
```

---

### Exercício 3: Somar valores de HashMap

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 10);
scores.insert("Bob", 20);
scores.insert("Carol", 30);

let total: i32 = scores.values().sum();
// total = 60
```

---

## Resumo

**Operações principais:**
- `.iter()` → cria iterador
- `.map()` → transforma cada item
- `.filter()` → mantém items que passam teste
- `.filter_map()` → transforma + filtra em um
- `.collect()` → materializa resultado
- `.join()` → junta strings

**Pipeline típico:**
```rust
dados
    .iter()           // iterador
    .filter(...)      // filtra
    .map(...)         // transforma
    .collect()        // materializa
```

**Comparação:**
- **Rust:** Lazy, eficiente, type-safe
- **PHP:** Eager, verboso, runtime errors
- **JavaScript:** Eager, clean syntax, runtime errors

**Rust iteradores = mais código no início, menos bugs depois.** ✅