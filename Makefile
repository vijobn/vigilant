# Specify the desired Node.js version
NODE_VERSION := 18.16.0  # Change this to your desired version
TAURI_APP_NAME := vigilant
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

/usr/local/bin/pnpm:
	npm install -g pnpm

/usr/local/bin/pm2:
	npm install -g pm2

build: /usr/local/bin/pm2
	(cd $(TAURI_APP_NAME)/src-tauri && cargo build)
ifeq ($(IS_DOCKER),no)
	$(error "NOT in docker container")
endif

PROCESS_NAME = vigilant-react
NPM_CMD = pnpm start

$(TAURI_APP_NAME)/package-lock.json $(TAURI_APP_NAME)/node_modules $(TAURI_APP_NAME)/pnpm-lock.yaml:
	(cd $(TAURI_APP_NAME) && pnpm install)

start: $(TAURI_APP_NAME)/node_modules
	#(cd $(TAURI_APP_NAME) && pnpm tauri dev)

.PHONY: devui
devui:
	@echo "Watching for changes..."
	@while true; do \
		make ui || break; \
	    #inotifywait -e modify -e create -e delete -r ./vigilant ./README.md ./Makefile; \
	    #echo "Changes detected, rebuilding..."; \
		sleep 1; \
	done

.PHONY: ui
ui: build start
	@echo "Waiting for TIME_WAIT states on port $(TARGET_PORT) to clear..."
	@while netstat -ant | grep ":$(TARGET_PORT)" | grep -q "TIME_WAIT"; do \
	    echo "TIME_WAIT exists for port $(TARGET_PORT), retrying in 2 seconds..."; \
	    sleep 2; \
	done
ifeq ($(IS_DOCKER),yes)
	#(cd $(TAURI_APP_NAME)/ && ./src-tauri/target/debug/vigilant lslogins)
	(cd $(TAURI_APP_NAME) && pnpm tauri dev)
else
	(cd $(TAURI_APP_NAME)/ && ./src-tauri/target/debug/vigilant lslogins)
endif

package:
	(cd $(TAURI_APP_NAME) && pnpm tauri build)
.PHONY: stop
stop:
	pm2 stop $(PROCESS_NAME) && pm2 delete $(PROCESS_NAME)

.PHONY:clean
clean: stop
	@echo "Cleaning up..."
	rm -rf node_modules package.json package-lock.json
	(cd $(TAURI_APP_NAME) && rm -rf node_modules package-lock.json)
	@echo "Cleanup completed."
