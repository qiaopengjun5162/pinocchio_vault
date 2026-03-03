mod pb;
use crate::pb::pinocchio_vault::v1 as pb_vault;
use crate::pb::sf::solana::r#type::v1 as solana;
use crate::pb::sf::substreams::solana::v1::Transactions;

// ✅ 仅使用本地生成的类型，不再引用外部 crate
use crate::pb::sf::substreams::sink::database::v1::{
    DatabaseChanges, Field, TableChange, table_change::Operation,
};

const DISCRIMINATOR_DEPOSIT: u8 = 0;
const DISCRIMINATOR_WITHDRAW: u8 = 1;

#[substreams::handlers::map]
pub fn map_program_data(trxs: Transactions) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes = DatabaseChanges::default();

    for confirmed_trx in trxs.transactions {
        // 获取区块高度
        let block_num = confirmed_trx.slot;

        if let Some(trx) = &confirmed_trx.transaction {
            let tx_hash = bs58::encode(&trx.signatures[0]).into_string();
            let message = trx.message.as_ref().expect("message should exist");

            // 使用 enumerate() 获取指令索引，防止多指令主键冲突
            for (inst_idx, inst) in message.instructions.iter().enumerate() {
                let program_id =
                    bs58::encode(&message.account_keys[inst.program_id_index as usize])
                        .into_string();
                substreams::log::info!("Checking program: {}", program_id);
                if program_id != "A11gcDm7e8Pit4RiunfhtrK1BKU4oYAa3nx54R4YnFgS" {
                    continue;
                }

                if let Some((&discriminator, rest_data)) = inst.data.split_first() {
                    // 生成唯一主键：交易哈希 + 指令索引
                    let pk = format!("{}-{}", tx_hash, inst_idx);
                    let rest_data_slice: &[u8] = rest_data;

                    match discriminator {
                        DISCRIMINATOR_DEPOSIT => {
                            if rest_data_slice.len() == 8 {
                                let mut amount_bytes = [0u8; 8];
                                amount_bytes.copy_from_slice(rest_data);
                                let amount = u64::from_le_bytes(amount_bytes);

                                // 安全获取账户
                                let owner = get_account_safe(message, &inst.accounts, 0);
                                let vault = get_account_safe(message, &inst.accounts, 1);

                                // ✅ 在 DISCRIMINATOR_DEPOSIT 匹配块中
                                database_changes.table_changes.push(TableChange {
                                    table: "deposits".to_string(),
                                    pk,
                                    ordinal: inst_idx as u64, // 记录指令在交易中的位置
                                    operation: Operation::Create as i32,
                                    fields: vec![
                                        new_field("owner", owner),
                                        new_field("vault", vault),
                                        new_field("amount", amount.to_string()),
                                        new_field("block_number", block_num.to_string()), // ✅ 高度
                                    ],
                                });
                            }
                        }
                        DISCRIMINATOR_WITHDRAW => {
                            let owner = get_account_safe(message, &inst.accounts, 0);
                            let vault = get_account_safe(message, &inst.accounts, 1);

                            database_changes.table_changes.push(TableChange {
                                table: "withdraws".to_string(),
                                pk,
                                ordinal: inst_idx as u64,
                                operation: Operation::Create as i32,
                                fields: vec![
                                    new_field("owner", owner),
                                    new_field("vault", vault),
                                    new_field("block_number", block_num.to_string()), // ✅ 高度
                                ],
                            });
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    Ok(database_changes)
}

// ✅ 辅助函数：创建 Field
fn new_field(name: &str, value: String) -> Field {
    Field {
        name: name.to_string(),
        new_value: value,
        old_value: "".to_string(),
    }
}

fn get_account_safe(message: &solana::Message, inst_accounts: &[u8], index: usize) -> String {
    match inst_accounts.get(index) {
        Some(&account_idx) => {
            let idx = account_idx as usize;
            if idx < message.account_keys.len() {
                bs58::encode(&message.account_keys[idx]).into_string()
            } else {
                "unknown_idx".to_string()
            }
        }
        None => "missing_account".to_string(),
    }
}
