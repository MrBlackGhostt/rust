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
    let mut iter = accounts.iter();
    let data_account = next_account_info(&mut iter)?;
    msg!("The data_account in the double_contract:- {data_account}");
    let instructions = CountInstruction::try_from_slice(instructions)?;

    let mut count = Count::try_from_slice(*data_account.data.borrow())?;

    match instructions {
        CountInstruction::Init(amount) => {
            count.no = amount;
            count.serialize(&mut *data_account.data.borrow_mut())?;
        }
        CountInstruction::Double => {
            count.no *= 2;
            count.serialize(&mut *data_account.data.borrow_mut())?;
        }

        CountInstruction::Half => {
            count.no /= 2;
            count.serialize(&mut *data_account.data.borrow_mut())?;
        }
    }

    Ok(())
}
