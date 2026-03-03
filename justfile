set shell := ["powershell.exe"]

build:
    bun run tauri build

run:
    bun run tauri dev

[working-directory("server")]
server:
    cargo watch -x run

typegen:
    bun x openapi-typescript http://localhost:3000/api-docs/openapi.json -o ./src/bindings/types.ts

swagger:
    cmd /c start http://localhost:3000/swagger-ui/

db-reset:
    docker compose exec db psql -U user -d chewback -c "TRUNCATE TABLE sessions, users CASCADE;"