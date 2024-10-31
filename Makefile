# Makefile for installing Node.js, npm, and npx

# Specify the desired Node.js version
NODE_VERSION := 18.16.0  # Change this to your desired version
TAURI_APP_NAME := vigilant4

.PHONY: all install_nvm install_node clean

all: install_nvm install_node

# Install NVM
install_nvm:
	@echo "Installing NVM (Node Version Manager)..."
	curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
	@echo "NVM installed. Please restart your terminal or run the following command:"
	@echo "export NVM_DIR=\"$$(HOME)/.nvm\" && [ -s \"$$(nvm --version)\" ] && . \"$$(nvm --version)\"/nvm.sh"

# Install Node.js using NVM
install_node: install_nvm
	@echo "Installing Node.js version $(NODE_VERSION)..."
	@bash -c "source $$HOME/.nvm/nvm.sh && nvm install $(NODE_VERSION) && nvm use $(NODE_VERSION) && nvm alias default $(NODE_VERSION)"
	@echo "Node.js installed successfully."

# Create a new Tauri app
create_tauri_app: install_node
	#@echo "Creating a new Tauri app named $(TAURI_APP_NAME)..."
	#npx create-react-app $(TAURI_APP_NAME) && cd $(TAURI_APP_NAME) && tauri init
	#@echo "Tauri app $(TAURI_APP_NAME) created successfully."

preinstall: install_node install_nvm 

build:
	(cd $(TAURI_APP_NAME)/src-tauri && cargo build)

start: build
	(cd $(TAURI_APP_NAME) && npm start)

clean:
	@echo "Cleaning up..."
	rm -rf node_modules package.json package-lock.json
	@echo "Cleanup completed."
