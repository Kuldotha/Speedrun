use crate::error::GameError;
use crate::gameplay::game_session;
use crate::utils::random::SplitMix64;
use crate::utils::vec2::Vector2;
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
    target_id: u32,
) -> ProgramResult {
    // Get accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let _system_program = next_account_info(account_info_iter)?;
    let game_session_account = next_account_info(account_info_iter)?;

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

    if target_id >= session_data.ships.len() as u32 {
        msg!("Invalid target id");
        return Err(GameError::InvalidTarget.into());
    }

    if ship_id >= session_data.ships.len() as u32 {
        msg!("Invalid ship id");
        return Err(GameError::InvalidShip.into());
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

    let target = session_data.ships[target_id as usize];
    let ship_pos = Vector2 {
        x: ship.x_position,
        y: ship.y_position,
    };
    let target_pos = Vector2 {
        x: target.x_position,
        y: target.y_position,
    };

    let delta = target_pos.subtract(&ship_pos);

    let rot_rad = ship.rotation.to_radians();
    let forward = Vector2 {
        x: rot_rad.cos(),
        y: rot_rad.sin(),
    };

    // Outside of firing arc
    let angle = forward.angle_between(&delta).to_degrees();
    if angle.abs() > ship.weapon_data.arc {
        msg!("Target outside firing arc");
        return Err(GameError::InvalidTarget.into());
    }

    // Outside of range
    let distance_squared = delta.sqr_magnitude();
    let range_squared = ship.weapon_data.range * ship.weapon_data.range;
    if distance_squared > range_squared {
        msg!("Target outside range");
        return Err(GameError::InvalidTarget.into());
    }

    let ship_amount = session_data.ships.len() as u32;
    let random = &mut SplitMix64::new(game_session_account.key);
    random.reset();
    random.skip(session_data.turn as u32 * ship_amount + ship.id);
    let r = random.next_double();

    if let Some(target) = session_data.ships.get_mut(target_id as usize) {
        // Check if we hit or miss
        if r < ship.weapon_data.hit_chance as f64 {
            target.health -= ship.weapon_data.damage;
        }

        // Destroy ships
        if target.health < 0.0 {
            target.activated = true;
            target.maneuver_data.speed = 0.0;
            target.maneuver_data.angle = 0.0;
        }
    }

    if let Some(ship) = session_data.ships.get_mut(ship_id as usize) {
        // Mark ship as activated
        ship.activated = true;
    }

    session_data.last_action = 1;
    session_data.last_action_data = vec![ship_id, target_id];

    let player1_has_alive_ship = session_data
        .ships
        .iter()
        .any(|x| x.owner == session_data.player1 && x.health > 0.0);
    let player2_has_alive_ship = session_data
        .ships
        .iter()
        .any(|x| x.owner == session_data.player2 && x.health > 0.0);

    // Player 2 wins!
    if !player1_has_alive_ship {
        session_data.winning_player = session_data.player2;
    }
    // Player 1 wins!
    else if !player2_has_alive_ship {
        session_data.winning_player = session_data.player1;
    }
    // This means all ships have activated
    else if session_data.ships.iter().all(|s| s.activated) {
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
    }
    // Toggle active player, but only if the other player has at least one non-activated ship
    else {
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
