use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum CountInstruction {
    Init(u32),
    Double,
    Half,
}

entrypoint!(process_instructions);

pub fn process_instructions(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8],
) -> ProgramResult {
    msg!("CPI program execution");
    msg!("Program ID: {}", program_id);
    let mut iter = accounts.iter();
    //The data_account in which the value to be change
    let siger = next_account_info(&mut iter)?;
    let data_account = next_account_info(&mut iter)?;
    let program_id = next_account_info(&mut iter)?;

    let instructions = CountInstruction::try_from_slice(instructions)?;
    msg!("Data Account: {}", data_account.key);
    msg!("Target Program: {}", program_id.key);
    msg!("Received instruction bytes: {:?}", instructions);
    match instructions {
        CountInstruction::Init(amount) => {
            let inx = Instruction {
                program_id: *program_id.key,
                accounts: vec![
                    AccountMeta::new(*siger.key, true),
                    AccountMeta::new(*data_account.key, false),
                ],
                data: vec![0],
            };
            msg!("Invoking with  {:?}", inx.data);
            invoke(&inx, &[data_account.clone()])?;
            msg!("✅ CPI: Init completed successfully");
        }

        CountInstruction::Double => {
            let inx = Instruction {
                program_id: *program_id.key,
                accounts: vec![AccountMeta::new(*data_account.key, false)],
                data: vec![1],
            };
            msg!("Invoking with  {:?}", inx.data);
            invoke(&inx, &[data_account.clone()])?;
            msg!("✅ CPI: Double completed successfully");
        }
        CountInstruction::Half => {
            let inx = Instruction {
                program_id: *program_id.key,

                accounts: vec![AccountMeta::new(*data_account.key, false)],
                data: vec![2],
            };
            msg!("Invoking with  {:?}", inx.data);
            invoke(&inx, &[data_account.clone()])?;
            msg!("✅ CPI: Half completed successfully");
        }
    }

    Ok(())
}
