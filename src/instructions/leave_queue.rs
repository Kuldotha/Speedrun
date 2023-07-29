use crate::gameplay::matchmaking;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;

    // Signer should be the same as the initializer
    if !initializer.is_signer {
        msg!("Initializer {} should be the signer", initializer.key);
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Get matchmaking account
    let mut matchmaking_data = matchmaking::get_or_create_data(program_id, accounts)?;

    // TODO: Can't leave if not in queue

    matchmaking_data.leave_queue(initializer.key);

    // Write data to pda
    matchmaking::write_data(accounts, &matchmaking_data)
}
