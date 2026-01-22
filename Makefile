# --- 项目配置 ---
PROGRAM_NAME := blueshift_vault
CLUSTER ?= localnet
WALLET ?= ~/.config/solana/id.json
RPC_URL_localnet := http://localhost:8899
RPC_URL_devnet := https://api.devnet.solana.com
RPC_URL := $(RPC_URL_$(CLUSTER))

# 路径定义
IDL_DIR := ./idl
CLIENTS_DIR := ./clients
SDK_RS_DIR := $(CLIENTS_DIR)/src/generated/rust
PROGRAM_SO := target/deploy/$(PROGRAM_NAME).so

# --- 核心指令 ---

.PHONY: all build idl sdk deploy clean help prep-rs

all: build sdk

# 1. 编译 (Pinocchio 使用 cargo build-sbf)
build:
	@echo "🦀 Building Rust program for Solana..."
	@cargo build-sbf
	@echo "✅ Build complete: $(PROGRAM_SO)"

# 2. 提取 IDL (Shank)
idl:
	@echo "📜 Extracting IDL with Shank..."
	@mkdir -p $(IDL_DIR)
	@shank idl -o $(IDL_DIR) -r .

# 3. 自动准备 Rust 客户端 Crate 结构
prep-rs:
	@if [ ! -f $(SDK_RS_DIR)/Cargo.toml ]; then \
		echo "📦 Initializing Rust Client Crate..."; \
		mkdir -p $(SDK_RS_DIR); \
		cargo init --lib $(SDK_RS_DIR) --name $(PROGRAM_NAME)_client; \
		printf 'solana-program = "1.18"\nborsh = "1.0"\nthiserror = "1.0"\nnum-derive = "0.4"\nnum-traits = "0.2"\n' >> $(SDK_RS_DIR)/Cargo.toml; \
		echo "✅ Rust crate initialized with dependencies."; \
	fi

# 4. 生成所有 SDK (Codama)
sdk: idl prep-rs
	@echo "🚀 Generating SDKs (TS & Rust) with Codama..."
	@cd $(CLIENTS_DIR) && bunx codama run --all

# 5. 部署逻辑 (修正参数顺序)
deploy: build
	@echo "🚀 Deploying to $(CLUSTER)..."
	@solana program deploy \
		--url $(RPC_URL) \
		--keypair $(WALLET) \
		--use-rpc \
		$(PROGRAM_SO)

# 6. 获取程序 ID (从生成的 deploy keypair 中提取)
info:
	@echo "Program ID:"
	@solana address -k target/deploy/$(PROGRAM_NAME)-keypair.json 2>/dev/null || echo "Not deployed yet"

clean:
	@echo "🧹 Cleaning artifacts..."
	@rm -rf target/
	@rm -rf $(IDL_DIR)
	@rm -rf $(CLIENTS_DIR)/src/generated

help:
	@echo "Usage: make [command] [CLUSTER=devnet]"
	@echo "  build    编译合约"
	@echo "  idl      提取 IDL"
	@echo "  sdk      生成 TS/Rust SDK (包含自动初始化)"
	@echo "  deploy   部署到指定集群"
	@echo "  all      全流程: 编译 + 生成 SDK"
