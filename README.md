# LLM Moderation API

REST API для модерации контента с использованием LLM. Анализирует сообщения на наличие опасного контента и возвращает оценку риска.

## Категории модерации

- **Радикальные позиции** — ненависть, дискриминация, призывы к насилию против рас, наций, религий
- **Рекламный спам** — нежелательная реклама, скам, коммерческий контент
- **Доксинг** — раскрытие персональных данных без согласия

## Требования

- Rust 1.75+
- PostgreSQL 14+
- vLLM сервер с OpenAI-compatible API

## Конфигурация

Переменные окружения:

| Переменная | Описание | По умолчанию |
|------------|----------|--------------|
| `DATABASE_URL` | PostgreSQL connection string | — |
| `API_KEY` | Ключ для аутентификации запросов | — |
| `VLLM_BASE_URL` | URL vLLM сервера | `http://localhost:8000` |
| `VLLM_MODEL` | Название модели | `GPT-OSS:20b` |
| `DANGER_THRESHOLD` | Порог для требования ревью (%) | `65.0` |
| `PORT` | Порт сервера | `8000` |

## Запуск

### Docker Compose

```bash
cp .env.example .env
# Отредактируйте .env

docker compose up -d
```

### Локально

```bash
export DATABASE_URL="postgres://user:pass@localhost/llm_moderation"
export API_KEY="your-secret-key"
export VLLM_BASE_URL="http://localhost:8000"

cargo run --release
```

## API

Все защищённые эндпоинты требуют заголовок `X-API-Key`.

### POST /api/v1/moderate

Анализ сообщения.

```bash
curl -X POST http://localhost:8000/api/v1/moderate \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{"message": "текст для проверки"}'
```

Ответ:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "danger_score": 75.5,
  "categories": ["radical_positions"],
  "requires_review": true
}
```

### POST /api/v1/moderate/{id}/reason

Получение объяснения для сообщений с высоким danger_score (выше порога).

```bash
curl -X POST http://localhost:8000/api/v1/moderate/{id}/reason \
  -H "X-API-Key: your-key"
```

Ответ:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "danger_score": 75.5,
  "reason": "Сообщение содержит дискриминационные высказывания...",
  "categories": ["radical_positions"]
}
```

### GET /health

Проверка состояния сервиса (без аутентификации).

## Документация API

Swagger UI доступен по адресу `/swagger-ui/`.

## Лицензия

MIT
