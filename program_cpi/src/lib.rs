use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult},
    instruction::{AccountMeta, Instruction},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?; //There the data is store 
    let double_contract = next_account_info(&mut iter)?; // This is the contract 

    let instruction = Instruction {
        program_id: *double_contract.key,
        accounts: vec![AccountMeta::new(*data_account.key, true)], // holding the double_contract
        // key which on run time get all
        // the meta data
        data: instructions.to_vec(),
    };

    invoke(&instruction, &[data_account.clone()])?;

    ProgramResult::Ok(())
}
