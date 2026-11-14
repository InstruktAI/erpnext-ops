.PHONY: install format lint test build

# The plugin is validated with Anthropic's own checker, which ships with Claude Code.
install:
	@command -v claude >/dev/null 2>&1 || npm install --global @anthropic-ai/claude-code

format:
	uvx ruff format tools

lint:
	uvx ruff check tools
	uvx ruff format --check tools

build:
	python3 tools/build_skill.py

# Builds the plugin from the documents and validates the plugin and the marketplace file.
test: build
	claude plugin validate dist/erp
	claude plugin validate .
