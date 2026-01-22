
# Blueshift Vault Client

一个基于 Rust 的高性能 Solana 客户端，用于与 **Blueshift Vault** 智能合约进行交互。本项目采用了最新的 **Solana 3.0+ (Agave)** 架构，并使用 **Codama (原 Kinobi)** 自动生成的代码来实现精确的指令构建和类型安全的账户处理。

## 核心功能

* **自动化指令构建**：利用 `DepositBuilder` 和 `WithdrawBuilder` 实现简洁的交易构造。
* **动态 PDA 派生**：客户端根据 Payer 地址自动计算 `Vault` 的程序派生地址 (PDA)。
* **最新架构兼容**：完全适配 Solana 3.1+ 的模块化依赖（如 `solana-rpc-client`, `solana-system-interface`）。
* **健壮的错误处理**：集成 `anyhow` 处理运行时的各类异常。

## 技术栈

* **语言**: Rust (Edition 2024)
* **区块链**: Solana (v3.0.0+)
* **代码生成**: Codama / Kinobi
* **序列化**: Borsh (v1.6.0)

---

## 快速开始

### 1. 前置条件

确保你已安装以下工具：

* [Rust](https://rustup.rs/) (建议使用最新的 stable 版本)
* [Solana CLI](https://docs.solanalabs.com/cli/install)
* 本地已运行 `solana-test-validator`

### 2. 配置环境

将你的合约（Program）部署到本地集群，并更新项目中的 `program_id`。

```bash
# 启动本地验证器
solana-test-validator

# 部署合约 (在合约项目目录下)
solana program deploy ./target/deploy/blueshift_vault.so

```

### 3. 安装依赖与编译

```bash
git clone https://github.com/your-username/blueshift_vault-client.git
cd blueshift_vault-client
cargo build

```

### 4. 运行测试脚本

脚本将自动执行一次 **Deposit (0.5 SOL)** 和一次 **Withdraw (全额提现)**。

```bash
cargo run --bin blueshift_test

```

---

## 项目结构

```text
.
├── src
│   ├── main.rs          # 测试脚本入口 (二进制目标)
│   ├── lib.rs           # 库入口，导出生成代码
│   └── generated        # 由 Codama 自动生成的代码 (Instructions, Accounts, Errors)
├── Cargo.toml           # 包含模块化的 Solana 3.x 依赖配置
└── README.md

```

## 核心交互逻辑

项目通过以下步骤实现安全存取：

1. **种子派生**：使用 `["vault", payer_pubkey]` 作为种子派生唯一的 Vault PDA。
2. **原子交易**：构建包含特定指令数据的 `Transaction` 对象。
3. **签名发送**：通过 RPC 接口推送到网络，并阻塞直到 `Confirmed` 状态。

## 许可证

本项目采用 MIT 许可证。
