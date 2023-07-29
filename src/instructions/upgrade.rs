use crate::error::GameError;
use crate::gameplay::game_session;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    game_id: u64,
    ship_id: u32,
    upgrade_id: u32,
) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;

    // Signer should be the same as the initializer
    if !initializer.is_signer {
        msg!("Initializer {} should be the signer", initializer.key);
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Get session data
    let mut session_data = game_session::get_or_create_data(program_id, accounts, game_id)?;

    if session_data.winning_player != Pubkey::default() {
        msg!("Game is already finished");
        return Err(GameError::InvalidShip.into());
    }

    if *initializer.key != session_data.active_player {
        msg!("Not the active player");
        return Err(GameError::InvalidShip.into());
    }

    if ship_id >= session_data.ships.len() as u32 {
        msg!("Invalid ship id");
        return Err(GameError::InvalidTarget.into());
    }

    let ship = session_data.ships[ship_id as usize];
    if ship.owner != *initializer.key {
        msg!("You can only activate your own ship");
        return Err(GameError::InvalidShip.into());
    }

    if ship.activated {
        msg!("Ship already activated");
        return Err(GameError::InvalidShip.into());
    }

    if ship.health <= 0.0 {
        msg!("Ship is destroyed");
        return Err(GameError::InvalidShip.into());
    }

    if let Some(ship) = session_data.ships.get_mut(ship_id as usize) {
        // Mark ship as activated
        ship.activated = true;

        match upgrade_id {
            1 => {
                msg!("Applying Speed upgrade");
                ship.max_speed += 10.0;
            }
            2 => {
                msg!("Applying Agility upgrade");
                ship.max_angle += 11.25;
            }
            3 => {
                msg!("Applying Weapon Arc upgrade");
                ship.weapon_data.arc += 12.5;
            }
            4 => {
                msg!("Applying Weapon Range upgrade");
                ship.weapon_data.range += 10.0;
            }
            5 => {
                msg!("Applying Weapon Damage upgrade");
                ship.weapon_data.damage += 20.0;
            }
            6 => {
                msg!("Applying Weapon Hit Chance upgrade");
                ship.weapon_data.hit_chance += 0.1;
            }
            _ => {
                msg!("Invalid upgrade_id: {}", upgrade_id);
                return Err(GameError::InvalidShip.into());
            }
        }
    }

    session_data.last_action = 2;
    session_data.last_action_data = vec![ship_id, upgrade_id];

    // This means all ships have activated
    if session_data.ships.iter().all(|s| s.activated) {
        session_data.turn = session_data.turn + 1;
        session_data.active_player = session_data.player1;
        session_data.phase = 0;

        // Reset ship activation, but only if ship isn't destroyed
        for s in &mut session_data.ships {
            if s.health <= 0.0 {
                continue;
            }

            s.activated = false;
        }
    } else {
        // Toggle active player, but only if the other player has at least one non-activated ship
        if session_data
            .ships
            .iter()
            .any(|s| s.owner != session_data.active_player && !s.activated)
        {
            if session_data.active_player == session_data.player1 {
                session_data.active_player = session_data.player2;
            } else {
                session_data.active_player = session_data.player1;
            }
        }
    }

    game_session::write_data(accounts, &session_data)
}
