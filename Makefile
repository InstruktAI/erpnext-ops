.PHONY: install format lint build test

install:
	uv run --frozen python -m tools.tasks install

format:
	uv run --frozen python -m tools.tasks format

lint:
	uv run --frozen python -m tools.tasks lint

build:
	uv run --frozen python -m tools.tasks build

test:
	uv run --frozen python -m tools.tasks test
