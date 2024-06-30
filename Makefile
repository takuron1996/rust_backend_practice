SHELL = /bin/bash
CONTAINER_NAME = app
APPLICATION = application
DOCKER = docker exec $(CONTAINER_NAME)

up:
	docker compose up -d

build:
	docker compose build

down:
	docker compose down

lint: format
	$(DOCKER) cargo clippy

format:
	$(DOCKER) cargo fmt

test: format
	$(DOCKER) cargo test

fix:
	$(DOCKER) cargo audit