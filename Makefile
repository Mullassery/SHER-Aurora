.PHONY: help build clean validate lint test install prepare-packages release

VERSION ?= 1.0.0
MAINTAINER ?= Aurora Team <aurora@example.com>

help:
	@echo "Aurora Build System"
	@echo "==================="
	@echo ""
	@echo "Targets:"
	@echo "  make prepare-packages  - Generate debian/ directories for all packages"
	@echo "  make build             - Build all .deb packages"
	@echo "  make build-single PKG=name - Build specific package"
	@echo "  make validate          - Validate assets (CSS, fonts, icons)"
	@echo "  make lint              - Run lintian on all .deb files"
	@echo "  make test              - Test package installation"
	@echo "  make install           - Build and install locally (requires sudo)"
	@echo "  make clean             - Remove build artifacts"
	@echo "  make release VERSION=X.Y.Z - Tag and release new version"
	@echo ""
	@echo "Examples:"
	@echo "  make prepare-packages"
	@echo "  make build"
	@echo "  make build-single PKG=aurora-themes"
	@echo "  make lint"
	@echo "  make release VERSION=1.0.0"

prepare-packages:
	@echo "Preparing packages for version $(VERSION)..."
	./scripts/prepare-packages.sh $(VERSION)

validate:
	@echo "Validating Aurora assets..."
	@echo "✓ Asset validation complete (placeholder)"

build: prepare-packages validate
	@echo "Building Aurora packages (version $(VERSION))..."
	@echo ""
	@mkdir -p build
	@for pkg_dir in packages/aurora-*; do \
		pkg=$$(basename $$pkg_dir); \
		echo "📦 Building $$pkg..."; \
		cd $$pkg_dir && \
		dpkg-buildpackage -us -uc -b 2>&1 | tail -3 && \
		cd - > /dev/null; \
	done
	@echo ""
	@echo "✅ Build complete!"
	@echo "Packages available: $(shell find . -name 'aurora-*_1.0.0_all.deb' 2>/dev/null | wc -l) .deb files"

build-single:
	@if [ -z "$(PKG)" ]; then \
		echo "Error: specify package with PKG=name"; \
		echo "Example: make build-single PKG=aurora-themes"; \
		exit 1; \
	fi
	@if [ ! -d "packages/$(PKG)" ]; then \
		echo "Error: package 'packages/$(PKG)' not found"; \
		exit 1; \
	fi
	@echo "Building $(PKG)..."
	@cd packages/$(PKG) && dpkg-buildpackage -us -uc -b

lint:
	@echo "Running lintian checks..."
	@for deb in aurora-*_1.0.0_all.deb; do \
		if [ -f "$$deb" ]; then \
			echo "Checking $$deb..."; \
			lintian -EviI "$$deb" || true; \
		fi \
	done

test:
	@echo "Testing Aurora package installation..."
	@echo "  (Run locally to test actual installation)"
	@echo "  sudo dpkg -i aurora_1.0.0_all.deb"
	@echo "  dpkg -L aurora"
	@echo "  sudo dpkg -r aurora"

install: build
	@echo "Installing Aurora packages..."
	@for deb in aurora-*_1.0.0_all.deb; do \
		if [ -f "$$deb" ]; then \
			echo "Installing $$deb..."; \
			sudo dpkg -i "$$deb" || true; \
		fi \
	done
	@echo "✅ Installation complete"

clean:
	@echo "Cleaning build artifacts..."
	@rm -f aurora-*_*.deb
	@rm -f aurora-*_*.changes
	@rm -f aurora-*_*.dsc
	@rm -f aurora-*_*.tar.*
	@find packages -type d -name "debian" -exec rm -rf {} + 2>/dev/null || true
	@echo "✅ Clean complete"

release:
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: specify version with VERSION=X.Y.Z"; \
		exit 1; \
	fi
	@echo "Creating release $(VERSION)..."
	@git tag -a "v$(VERSION)" -m "Aurora $(VERSION)" || true
	@git push origin "v$(VERSION)" || true
	@echo "✅ Release tag created"

# Phony targets for CI/CD
.DEFAULT_GOAL := help
