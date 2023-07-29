use crate::gameplay::game_session;
use crate::gameplay::matchmaking;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], game_id: u64) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let matchmaking_account = next_account_info(account_info_iter)?;
    let game_session_account = next_account_info(account_info_iter)?;

    // Signer should be the same as the initializer
    if !initializer.is_signer {
        msg!("Initializer {} should be the signer", initializer.key);
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Get matchmaking data
    let mut matchmaking_data = matchmaking::get_or_create_data(
        program_id,
        &[
            initializer.clone(),
            system_program.clone(),
            matchmaking_account.clone(),
        ],
    )?;

    // Get session data
    let mut session_data = game_session::get_or_create_data(
        program_id,
        &[
            initializer.clone(),
            system_program.clone(),
            game_session_account.clone(),
        ],
        game_id,
    )?;

    matchmaking_data.active_games.remove(&session_data.player1);
    matchmaking_data.active_games.remove(&session_data.player2);

    if session_data.winning_player == Pubkey::default() {
        if *initializer.key == session_data.player1 {
            session_data.winning_player = session_data.player2;
        } else {
            session_data.winning_player = session_data.player1;
        }
    }

    // // Close pda
    // game_session::close_pda(
    //     program_id,
    //     &[
    //         initializer.clone(),
    //         system_program.clone(),
    //         game_session_account.clone(),
    //     ],
    //     game_id,
    // )?;

    // Write data to pda
    game_session::write_data(
        &[
            initializer.clone(),
            system_program.clone(),
            game_session_account.clone(),
        ],
        &session_data,
    );

    matchmaking::write_data(accounts, &matchmaking_data)
}
