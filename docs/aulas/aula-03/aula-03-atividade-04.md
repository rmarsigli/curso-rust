# Atividade 04 - Teste do `CRUD` completo

Criei um postman com todas as chamadas, o `json` dele está aqui:

```json
{
	"info": {
		"_postman_id": "913fec85-d68c-4987-8ea1-9b2fadb7af0c",
		"name": "Curso Rust",
		"schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json",
		"_exporter_id": "3055717"
	},
	"item": [
		{
			"name": "list_tasks()",
			"request": {
				"method": "GET",
				"header": [],
				"url": {
					"raw": "{{baseURL}}/tasks",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks"
					]
				}
			},
			"response": []
		},
		{
			"name": "create_task()",
			"request": {
				"method": "POST",
				"header": [
					{
						"key": "Content-Type",
						"value": "application/json",
						"type": "text"
					}
				],
				"body": {
					"mode": "formdata",
					"formdata": [
						{
							"key": "title",
							"value": "Task de Teste",
							"type": "text"
						}
					]
				},
				"url": {
					"raw": "{{baseURL}}/tasks",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks"
					]
				}
			},
			"response": []
		},
		{
			"name": "update_task()",
			"request": {
				"method": "PATCH",
				"header": [],
				"body": {
					"mode": "raw",
					"raw": "{\r\n    \"title\": \"Esta task foi Atualizada via Postman\",\r\n    \"done\": false\r\n}",
					"options": {
						"raw": {
							"language": "json"
						}
					}
				},
				"url": {
					"raw": "{{baseURL}}/tasks/1",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks",
						"1"
					]
				}
			},
			"response": []
		},
		{
			"name": "delete_task()",
			"request": {
				"method": "DELETE",
				"header": [],
				"url": {
					"raw": "{{baseURL}}/tasks/1",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks",
						"1"
					]
				}
			},
			"response": []
		},
		{
			"name": "get_task()",
			"request": {
				"method": "GET",
				"header": [],
				"url": {
					"raw": "{{baseURL}}/tasks/1",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks",
						"1"
					]
				}
			},
			"response": []
		}
	],
	"variable": [
		{
			"key": "baseURL",
			"value": "",
			"type": "default"
		}
	]
}
```

## Retornos

### `create_task()`

```json
{
    "id": 1,
    "title": "Criado via Postman",
    "done": false
}
```

### `update_task()`

```json
{
    "id": 1,
    "title": "Esta task foi Atualizada via Postman",
    "done": false
}
```

### `list_tasks()`

```json
[
    {
        "id": 1,
        "title": "Esta task foi Atualizada via Postman",
        "done": false
    },
    {
        "id": 2,
        "title": "Criado via Postman",
        "done": false
    },
    {
        "id": 3,
        "title": "Criado via Postman",
        "done": false
    },
    {
        "id": 4,
        "title": "Criado via Postman",
        "done": false
    },
    {
        "id": 5,
        "title": "Criado via Postman",
        "done": false
    }
]
```

### `delete_task()`

```json
"Task 'Criado via Postman' deletada!"
```

### `get_task()`

```json
{
    "id": 1,
    "title": "Esta task foi Atualizada via Postman",
    "done": false
}
```
# Rust - Aula 03 - Atividade 04

# Atividade 04 - Teste do `CRUD` completo

Criei um postman com todas as chamadas, o `json` dele está aqui:

```json
{
	"info": {
		"_postman_id": "913fec85-d68c-4987-8ea1-9b2fadb7af0c",
		"name": "Curso Rust",
		"schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json",
		"_exporter_id": "3055717"
	},
	"item": [
		{
			"name": "list_tasks()",
			"request": {
				"method": "GET",
				"header": [],
				"url": {
					"raw": "{{baseURL}}/tasks",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks"
					]
				}
			},
			"response": []
		},
		{
			"name": "create_task()",
			"request": {
				"method": "POST",
				"header": [
					{
						"key": "Content-Type",
						"value": "application/json",
						"type": "text"
					}
				],
				"body": {
					"mode": "formdata",
					"formdata": [
						{
							"key": "title",
							"value": "Task de Teste",
							"type": "text"
						}
					]
				},
				"url": {
					"raw": "{{baseURL}}/tasks",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks"
					]
				}
			},
			"response": []
		},
		{
			"name": "update_task()",
			"request": {
				"method": "PATCH",
				"header": [],
				"body": {
					"mode": "raw",
					"raw": "{\r\n    \"title\": \"Esta task foi Atualizada via Postman\",\r\n    \"done\": false\r\n}",
					"options": {
						"raw": {
							"language": "json"
						}
					}
				},
				"url": {
					"raw": "{{baseURL}}/tasks/1",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks",
						"1"
					]
				}
			},
			"response": []
		},
		{
			"name": "delete_task()",
			"request": {
				"method": "DELETE",
				"header": [],
				"url": {
					"raw": "{{baseURL}}/tasks/1",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks",
						"1"
					]
				}
			},
			"response": []
		},
		{
			"name": "get_task()",
			"request": {
				"method": "GET",
				"header": [],
				"url": {
					"raw": "{{baseURL}}/tasks/1",
					"host": [
						"{{baseURL}}"
					],
					"path": [
						"tasks",
						"1"
					]
				}
			},
			"response": []
		}
	],
	"variable": [
		{
			"key": "baseURL",
			"value": "",
			"type": "default"
		}
	]
}
```

## Retornos

### `create_task()`

```json
{
    "id": 1,
    "title": "Criado via Postman",
    "done": false
}
```

### `update_task()`

```json
{
    "id": 1,
    "title": "Esta task foi Atualizada via Postman",
    "done": false
}
```

### `list_tasks()`

```json
[
    {
        "id": 1,
        "title": "Esta task foi Atualizada via Postman",
        "done": false
    },
    {
        "id": 2,
        "title": "Criado via Postman",
        "done": false
    },
    {
        "id": 3,
        "title": "Criado via Postman",
        "done": false
    },
    {
        "id": 4,
        "title": "Criado via Postman",
        "done": false
    },
    {
        "id": 5,
        "title": "Criado via Postman",
        "done": false
    }
]
```

### `delete_task()`

```json
"Task 'Criado via Postman' deletada!"
```

### `get_task()`

```json
{
    "id": 1,
    "title": "Esta task foi Atualizada via Postman",
    "done": false
}
```
