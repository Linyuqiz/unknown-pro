PROJECT_ROOT := $(shell pwd)

.PHONY: server

postgres:
	@docker run -d --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=postgres-password \
	-v ${PROJECT_ROOT}/server/src/config/app.sql:/docker-entrypoint-initdb.d/init.sql postgres:latest

generate-entity:
	@sea-orm-cli generate entity -v --with-serde=both -o=server/src/entity \
	--max-connections=10 --ignore-tables=migration --seaography \
	-u=postgres://unknown:unknown-password@postgres.orb.local:5432/unknown -s=public

server:
	@lsof -t -i :32432 | xargs kill -9 && cd server && cargo run

desktop:
	@cd unknown && yarn && yarn tauri dev
