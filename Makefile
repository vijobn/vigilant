# Specify the desired Node.js version
NODE_VERSION := 18.16.0  # Change this to your desired version
TAURI_APP_NAME := vigilant4
# Define variables
#TARGET = target/debug/c5app
CARGO = cargo

# Check if running inside Docker
IS_DOCKER := $(shell [ -f /.dockerenv ] && echo "yes" || echo "no")

# Check if cargo is installed
HAS_CARGO := $(shell command -v $(CARGO) >/dev/null 2>&1 && echo "yes" || echo "no")

# Ensure cargo is available
ifeq ($(HAS_CARGO),no)
$(warning "Error: cargo is not available. Please install Rust and Cargo.")
endif

# Ensure running inside Docker
ifeq ($(IS_DOCKER),no)
$(warning "Warning: Not running inside Docker. Make sure this is intentional.")
endif

# Print help
help:
	@echo "Vigilant app Make options"
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  buildsh   - Shell to build"
	@echo "  build     - Build the project"
	@echo "  run       - Run the executable"
	@echo "  test      - Run tests"
	@echo "  clean     - Clean the build artifacts"
	@echo "  format    - Format the code with rustfmt"
	@echo "  lint      - Lint the code with clippy"
	@echo "  doc       - Generate and open documentation"
	@echo "  help      - Show this help message"

.PHONY: all install_nvm install_node clean

all: install_nvm install_node

.PHONY: buildsh
buildsh:
ifeq ($(IS_DOCKER),yes)
	$(error "Already in docker container")
endif
	(bash ./dev_container.sh)

# Install NVM
install_nvm:
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif
	@echo "Installing NVM (Node Version Manager)..."
	curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
	@echo "NVM installed. Please restart your terminal or run the following command:"
	@echo "export NVM_DIR=\"$$(HOME)/.nvm\" && [ -s \"$$(nvm --version)\" ] && . \"$$(nvm --version)\"/nvm.sh"

# Install Node.js using NVM
install_node: install_nvm
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif
	@echo "Installing Node.js version $(NODE_VERSION)..."
	@bash -c "source $$HOME/.nvm/nvm.sh && nvm install $(NODE_VERSION) && nvm use $(NODE_VERSION) && nvm alias default $(NODE_VERSION)"
	@echo "Node.js installed successfully."

# Create a new Tauri app
create_tauri_app: install_node
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif
	#@echo "Creating a new Tauri app named $(TAURI_APP_NAME)..."
	#npx create-react-app $(TAURI_APP_NAME) && cd $(TAURI_APP_NAME) && tauri init
	#@echo "Tauri app $(TAURI_APP_NAME) created successfully."

preinstall: install_node install_nvm 
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif

build:
	(cd $(TAURI_APP_NAME)/src-tauri && cargo build)
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif


/usr/local/bin/pm2:
	npm install -g pm2

build: /usr/local/bin/pm2
	(cd $(TAURI_APP_NAME)/src-tauri && cargo build)
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif

PROCESS_NAME = vigilant-react
NPM_CMD = npm start

$(TAURI_APP_NAME)/package-lock.json $(TAURI_APP_NAME)/node_modules:
	(cd $(TAURI_APP_NAME) && npm install)

start: build $(TAURI_APP_NAME)/node_modules
	@if pm2 list | grep -q "$(PROCESS_NAME)"; then \
		echo "$(PROCESS_NAME) is running. Restarting..."; \
		(cd $(TAURI_APP_NAME) && pm2 restart "$(PROCESS_NAME)"); \
	else \
		echo "$(PROCESS_NAME) is not running. Starting..."; \
		(cd $(TAURI_APP_NAME) && pm2 start "$(NPM_CMD)" --name "$(PROCESS_NAME)"); \
	fi

.PHONY: ui
ui:
ifeq ($(IS_DOCKER),yes)
	(cd $(TAURI_APP_NAME)/ && ./src-tauri/target/debug/vigilant ls)
else
	(cd $(TAURI_APP_NAME)/ && ./src-tauri/target/debug/vigilant ls)
endif

.PHONY: stop
stop:
	pm2 stop $(PROCESS_NAME) && pm2 delete $(PROCESS_NAME)

clean:
	@echo "Cleaning up..."
	pm2 stop $(PROCESS_NAME) && pm2 delete $(PROCESS_NAME)
	rm -rf node_modules package.json package-lock.json
	(cd $(TAURI_APP_NAME) && rm -rf node_modules package-lock.json)
	@echo "Cleanup completed."
