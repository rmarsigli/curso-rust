# Rust - Aula 04 - Atividade 07

# Atividade 07 - Teste

```shellscript
# Compila
cargo build

# Roda
cargo run

# Testa endpoints
curl -X POST http://127.0.0.1:3003/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Módulos funcionando!"}'

curl http://127.0.0.1:3003/tasks
```

Deve funcionar **exatamente** igual antes. Código diferente, comportamento idêntico.
