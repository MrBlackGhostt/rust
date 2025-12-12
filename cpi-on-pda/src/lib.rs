use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    example_mocks::solana_sdk::address_lookup_table::instruction,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: &[u8],
) -> ProgramResult {
    let mut iter = accounts.iter();

    let pda = next_account_info(&mut iter)?;
    let user_account = next_account_info(&mut iter)?;
    let double_contract_program = next_account_info(&mut iter)?;

    let instruction = Instruction {
        program_id: *double_contract_program.key,
        accounts: vec![AccountMeta::new(*pda.key, true)],
        data: instruction.to_vec(),
    };

    let seed = &[b"data_account", user_account.key.as_ref()];

    let (pda, bump) = Pubkey::find_program_address(seed, program_id);

    invoke_signed(&instruction, accounts, &[seed, &[[bump]]]);
    Ok(())
}
