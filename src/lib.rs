#[allow(unused)]
mod pb;

use pb::mydata::v1 as mydata;

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana::b58;

// Solana System Program ID (native SOL transfers)
const SYSTEM_PROGRAM_ID: [u8; 32] =
    b58!("11111111111111111111111111111111");

#[substreams::handlers::map]
fn map_my_data(blk: Block) -> mydata::Transfers {
    let mut transfers = mydata::Transfers { transfers: vec![] };

    for trx in blk.transactions() {
        for instruction in trx.walk_instructions() {

            // Only system program
            if instruction.program_id() != SYSTEM_PROGRAM_ID {
                continue;
            }

            let accounts = instruction.accounts();
            if accounts.len() < 2 {
                continue;
            }

            // System transfer instruction = 4 bytes discriminator + 8 bytes amount
            let data = instruction.data();
            if data.len() < 12 {
                continue;
            }

            // First 4 bytes = instruction type
            let instruction_type = u32::from_le_bytes([
                data[0], data[1], data[2], data[3]
            ]);

            // 2 = SystemProgram::Transfer
            if instruction_type != 2 {
                continue;
            }

            // Next 8 bytes = amount
            let amount = u64::from_le_bytes([
                data[4], data[5], data[6], data[7],
                data[8], data[9], data[10], data[11],
            ]);

            let from = bs58::encode(accounts[0].0.clone()).into_string();
            let to = bs58::encode(accounts[1].0.clone()).into_string();

            transfers.transfers.push(mydata::Transfer {
                from,
                to,
                amount,
            });
        }
    }

    transfers
}