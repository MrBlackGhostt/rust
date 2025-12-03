use core::num;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use borsh::{BorshDeserialize, BorshSerialize};
entrypoint!(process_instruction);

#[derive(BorshDeserialize, BorshSerialize)]
struct Number {
    no: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
enum Math {
    increment(u32),
    decrement(u32),
}
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_data = next_account_info(&mut _accounts.iter())?; //the client
                                                                  //put the
                                                                  //pubkey of the
                                                                  //account and
                                                                  //the chain put
                                                                  //all the
                                                                  //accoun ifo
                                                                  //into and come
                                                                  //here
    let mut number = Number::try_from_slice(&account_data.data.borrow())?; //  borrow the
                                                                           // data comes
                                                                           // from the
                                                                           // account here

    match Math::try_from_slice(_instruction_data)? {
        // convert the byte of instruction so to know
        // what to do
        Math::increment(amount) => number.no += amount,
        Math::decrement(amount) => number.no -= amount,
    }

    number.serialize(&mut *account_data.data.borrow_mut())?;

    msg!("The Number update to {}", number.no);
    Ok(())
}
