# Validação de Tipos Primitivos

> **Conceito:** Quando validar tipos primitivos (bool, números) e quando confiar em Serde  
> **Nível:** Iniciante/Intermediário  
> **Relacionado:** Aula 05 (Validação - bool, números, strings)

## O Problema

Você tem esta struct:

```rust
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}
```

**Pergunta:** Preciso validar `done` (bool) e `id` (u32)?

**Resposta curta:** **Geralmente NÃO.**

---

## Como Funciona a Validação

### Duas Camadas de Validação

**1. Validação de TIPO (Serde) - automática**
```
JSON → Serde → Struct
```

**2. Validação de REGRAS (Validator) - manual**
```
Struct → Validator → Lógica de negócio
```

---

## Camada 1: Serde (Tipo)

### O que Serde valida automaticamente:

```rust
#[derive(Deserialize)]
pub struct Task {
    pub id: u32,        // Serde valida: é número? cabe em u32?
    pub title: String,  // Serde valida: é string?
    pub done: bool,     // Serde valida: é bool (true/false)?
}
```

### Testes com bool:

**JSON válido:**
```json
{"done": true}   // ✅ OK
{"done": false}  // ✅ OK
```

**JSON inválido (Serde rejeita):**
```json
{"done": "true"}      // ❌ Erro: expected bool, got string
{"done": 1}           // ❌ Erro: expected bool, got number
{"done": "sim"}       // ❌ Erro: expected bool, got string
{"done": null}        // ❌ Erro: expected bool, got null
```

**Resultado:** HTTP 400 ou 422 (antes de chegar no handler)

### Testes com números:

**JSON válido:**
```json
{"id": 42}          // ✅ OK
{"id": 0}           // ✅ OK
{"id": 4294967295}  // ✅ OK (max u32)
```

**JSON inválido (Serde rejeita):**
```json
{"id": -1}          // ❌ Erro: u32 não aceita negativo
{"id": 4294967296}  // ❌ Erro: overflow u32
{"id": "42"}        // ❌ Erro: expected number, got string
{"id": 3.14}        // ❌ Erro: expected integer, got float
{"id": null}        // ❌ Erro: expected number, got null
```

### Testes com strings:

**JSON válido:**
```json
{"title": ""}              // ✅ Serde aceita (validação vem depois)
{"title": "Hello"}         // ✅ OK
{"title": "a".repeat(1000)} // ✅ Serde aceita (validação vem depois)
```

**JSON inválido (Serde rejeita):**
```json
{"title": 123}      // ❌ Erro: expected string, got number
{"title": true}     // ❌ Erro: expected string, got bool
{"title": null}     // ❌ Erro: expected string, got null
```

---

## Camada 2: Validator (Regras de Negócio)

**Aqui você valida REGRAS, não TIPOS.**

### String - Precisa validar?

**SIM, quase sempre:**

```rust
#[derive(Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: String,  // Serde aceita "", mas negócio não
}
```

**Por quê:**
- Serde aceita string vazia `""`
- Serde aceita string gigante (10MB)
- **Você** precisa definir limites de negócio

**Validações comuns:**
- `length(min, max)` - tamanho
- `email` - formato de email
- `url` - formato de URL
- `regex(pattern = "...")` - padrão custom

---

### Bool - Precisa validar?

**99% das vezes: NÃO.**

```rust
pub struct Task {
    pub done: bool,  // ✅ Serde já valida
}
```

**Por quê:**
- Bool só pode ser `true` ou `false`
- Não tem "valor inválido"
- Serde já garante tipo correto

**Quando validar bool:**

**Caso 1: Forçar valor específico**
```rust
#[derive(Deserialize, Validate)]
pub struct AcceptTerms {
    #[validate(custom(function = "must_be_true"))]
    pub accept: bool,  // DEVE ser true
}

fn must_be_true(value: &bool) -> Result<(), ValidationError> {
    if !*value {
        return Err(ValidationError::new("must_accept_terms"));
    }
    Ok(())
}
```

**Caso 2: Validação condicional**
```rust
#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_task"))]
pub struct UpdateTask {
    pub title: String,
    pub done: bool,
}

fn validate_task(task: &UpdateTask) -> Result<(), ValidationError> {
    // Não pode marcar como "done" se título está vazio
    if task.done && task.title.is_empty() {
        return Err(ValidationError::new("cannot_complete_without_title"));
    }
    Ok(())
}
```

**Caso 3: Lógica de negócio complexa**
```rust
pub struct PaymentSettings {
    pub auto_renew: bool,
    pub send_invoice: bool,
}

// Regra: se auto_renew = true, DEVE enviar invoice
fn validate_payment(settings: &PaymentSettings) -> Result<(), ValidationError> {
    if settings.auto_renew && !settings.send_invoice {
        return Err(ValidationError::new("auto_renew_requires_invoice"));
    }
    Ok(())
}
```

**Fora desses casos: não valide bool.** ✅

---

### Números - Precisa validar?

**Depende:**

**u32, i32, etc (tipos fixos):**

```rust
pub struct Task {
    pub id: u32,  // ✅ Serde já valida (0 a 4,294,967,295)
}
```

**Serde garante:**
- Não aceita negativo em `u32`
- Não aceita overflow
- Não aceita float quando espera int

**Quando validar números:**

**Caso 1: Range de negócio**
```rust
#[derive(Deserialize, Validate)]
pub struct Product {
    #[validate(range(min = 1, max = 1000))]
    pub price: u32,  // preço entre R$1 e R$1000
    
    #[validate(range(min = 0, max = 100))]
    pub discount: u8,  // desconto 0-100%
}
```

**Caso 2: Validação customizada**
```rust
#[derive(Deserialize, Validate)]
pub struct User {
    #[validate(custom(function = "validate_age"))]
    pub age: u8,
}

fn validate_age(age: &u8) -> Result<(), ValidationError> {
    if *age < 18 {
        return Err(ValidationError::new("must_be_adult"));
    }
    Ok(())
}
```

**Caso 3: Múltiplos em 5**
```rust
fn validate_multiple_of_5(value: &u32) -> Result<(), ValidationError> {
    if value % 5 != 0 {
        return Err(ValidationError::new("must_be_multiple_of_5"));
    }
    Ok(())
}
```

---

## Comparação: PHP/Laravel vs Rust

### Laravel (tudo em runtime):

```php
$request->validate([
    'title' => 'required|string|min:1|max:100',  // valida tipo E regras
    'done' => 'required|boolean',                 // valida tipo E bool
    'price' => 'required|integer|min:1|max:1000', // valida tipo E range
]);
```

**Laravel valida TUDO em runtime:**
- Tipo (string, bool, int)
- Regras de negócio (min, max)

**Se esquecer `boolean`, aceita qualquer coisa:**
```php
// Sem validação:
$done = $request->done;  // pode ser "sim", 1, "true", qualquer coisa ❌
```

### Rust (tipo em compile-time, regras em runtime):

```rust
#[derive(Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: String,  // tipo garantido por Serde (compile-time)
    
    pub done: bool,  // tipo garantido por Serde (compile-time)
    
    #[validate(range(min = 1, max = 1000))]
    pub price: u32,  // tipo garantido por Serde (compile-time)
}
```

**Rust separa responsabilidades:**
- **Serde:** valida TIPO (automático)
- **Validator:** valida REGRAS (você decide)

**Vantagens:**
- ✅ Tipo garantido em compile-time (não pode esquecer)
- ✅ Validação de regras explícita (menos bugs)
- ✅ Type-safe (compilador ajuda)

---

## Option<T> - Campos Opcionais

### Quando campo pode ser nulo:

```rust
#[derive(Deserialize, Validate)]
pub struct User {
    pub name: String,           // obrigatório
    pub email: Option<String>,  // opcional (pode ser null)
}
```

**JSON válido:**
```json
{"name": "Alice", "email": "alice@example.com"}  // ✅
{"name": "Bob", "email": null}                   // ✅
{"name": "Carol"}                                // ✅ (email ausente)
```

**JSON inválido:**
```json
{"email": "alice@example.com"}  // ❌ name obrigatório
```

### Validar Option:

```rust
#[derive(Deserialize, Validate)]
pub struct User {
    #[validate(length(min = 1))]
    pub name: String,
    
    #[validate(email)]
    pub email: Option<String>,  // se presente, valida email
}
```

**Validator valida Option automaticamente:**
- Se `None` → não valida (ok)
- Se `Some(value)` → valida value

---

## Regras de Quando Validar

### ✅ SEMPRE valide:

**String:**
- `length(min = X, max = Y)` - evita vazio e gigante
- `email`, `url` - formatos específicos
- `regex(...)` - padrões custom

**Exemplo:**
```rust
#[validate(length(min = 1, max = 100))]
pub title: String,

#[validate(email)]
pub email: String,
```

---

### ⚠️ VALIDE SE NECESSÁRIO:

**Números (u32, i32, etc):**
- Se tem range de negócio → `range(min, max)`
- Se tem regra custom → `custom(function = "...")`

**Exemplo:**
```rust
#[validate(range(min = 0, max = 100))]
pub discount_percent: u8,
```

---

### ❌ RARAMENTE valide:

**Bool:**
- Só se precisa forçar valor (`accept_terms = true`)
- Ou validação condicional (depende de outros campos)

**Exemplo:**
```rust
// Raro:
#[validate(custom(function = "must_be_true"))]
pub accept_terms: bool,
```

---

### ❌ NUNCA valide:

**IDs auto-incrementados:**
```rust
pub id: u32,  // gerado pelo backend, não valida
```

**Timestamps:**
```rust
pub created_at: DateTime<Utc>,  // gerado pelo backend
```

**Campos calculados:**
```rust
pub total: f64,  // calculado, não vem do usuário
```

---

## Tabela de Decisão Rápida

| Tipo | Serde valida? | Precisa Validator? | Quando validar? |
|------|---------------|---------------------|-----------------|
| **String** | Tipo | ✅ SIM | length, email, url, regex |
| **bool** | Tipo | ❌ RARAMENTE | Forçar true, validação condicional |
| **u32/i32** | Tipo + range | ⚠️ SE NECESSÁRIO | Range de negócio |
| **u8** | Tipo + range | ⚠️ SE NECESSÁRIO | 0-100 (porcentagem) |
| **f64** | Tipo | ⚠️ SE NECESSÁRIO | Positivo, range |
| **Option<T>** | Tipo | ⚠️ MESMAS REGRAS | Se presente, valida |
| **ID (gerado)** | - | ❌ NUNCA | Backend gera |

---

## Exemplos Práticos

### Exemplo 1: Cadastro de usuário

```rust
#[derive(Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,  // 3-50 chars
    
    #[validate(email)]
    pub email: String,  // formato de email
    
    #[validate(length(min = 8))]
    pub password: String,  // mínimo 8 chars
    
    #[validate(custom(function = "must_be_true"))]
    pub accept_terms: bool,  // DEVE ser true
    
    #[validate(range(min = 18, max = 120))]
    pub age: u8,  // 18-120 anos
}
```

---

### Exemplo 2: Produto e-commerce

```rust
#[derive(Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    
    #[validate(range(min = 1))]
    pub price_cents: u32,  // mínimo R$0,01
    
    #[validate(range(min = 0, max = 100))]
    pub discount_percent: u8,  // 0-100%
    
    pub active: bool,  // ✅ não valida (true/false já válido)
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,  // opcional, max 1000
}
```

---

### Exemplo 3: Configurações

```rust
#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_settings"))]
pub struct Settings {
    pub auto_save: bool,  // ✅ não valida individual
    pub auto_backup: bool,
    
    #[validate(range(min = 1, max = 60))]
    pub backup_interval_minutes: u32,  // 1-60 min
}

fn validate_settings(settings: &Settings) -> Result<(), ValidationError> {
    // Se auto_backup ativo, precisa ter intervalo
    if settings.auto_backup && settings.backup_interval_minutes < 5 {
        return Err(ValidationError::new("backup_interval_too_short"));
    }
    Ok(())
}
```

---

## Resumo

**Regra de ouro:**
1. **Serde valida TIPO** (automático) ✅
2. **Você valida REGRAS DE NEGÓCIO** (manual) ✅

**Quando validar:**
- **String:** SEMPRE (length, format)
- **Números:** SE tem range/regra
- **Bool:** RARAMENTE (só se forçar valor)
- **IDs gerados:** NUNCA

**Confiança:**
- Confie em Serde para tipos ✅
- Não confie em Serde para regras de negócio ❌

**Comparação:**
- **Laravel:** Valida tudo em runtime (tipo + regras)
- **Rust:** Valida tipo em compile-time (Serde), regras em runtime (Validator)

**Rust = type-safe + explicit = menos bugs** 🦀✅