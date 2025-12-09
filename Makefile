.PHONY: help build build-release build-debug build-all clean test validate-release publish-release

# Variables
CARGO_BIN := cargo
POWERSHELL := pwsh -NoProfile -ExecutionPolicy Bypass
OUTPUT_DIR ?= release-builds
VERSION ?= $(shell grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)

# Colors
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
CYAN := \033[0;36m
MAGENTA := \033[0;35m
NC := \033[0m

.DEFAULT_GOAL := help

help: ## Mostrar esta ayuda
	@echo "$(MAGENTA)╔═══════════════════════════════════════════════════════╗$(NC)"
	@echo "$(MAGENTA)║         NVM-RS Build & Release Makefile                ║$(NC)"
	@echo "$(MAGENTA)╚═══════════════════════════════════════════════════════╝$(NC)"
	@echo ""
	@echo "$(CYAN)Comandos disponibles:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'
	@echo ""
	@echo "$(CYAN)Variables:$(NC)"
	@echo "  $(YELLOW)OUTPUT_DIR$(NC)  Directorio de salida (default: $(OUTPUT_DIR))"
	@echo "  $(YELLOW)VERSION$(NC)     Versión de build (default: $(VERSION))"
	@echo ""
	@echo "$(CYAN)Ejemplos:$(NC)"
	@echo "  $(GREEN)make build$(NC)                    # Build release por defecto"
	@echo "  $(GREEN)make build-all$(NC)               # Build todos los targets"
	@echo "  $(GREEN)make validate-release$(NC)        # Validar artifacts"
	@echo "  $(GREEN)make publish-release VERSION=v0.5.0$(NC)  # Publicar release"

## Build Commands

build: build-release ## Compilar release (default)

build-release: ## Compilar release
	@echo "$(CYAN)Compilando release...$(NC)"
	$(CARGO_BIN) build --release
	@echo "$(GREEN)✓ Build completado$(NC)"

build-debug: ## Compilar debug
	@echo "$(CYAN)Compilando debug...$(NC)"
	$(CARGO_BIN) build
	@echo "$(GREEN)✓ Build completado$(NC)"

build-all: ## Compilar todos los targets (requiere PowerShell)
	@echo "$(CYAN)Compilando todos los targets...$(NC)"
	@if command -v pwsh >/dev/null 2>&1; then \
		$(POWERSHELL) -Command "& './scripts/build/build-releases.ps1' -OutputDir '$(OUTPUT_DIR)'"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

build-windows: ## Compilar solo targets de Windows
	@echo "$(CYAN)Compilando targets de Windows...$(NC)"
	@if command -v pwsh >/dev/null 2>&1; then \
		$(POWERSHELL) -Command "& './scripts/build/build-releases.ps1' -OutputDir '$(OUTPUT_DIR)'"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

build-linux: ## Compilar targets de Linux
	@echo "$(CYAN)Compilando targets de Linux...$(NC)"
	$(CARGO_BIN) build --release --target x86_64-unknown-linux-gnu
	@echo "$(GREEN)✓ Build completado: Linux x64$(NC)"

build-macos: ## Compilar targets de macOS
	@echo "$(CYAN)Compilando targets de macOS...$(NC)"
	$(CARGO_BIN) build --release --target x86_64-apple-darwin
	@echo "$(GREEN)✓ Build completado: macOS x64$(NC)"

## Testing & Validation

test: ## Ejecutar tests
	@echo "$(CYAN)Ejecutando tests...$(NC)"
	$(CARGO_BIN) test --release
	@echo "$(GREEN)✓ Tests completados$(NC)"

test-verbose: ## Ejecutar tests con output detallado
	@echo "$(CYAN)Ejecutando tests (verbose)...$(NC)"
	$(CARGO_BIN) test --release -- --nocapture
	@echo "$(GREEN)✓ Tests completados$(NC)"

validate-release: ## Validar artifacts del release
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "$(CYAN)Validando release...$(NC)"; \
		$(POWERSHELL) -Command "& './scripts/release/validate-release.ps1' -ReleaseDir '$(OUTPUT_DIR)'"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

validate-strict: ## Validar release en modo estricto
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "$(CYAN)Validando release (strict)...$(NC)"; \
		$(POWERSHELL) -Command "& './scripts/release/validate-release.ps1' -ReleaseDir '$(OUTPUT_DIR)' -Strict"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

## Release Management

release: build-all validate-strict ## Build + Validar + Publicar
	@echo "$(GREEN)✓ Release listo para publicar$(NC)"
	@echo "Ejecuta: make publish-release"

publish-release: ## Publicar release en GitHub
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "$(CYAN)Publicando release $(VERSION)...$(NC)"; \
		$(POWERSHELL) -Command "& './scripts/release/publish-release.ps1' -Version v$(VERSION)"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

publish-draft: ## Publicar como draft
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "$(CYAN)Publicando draft release $(VERSION)...$(NC)"; \
		$(POWERSHELL) -Command "& './scripts/release/publish-release.ps1' -Version v$(VERSION) -Draft"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

publish-prerelease: ## Publicar como pre-release
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "$(CYAN)Publicando pre-release $(VERSION)...$(NC)"; \
		$(POWERSHELL) -Command "& './scripts/release/publish-release.ps1' -Version v$(VERSION) -PreRelease"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

tag-only: ## Crear solo Git tag (sin publicar)
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "$(CYAN)Creando Git tag...$(NC)"; \
		$(POWERSHELL) -Command "& './scripts/release/publish-release.ps1' -Version v$(VERSION) -TagOnly"; \
	else \
		echo "$(RED)Error: PowerShell no encontrado$(NC)"; \
		exit 1; \
	fi

## Maintenance

clean: ## Limpiar build cache
	@echo "$(CYAN)Limpiando...$(NC)"
	$(CARGO_BIN) clean
	@echo "$(GREEN)✓ Limpieza completada$(NC)"

clean-release: ## Limpiar artifacts de release
	@echo "$(CYAN)Limpiando release artifacts...$(NC)"
	rm -rf $(OUTPUT_DIR)
	@echo "$(GREEN)✓ Artifacts eliminados$(NC)"

format: ## Formatear código
	@echo "$(CYAN)Formateando código...$(NC)"
	$(CARGO_BIN) fmt
	@echo "$(GREEN)✓ Código formateado$(NC)"

lint: ## Ejecutar clippy
	@echo "$(CYAN)Ejecutando clippy...$(NC)"
	$(CARGO_BIN) clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✓ Análisis completado$(NC)"

check: format lint test ## Format + Lint + Test
	@echo "$(GREEN)✓ Todas las verificaciones pasadas$(NC)"

## Information

info: ## Mostrar información del proyecto
	@echo ""
	@echo "$(MAGENTA)Project Information:$(NC)"
	@echo "  $(CYAN)Version:$(NC) $(VERSION)"
	@echo "  $(CYAN)Cargo:$(NC) $(CARGO_BIN)"
	@echo "  $(CYAN)Output Dir:$(NC) $(OUTPUT_DIR)"
	@echo ""
	@echo "$(MAGENTA)Build Targets:$(NC)"
	@cargo --version
	@rustc --version
	@echo ""
	@echo "$(MAGENTA)Installed Targets:$(NC)"
	@rustup target list --installed | sed 's/^/  /'
	@echo ""

deps: ## Mostrar dependencias
	@echo "$(CYAN)Dependencias del proyecto:$(NC)"
	@$(CARGO_BIN) tree
	@echo ""

## Utilities

install-targets: ## Instalar targets comunes para cross-compilation
	@echo "$(CYAN)Instalando targets de Rust...$(NC)"
	rustup target add x86_64-pc-windows-msvc
	rustup target add aarch64-pc-windows-msvc
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-linux-musl
	rustup target add aarch64-unknown-linux-gnu
	rustup target add aarch64-unknown-linux-musl
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	@echo "$(GREEN)✓ Targets instalados$(NC)"

setup: install-targets ## Setup inicial (instalar targets)
	@echo "$(GREEN)✓ Setup completado$(NC)"

.PHONY: all
all: clean check build-all validate-release ## Complete workflow

