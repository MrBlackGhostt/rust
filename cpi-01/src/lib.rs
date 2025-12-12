use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    no: u32,
}
entrypoint!(process_instruction);

fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let mut iter = accounts.iter();
    let account_data = next_account_info(&mut iter)?;
    let mut counter = Counter::try_from_slice(&mut ac ount_data.data.borrow_mut())?;

    if counter.no == 0 {
        counter.no += 1
    } else {
        counter.no *= 2
    }
    counter.serialize(&mut *account_data.data.borrow_mut());
    Ok(())
}
