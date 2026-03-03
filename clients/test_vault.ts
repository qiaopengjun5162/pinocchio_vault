import {
    createSolanaClient,
    createTransaction,
    signTransactionMessageWithSigners,
    address,
    getSignatureFromTransaction,
    getProgramDerivedAddress,
    getAddressEncoder
} from "gill"
import { loadKeypairSignerFromFile } from "gill/node"
import { getDepositInstruction } from "./src/generated/js/instructions/deposit.js"
import { getWithdrawInstruction } from "./src/generated/js/instructions/withdraw.js"
import { BLUESHIFT_VAULT_PROGRAM_ADDRESS } from "./src/generated/js/programs/blueshiftVault.js"
import path from "path"

// --- 1. 配置 ---
// const RPC_URL = "http://127.0.0.1:8899"
const RPC_URL = process.env.DEVNET_RPC_URL ?? "http://127.0.0.1:8899"
const WALLET_PATH = path.join(process.env.HOME!, ".config/solana/id.json")
// 填入你 make deploy 得到的真实 Program ID
// const MY_PROGRAM_ID = address("A11gcDm7e8Pit4RiunfhtrK1BKU4oYAa3nx54R4YnFgS")
const MY_PROGRAM_ID = address(process.env.PROGRAM_ID ?? "A11gcDm7e8Pit4RiunfhtrK1BKU4oYAa3nx54R4YnFgS")
async function main() {
    console.log("🚀 Starting Blueshift Vault Test...")

    // 2. 初始化客户端
    const { rpc, sendAndConfirmTransaction } = createSolanaClient({
        urlOrMoniker: RPC_URL,
    })

    // 3. 加载钱包 (Depositor/Owner)
    const signer = await loadKeypairSignerFromFile(WALLET_PATH)
    console.log("🔑 Payer loaded:", signer.address)

    // 4. 正确推导 Vault PDA
    console.log("🔍 Deriving Vault PDA...")

    // 注意：seeds 必须是 Uint8Array 数组
    const [vaultAddress] = await getProgramDerivedAddress({
        programAddress: MY_PROGRAM_ID,
        seeds: [
            new TextEncoder().encode("vault"),          // 字符串种子
            getAddressEncoder().encode(signer.address), // 地址种子（必须先 encode）
        ],
    })

    console.log("📍 Vault PDA Address:", vaultAddress)

    // 5. 获取最新 Blockhash
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send()

    // 6. 构建 Deposit 指令
    console.log("📦 Creating Deposit Instruction...")
    const depositAmount = 500_000_000n // 0.5 SOL

    const depositIx = getDepositInstruction({
        owner: signer,
        vault: vaultAddress,
        amount: depositAmount,
    }, { programAddress: MY_PROGRAM_ID })

    // 7. 构建 Withdraw 指令
    const withdrawIx = getWithdrawInstruction({
        owner: signer,
        vault: vaultAddress,
    }, { programAddress: MY_PROGRAM_ID })

    // --- 8. 发送 Deposit 交易 ---
    const depositTx = createTransaction({
        feePayer: signer,
        latestBlockhash: (await rpc.getLatestBlockhash().send()).value,
        instructions: [depositIx],
        version: "legacy",
    })
    try {
        console.log("⏳ Sending Deposit...")
        const signedDeposit = await signTransactionMessageWithSigners(depositTx)
        const signature = getSignatureFromTransaction(signedDeposit)

        await sendAndConfirmTransaction(signedDeposit)
        console.log("✅ Deposit OK!")
        console.log(`🔗 Transaction: https://explorer.solana.com/tx/${signature}?cluster=custom&customUrl=${RPC_URL}`)


        // --- 9. 发送 Withdraw 交易 ---
        // 重新获取 Blockhash 保证交易新鲜
        const withdrawTx = createTransaction({
            feePayer: signer,
            latestBlockhash: (await rpc.getLatestBlockhash().send()).value,
            instructions: [withdrawIx],
            version: "legacy",
        })

        // 10. 签名并发送
        console.log("⏳ Sending Withdraw...")
        const signedWithdraw = await signTransactionMessageWithSigners(withdrawTx)
        const withdrawSig = getSignatureFromTransaction(signedWithdraw)


        await sendAndConfirmTransaction(signedWithdraw)
        console.log("✅ Withdraw OK!")
        console.log(`🔗 Transaction: https://explorer.solana.com/tx/${withdrawSig}?cluster=custom&customUrl=${RPC_URL}`)
    } catch (err) {
        console.error("❌ Transaction failed:", err)
    }
}

main().catch(console.error)
