use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Define the data structure that our contract will use
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MyData {
    pub data: u32,
}

// Entry point for the contract
entrypoint!(process_instruction);

// Main processing function
fn process_instruction(
    program_id: &Pubkey,      // Public key of the program
    accounts: &[AccountInfo], // Accounts passed to the program
    _instruction_data: &[u8], // Instruction data
) -> ProgramResult {
    msg!("Hello, Solana!");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to hold the data
    let account = next_account_info(accounts_iter)?;

    // Deserialize the data, increment it, and serialize it back
    let mut my_data = MyData::try_from_slice(&account.data.borrow())?;
    my_data.data += 1;
    my_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Data incremented!");

    Ok(())
}
