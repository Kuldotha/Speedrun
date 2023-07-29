use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::GameError;
use crate::utils::dictionary::Dictionary;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MatchmakingData {
    pub bump_seed: u8,
    pub queue: Vec<Pubkey>,
    pub active_games: Dictionary<Pubkey, u64>,
}

// Derive PDA
pub fn get_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[program_id.to_bytes().as_ref(), b"matchmaking"],
        program_id,
    )
}

pub fn get_or_create_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<MatchmakingData, ProgramError> {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let matchmaking_account = next_account_info(account_info_iter)?;

    // Provided pda should be the same as the derived pda
    let (pda, bump_seed) = get_pda(program_id);
    if pda != *matchmaking_account.key {
        msg!(
            "Invalid PDA, expected {} but received {}",
            pda,
            matchmaking_account.key
        );
        return Err(GameError::InvalidPDA.into());
    }

    // Check if account has already been initialized
    let lamports = **matchmaking_account.try_borrow_lamports()?;

    // Create account if it doesn't already exist
    if lamports == 0 {
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(1024);

        invoke_signed(
            &system_instruction::create_account(
                initializer.key,
                matchmaking_account.key,
                rent_lamports,
                1024.try_into().unwrap(),
                program_id,
            ),
            &[
                initializer.clone(),
                matchmaking_account.clone(),
                system_program.clone(),
            ],
            &[&[program_id.as_ref(), b"matchmaking", &[bump_seed]]],
        )?;
    }

    // Get account data
    let matchmaking_data = match MatchmakingData::try_from_slice(&matchmaking_account.data.borrow())
    {
        Ok(data) => data,
        Err(_) => {
            let new_matchmaking_data = MatchmakingData {
                bump_seed: bump_seed,
                queue: Vec::new(),
                active_games: Dictionary::new(),
            };
            new_matchmaking_data
        }
    };

    Ok(matchmaking_data)
}

pub fn write_data(accounts: &[AccountInfo], matchmaking_data: &MatchmakingData) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let matchmaking_account = next_account_info(account_info_iter)?;

    // Serialize data
    let mut new_serialized_data = vec![];
    matchmaking_data.serialize(&mut new_serialized_data)?;

    let cur_size = matchmaking_account.data.borrow().len();
    let new_size = new_serialized_data.len();

    // Check if account needs to be resized
    if cur_size != new_size {
        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_size);

        let lamports = **matchmaking_account.try_borrow_lamports()?;
        let lamports_diff = new_minimum_balance.saturating_sub(lamports);
        if lamports_diff > 0 {
            invoke(
                &system_instruction::transfer(
                    initializer.key,
                    matchmaking_account.key,
                    lamports_diff,
                ),
                &[
                    initializer.clone(),
                    matchmaking_account.clone(),
                    system_program.clone(),
                ],
            )?;
        }

        matchmaking_account.realloc(new_size, false)?;
    }

    // Store in matchmaking_account
    let data_field = &mut matchmaking_account.data.borrow_mut();
    data_field.copy_from_slice(&new_serialized_data);

    Ok(())
}

impl MatchmakingData {
    pub fn add_to_queue(&mut self, player: &Pubkey) {
        self.queue.push(*player);
    }

    pub fn leave_queue(&mut self, player: &Pubkey) {
        self.queue.retain(|&pubkey| pubkey != *player);
    }
}
