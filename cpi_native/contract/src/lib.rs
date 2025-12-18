use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::sysvar::instructions,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
};

#[derive(BorshSerialize, BorshDeserialize)]
struct Count {
    no: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum CountInstruction {
    Double,
    Half,
}

entrypoint!(process_instructions);

pub fn process_instructions(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let signer = next_account_info(&mut iter)?;
    let programId = next_account_info(&mut iter)?;
    let instructions = CountInstruction::try_from_slice(instructions)?;

    match instructions {
        CountInstruction::Double => {
            let inx = Instruction {
                program_id: *programId.key,
                accounts: vec![AccountMeta::new(*signer.key, true)],
                data: vec![],
            };
            invoke(&inx, &[signer.clone(), programId.clone()])?;
        }
        CountInstruction::Half => {
            let inx = Instruction {
                program_id: *programId.key,
                accounts: vec![AccountMeta::new(*signer.key, true)],
                data: vec![],
            };
            invoke(&inx, &[signer.clone(), programId.clone()])?;
        }
    }

    Ok(())
}

