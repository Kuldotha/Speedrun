use crate::error::GameError;
use crate::gameplay::game_session;
use crate::gameplay::ship::ManeuverData;
use crate::utils::dictionary::Dictionary;
use crate::utils::vec2::Vector2;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::f32::consts::PI;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    game_id: u64,
    maneuver_data: Dictionary<u32, ManeuverData>,
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

    // Mark us as ready
    if session_data.player1 == *initializer.key {
        if session_data.player1_ready {
            msg!("You already submitted your maneuvers");
            return Err(GameError::InvalidShip.into());
        }

        session_data.player1_ready = true;
    } else {
        if session_data.player2_ready {
            msg!("You already submitted your maneuvers");
            return Err(GameError::InvalidShip.into());
        }

        session_data.player2_ready = true;
    }

    // Loop over ships in session data and store the maneuvers
    for (ship_id, maneuver) in maneuver_data.iter() {
        if let Some(ship) = session_data.ships.get_mut(*ship_id as usize) {
            // Store this maneuver
            ship.maneuver_data.angle = maneuver.angle;
            ship.maneuver_data.speed = maneuver.speed;
        }
    }

    if session_data.player1_ready && session_data.player2_ready {
        for ship in session_data.ships.iter_mut() {
            // TODO: Check if maneuver is possible. End user can send anything they want, so we should check here
            // whether it's a valid option for this particular ship.

            if ship.health <= 0.0 {
                continue;
            }

            // Straight maneuver
            if ship.maneuver_data.angle == 0.0 {
                let rot_rad = ship.rotation.to_radians();
                let forward = Vector2 {
                    x: rot_rad.cos(),
                    y: rot_rad.sin(),
                };

                // Calculate the change in x and y based on the speed and angle
                let delta_x = ship.maneuver_data.speed * forward.x;
                let delta_y = ship.maneuver_data.speed * forward.y;

                // Apply the relative movement based on the ship's rotation
                ship.x_position += delta_x;
                ship.y_position += delta_y;
            }
            // Arc maneuver
            else {
                let radius = ship.maneuver_data.speed / (ship.maneuver_data.angle / 8.0 * PI);
                let arc = -(ship.maneuver_data.speed / (2.0 * PI * radius) * 360.0);

                let rot_rad = (ship.rotation + 90.0).to_radians();
                let right_x = rot_rad.cos();
                let right_y = rot_rad.sin();

                let centre_x = ship.x_position - right_x * radius;
                let centre_y = ship.y_position - right_y * radius;

                let arm_x = ship.x_position - centre_x;
                let arm_y = ship.y_position - centre_y;

                let arc_rad = arc.to_radians();
                let final_arm_x = arm_x * arc_rad.cos() - arm_y * arc_rad.sin();
                let final_arm_y = arm_x * arc_rad.sin() + arm_y * arc_rad.cos();

                ship.x_position = centre_x + final_arm_x;
                ship.y_position = centre_y + final_arm_y;
                ship.rotation = ship.rotation + arc;
            }

            if ship.x_position.abs() > 42.0 || ship.y_position.abs() > 73.0 {
                ship.health = 0.0;
            }
        }

        session_data.player1_ready = false;
        session_data.player2_ready = false;

        if !session_data.player1_ready && !session_data.player2_ready {
            session_data.phase = session_data.phase + 1;
            session_data.last_action = 0;
        }
    }

    game_session::write_data(accounts, &session_data)
}
