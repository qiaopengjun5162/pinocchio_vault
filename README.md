# Pinocchio Vault 🤥

**Pinocchio Vault** 是一个高度模块化的 Solana 智能合约项目，旨在展示如何使用现代 Solana 开发范式构建安全的资金存取库。项目不仅包含高效的 Rust 合约，还通过 **Codama** (原 Kinobi) 实现了从 IDL 到多语言客户端（TypeScript & Rust）的自动化代码生成。

## 🌟 项目亮点

* **Anchor-free 原生 Rust**: 核心合约基于原生 Rust 编写，追求极致的性能和低堆栈占用。
* **Codama 驱动的工程化**: 使用 `codegen.ts` 自动化生成客户端 SDK，确保前端与合约指令完全同步。
* **多语言 SDK 支持**:
* **TypeScript**: 基于 `solana/web3.js` v1.x 生成的 TS 库。
* **Rust**: 基于最新 **Solana 3.x (Agave)** 架构生成的强类型 Rust Client。

* **自动化 PDA 处理**: 客户端自动派生 `["vault", owner]` 种子地址，简化集成难度。

---

## 📂 项目结构

```text
pinocchio_vault
├── src/                # [核心] Solana 智能合约代码 (Rust)
│   ├── instructions/   # 指令逻辑: Deposit, Withdraw
│   └── lib.rs          # 程序入口与账户定义
├── idl/                # 合约接口定义文件 (JSON)
├── clients/            # [自动生成] 客户端 SDK
│   ├── src/generated/  # 包含生成的 TS 和 Rust 核心代码
│   │   ├── js/         # TypeScript SDK
│   │   └── rust/       # Rust SDK (包含测试 bin)
│   ├── codegen.ts      # Codama 生成配置文件
│   └── test_vault.ts   # TS 集成测试脚本
├── scripts/            # 工具脚本
└── Makefile            # 快捷指令集 (Build, Deploy, Codegen)

```

---

## 🛠 开发环境配置

### 1. 合约编译与部署

```bash
# 编译合约
cargo build-sbf

# 部署至本地节点 (确保 validator 已启动)
solana program deploy ./deploy_out/blueshift_vault.so

```

### 2. 代码生成 (Codama)

本项目使用 Codama 根据 `idl/*.json` 自动同步客户端代码：

```bash
cd clients
pnpm install
# 执行生成脚本，同步更新 js 和 rust 目录下的 generated 代码
pnpm run generate

```

---

## 🚀 客户端集成示例

### Rust 客户端 (Pinocchio Test)

位于 `clients/src/generated/rust`，适配最新的模块化依赖。

```bash
cd clients/src/generated/rust
# 运行存取自动化测试流
cargo run --bin blueshift_test

```

### TypeScript 客户端

```typescript
import { createDepositInstruction } from './clients/src/generated/js';

// 极简的指令构建示例
const ix = createDepositInstruction({
  owner: payer.publicKey,
  vault: vaultPda,
  amount: 500_000_000, // 0.5 SOL
});

```

---

## ⛓ 合约逻辑说明

| 指令 | 说明 | 涉及账户 |
| --- | --- | --- |
| **Deposit** | 用户将 SOL 存入 Vault PDA | `owner` (Signer), `vault` (Writable), `system_program` |
| **Withdraw** | 用户从 Vault 提取资金 | `owner` (Signer), `vault` (Writable), `system_program` |

**PDA 派生种子**: `[b"vault", payer_pubkey.as_ref()]`

---

## 🤝 贡献与反馈

如果你在运行过程中发现任何 Bug 或有功能建议，欢迎提交 Issue 或 Pull Request。

## 📜 许可证

本项目采用 [MIT](https://www.google.com/search?q=LICENSE) 许可证。
