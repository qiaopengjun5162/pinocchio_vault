use solana_commitment_config::CommitmentConfig;
use solana_rpc_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Signer, read_keypair_file};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;

// 导入你生成的代码
use blueshift_vault_client::generated::instructions::{DepositBuilder, WithdrawBuilder};
fn main() -> anyhow::Result<()> {
    // 1. 初始化 RPC 客户端 (Localnet)
    let rpc_url = "http://127.0.0.1:8899";
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // 2. 加载本地钱包
    let payer = read_keypair_file(format!("{}/.config/solana/id.json", env!("HOME")))
        .expect("需要本地 Solana 钱包文件");

    let program_id = solana_sdk::pubkey!("A11gcDm7e8Pit4RiunfhtrK1BKU4oYAa3nx54R4YnFgS");

    println!("🔑 Payer: {}", payer.pubkey());

    // 3. 派生 Vault PDA
    let (vault_pda, _) =
        Pubkey::find_program_address(&[b"vault", payer.pubkey().as_ref()], &program_id);
    println!("📍 Vault PDA: {}", vault_pda);

    // --- 4. 执行 Deposit ---
    println!("📦 构建 Deposit 指令...");
    let deposit_amount = 500_000_000; // 0.5 SOL

    let system_prog_id = system_program::id();
    // 使用生成的 DepositBuilder
    let deposit_ix = DepositBuilder::new()
        .owner(payer.pubkey())
        .vault(vault_pda)
        .system_program(system_prog_id)
        .amount(deposit_amount)
        .instruction();

    let latest_blockhash = client.get_latest_blockhash()?;
    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&deposit_tx)?;
    println!("✅ Deposit 成功! 签名: {}", sig);

    // --- 5. 执行 Withdraw ---
    println!("💸 构建 Withdraw 指令...");

    // 使用生成的 WithdrawBuilder
    let withdraw_ix = WithdrawBuilder::new()
        .owner(payer.pubkey())
        .vault(vault_pda)
        .system_program(system_prog_id)
        .instruction();

    let latest_blockhash = client.get_latest_blockhash()?;
    let withdraw_tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&withdraw_tx)?;
    println!("✅ Withdraw 成功! 签名: {}", sig);

    Ok(())
}
