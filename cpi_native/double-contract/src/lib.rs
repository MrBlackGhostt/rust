use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::{self, ProgramResult},
    example_mocks::{
        solana_sdk::{system_program, sysvar::instructions},
        solana_transaction::Transaction,
    },
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    pubkey::{self, Pubkey},
    rent::Rent,
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
    let pda_account = next_account_info(&mut iter)?;
    let signer = next_account_info(&mut iter)?;
    let system_program = next_account_info(&mut iter)?;
    let seed = &[b"pda", signer.key.as_ref()];

    let (pda, bump) = Pubkey::find_program_address(seed, program_id);

    if pda != *pda_account.key {
        return Err(solana_program::program_error::ProgramError::NotEnoughAccountKeys);
    }

    msg!("The data_account in the double_contract:- {data_account}");
    let instructions = CountInstruction::try_from_slice(instructions)?;

    let mut count = Count::try_from_slice(*pda_account.data.borrow())?;
    match instructions {
        CountInstruction::Init(amount) => {
            // Create account if it doesn't exist
            if pda_account.data_is_empty() {
                let space = std::mem::size_of::<Count>();
                let rent = Rent::get()?.minimum_balance(space);

                // Proper create_account instruction
                let create_ix = system_instruction::create_account(
                    signer.key,   // From (payer)
                    &pda,         // To (new account)
                    rent,         // Lamports
                    space as u64, // Space
                    program_id,   // Owner
                );

                let signer_seeds: &[&[&[u8]]] = &[&[b"pda", signer.key.as_ref(), &[bump]]];

                invoke_signed(
                    &create_ix,
                    &[signer.clone(), pda_account.clone(), system_program.clone()],
                    signer_seeds,
                )?;

                msg!("PDA account created: {}", pda_account.key);
            }

            let mut count = Count { no: amount };
            count.serialize(&mut *pda_account.data.borrow_mut())?;
        }
        CountInstruction::Double => {
            count.no *= 2;
            count.serialize(&mut *pda_account.data.borrow_mut())?;
        }

        CountInstruction::Half => {
            count.no /= 2;
            count.serialize(&mut *pda_account.data.borrow_mut())?;
        }
    }

    Ok(())
}
