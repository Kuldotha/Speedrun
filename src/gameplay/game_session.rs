use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::GameError;
use crate::gameplay::ship::Ship;
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
pub struct GameSession {
    pub bump_seed: u8,
    pub game_id: u64,
    pub player1: Pubkey,
    pub player2: Pubkey,

    // Flags for use in ending turns, phases etc.
    pub player1_ready: bool,
    pub player2_ready: bool,
    pub active_player: Pubkey,
    pub winning_player: Pubkey,

    // This is so the clients can see what action was done
    pub last_action: u32,
    pub last_action_data: Vec<u32>,

    // Game state
    pub turn: u8,
    pub phase: u8,
    pub ships: Vec<Ship>,
}

// Derive PDA
pub fn get_pda(program_id: &Pubkey, game_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            program_id.to_bytes().as_ref(),
            game_id.to_be_bytes().as_ref(),
        ],
        program_id,
    )
}

pub fn get_or_create_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    game_id: u64,
) -> Result<GameSession, ProgramError> {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let game_session_account = next_account_info(account_info_iter)?;

    // Provided pda should be the same as the derived pda
    let (pda, bump_seed) = get_pda(program_id, game_id);
    if pda != *game_session_account.key {
        msg!(
            "Invalid PDA for game_id {}, expected {} but received {}",
            game_id,
            pda,
            game_session_account.key
        );
        return Err(GameError::InvalidPDA.into());
    }

    // Check if account has already been initialized
    let lamports = **game_session_account.try_borrow_lamports()?;

    // Create account if it doesn't already exist
    if lamports == 0 {
        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(1024);

        invoke_signed(
            &system_instruction::create_account(
                initializer.key,
                game_session_account.key,
                rent_lamports,
                1024.try_into().unwrap(),
                program_id,
            ),
            &[
                initializer.clone(),
                game_session_account.clone(),
                system_program.clone(),
            ],
            &[&[
                program_id.as_ref(),
                game_id.to_be_bytes().as_ref(),
                &[bump_seed],
            ]],
        )?;
    }

    // Get account data
    let game_session_data = match GameSession::try_from_slice(&game_session_account.data.borrow()) {
        Ok(data) => data,
        Err(_) => {
            let new_game_session_data = GameSession {
                bump_seed: bump_seed,
                game_id: game_id,
                player1: Pubkey::default(),
                player2: Pubkey::default(),

                player1_ready: false,
                player2_ready: false,
                active_player: Pubkey::default(),
                winning_player: Pubkey::default(),

                last_action: 0,
                last_action_data: Vec::new(),

                // New game state
                turn: 1,
                phase: 0,
                ships: Vec::new(),
            };
            new_game_session_data
        }
    };

    Ok(game_session_data)
}

pub fn write_data(accounts: &[AccountInfo], game_session_data: &GameSession) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let game_session_account = next_account_info(account_info_iter)?;

    // Serialize data
    let mut new_serialized_data = vec![];
    game_session_data.serialize(&mut new_serialized_data)?;

    let cur_size = game_session_account.data.borrow().len();
    let new_size = new_serialized_data.len();

    // Check if account needs to be resized
    if cur_size != new_size {
        let rent = Rent::get()?;
        let new_minimum_balance = rent.minimum_balance(new_size);

        let lamports = **game_session_account.try_borrow_lamports()?;
        let lamports_diff = new_minimum_balance.saturating_sub(lamports);
        if lamports_diff > 0 {
            invoke(
                &system_instruction::transfer(
                    initializer.key,
                    game_session_account.key,
                    lamports_diff,
                ),
                &[
                    initializer.clone(),
                    game_session_account.clone(),
                    system_program.clone(),
                ],
            )?;
        }

        game_session_account.realloc(new_size, false)?;
    }

    // Store in game_session_account
    let data_field = &mut game_session_account.data.borrow_mut();
    data_field.copy_from_slice(&new_serialized_data);

    Ok(())
}

pub fn close_pda(program_id: &Pubkey, accounts: &[AccountInfo], game_id: u64) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let game_session_account = next_account_info(account_info_iter)?;

    // Provided pda should be the same as the derived pda
    let (pda, bump_seed) = get_pda(program_id, game_id);
    if pda != *game_session_account.key {
        msg!(
            "Invalid PDA for game_id {}, expected {} but received {}",
            game_id,
            pda,
            game_session_account.key
        );
        return Err(GameError::InvalidPDA.into());
    }

    // Check if account has already been initialized
    let mut lamports = 0;
    {
        lamports = **game_session_account.try_borrow_lamports()?;
    }

    if lamports > 0 {
        // Purge all data
        {
            let data_field = &mut game_session_account.data.borrow_mut();
            data_field.fill(0u8);
        }

        // Remove all funds from session account
        invoke_signed(
            &system_instruction::transfer(&game_session_account.key, &initializer.key, lamports),
            &[
                game_session_account.clone(),
                initializer.clone(),
                system_program.clone(),
            ],
            &[&[
                program_id.to_bytes().as_ref(),
                game_id.to_be_bytes().as_ref(),
                &[bump_seed],
            ]],
        )?;
    }

    Ok(())
}
