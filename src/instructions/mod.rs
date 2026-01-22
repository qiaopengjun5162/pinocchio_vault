// 1. 声明子模块
pub mod deposit;
pub mod withdraw;

// 2. 导出子模块内容，方便外部调用
pub use deposit::*;
pub use withdraw::*;

// 只有在开启 idl-build 时才引入和编译这段
#[cfg(feature = "idl-build")]
use {
    borsh::{BorshDeserialize, BorshSerialize},
    shank::ShankInstruction,
};

#[cfg(feature = "idl-build")]
#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum VaultInstruction {
    /// 指令 0: 向 Vault 存入 SOL
    /// 账户顺序必须对应 DepositAccounts 的 try_from 逻辑
    #[account(0, signer, writable, name = "owner", desc = "存款人和支付者")]
    #[account(1, writable, name = "vault", desc = "派生的 Vault PDA 账户")]
    #[account(2, name = "system_program", desc = "System Program")]
    Deposit(DepositArgs), // Deposit { amount: u64 }, // 直接写成 struct 风格更直观

    /// 指令 1: 从 Vault 提取所有 SOL
    /// 账户顺序必须对应 WithdrawAccounts 的 try_from 逻辑
    #[account(0, signer, writable, name = "owner", desc = "提款人/所有者")]
    #[account(1, writable, name = "vault", desc = "派生的 Vault PDA 账户")]
    #[account(2, name = "system_program", desc = "System Program")]
    Withdraw,
}

#[cfg(feature = "idl-build")]
/// 定义 Deposit 指令接收的参数
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct DepositArgs {
    pub amount: u64,
}
