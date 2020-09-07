CARGO ?= cargo

all: build ## Run the 'build' target

.PHONY: build
build: ## Build the main binary
	$(CARGO) build

.PHONY: build-release
build-release: ## Build the main binary in release mode
	$(CARGO) build --release

.PHONY: clean
clean: ## Clean the work tree
	$(CARGO) clean

.PHONY: doc
doc: ## Build the documentation
	$(CARGO) doc --no-deps

.PHONY: lint ## Run all linters
lint: lint-clippy lint-rustfmt

.PHONY: lint-clippy
lint-clippy: ## Run the clippy linter
	$(CARGO) clippy --all -- -D warnings

.PHONY: lint-rustfmt
lint-rustfmt: ## Run the rustfmt linter
	$(CARGO) fmt && git diff --exit-code

.PHONY: run
run: ## Run the main binary
	$(CARGO) run

define test
	$(CARGO) test \
		--test $(1) $(ARGS) \
		-- \
		--test-threads 1 \
		--nocapture
endef

.PHONY: test-integration
test-integration: ## Run the integration tests
	$(call test,integration)

.PHONY: test-e2e
test-e2e: ## Run the e2e tests
	$(call test,e2e)

.PHONY: test-unit
test-unit: ## Run the unit tests
	$(CARGO) test --lib

.PHONY: help
help: ## Display this help
	@awk \
		-v "col=${COLOR}" -v "nocol=${NOCOLOR}" \
		' \
			BEGIN { \
				FS = ":.*##" ; \
				printf "Available targets:\n"; \
			} \
			/^[a-zA-Z0-9_-]+:.*?##/ { \
				printf "  %s%-25s%s %s\n", col, $$1, nocol, $$2 \
			} \
			/^##@/ { \
				printf "\n%s%s%s\n", col, substr($$0, 5), nocol \
			} \
		' $(MAKEFILE_LIST)
