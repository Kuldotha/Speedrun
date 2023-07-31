use crate::gameplay::game_session;
use crate::gameplay::matchmaking;
use crate::gameplay::ship::{ManeuverData, Ship, WeaponData};
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

    let mut matchmaking_data = matchmaking::get_or_create_data(
        program_id,
        &[
            initializer.clone(),
            system_program.clone(),
            matchmaking_account.clone(),
        ],
    )?;

    // TODO: Can't join if already in queue or in active game

    // Try make match
    if matchmaking_data.queue.len() < 1 {
        matchmaking_data.add_to_queue(initializer.key);
    } else {
        let player1 = *initializer.key;
        let player2 = matchmaking_data.queue.remove(0);
        matchmaking_data.queue.retain(|&pubkey| pubkey != player2);

        // Add both players to the active games list so they can find the id of their active game
        matchmaking_data.active_games.insert(player1, game_id);
        matchmaking_data.active_games.insert(player2, game_id);

        // Create game session
        let mut game_session_data = game_session::get_or_create_data(
            program_id,
            &[
                initializer.clone(),
                system_program.clone(),
                game_session_account.clone(),
            ],
            game_id,
        )?;

        // Initialize game session
        game_session_data.player1 = player1;
        game_session_data.player2 = player2;
        game_session_data.active_player = player1;

        let health = 100.0;
        let min_speed = 10.0;
        let max_speed = 20.0;
        let min_angle = 0.0;
        let max_angle = 90.0;
        let weapon_arc = 45.0;
        let weapon_range = 30.0;
        let weapon_damage = 40.0;
        let hit_chance = 0.7;

        game_session_data.ships = vec![
            Ship {
                id: 0,
                owner: player1,
                x_position: 0.0,
                y_position: -20.0,
                rotation: 90.0,
                health: health,
                min_speed: min_speed,
                max_speed: max_speed,
                min_angle: min_angle,
                max_angle: max_angle,
                maneuver_data: ManeuverData {
                    angle: 0.0,
                    speed: 0.0,
                },
                weapon_data: WeaponData {
                    arc: weapon_arc,
                    range: weapon_range,
                    damage: weapon_damage,
                    hit_chance: hit_chance,
                },
                activated: false,
            },
            Ship {
                id: 1,
                owner: player1,
                x_position: -20.0,
                y_position: -20.0,
                rotation: 90.0,
                health: health,
                min_speed: min_speed,
                max_speed: max_speed,
                min_angle: min_angle,
                max_angle: max_angle,
                maneuver_data: ManeuverData {
                    angle: 0.0,
                    speed: 0.0,
                },
                weapon_data: WeaponData {
                    arc: weapon_arc,
                    range: weapon_range,
                    damage: weapon_damage,
                    hit_chance: hit_chance,
                },
                activated: false,
            },
            Ship {
                id: 2,
                owner: player1,
                x_position: -40.0,
                y_position: -20.0,
                rotation: 90.0,
                health: health,
                min_speed: min_speed,
                max_speed: max_speed,
                min_angle: min_angle,
                max_angle: max_angle,
                maneuver_data: ManeuverData {
                    angle: 0.0,
                    speed: 0.0,
                },
                weapon_data: WeaponData {
                    arc: weapon_arc,
                    range: weapon_range,
                    damage: weapon_damage,
                    hit_chance: hit_chance,
                },
                activated: false,
            },
            Ship {
                id: 3,
                owner: player2,
                x_position: 0.0,
                y_position: 40.0,
                rotation: -90.0,
                health: health,
                min_speed: min_speed,
                max_speed: max_speed,
                min_angle: min_angle,
                max_angle: max_angle,
                maneuver_data: ManeuverData {
                    angle: 0.0,
                    speed: 0.0,
                },
                weapon_data: WeaponData {
                    arc: weapon_arc,
                    range: weapon_range,
                    damage: weapon_damage,
                    hit_chance: hit_chance,
                },
                activated: false,
            },
            Ship {
                id: 4,
                owner: player2,
                x_position: 20.0,
                y_position: 40.0,
                rotation: -90.0,
                health: health,
                min_speed: min_speed,
                max_speed: max_speed,
                min_angle: min_angle,
                max_angle: max_angle,
                maneuver_data: ManeuverData {
                    angle: 0.0,
                    speed: 0.0,
                },
                weapon_data: WeaponData {
                    arc: weapon_arc,
                    range: weapon_range,
                    damage: weapon_damage,
                    hit_chance: hit_chance,
                },
                activated: false,
            },
            Ship {
                id: 5,
                owner: player2,
                x_position: 40.0,
                y_position: 40.0,
                rotation: -90.0,
                health: health,
                min_speed: min_speed,
                max_speed: max_speed,
                min_angle: min_angle,
                max_angle: max_angle,
                maneuver_data: ManeuverData {
                    angle: 0.0,
                    speed: 0.0,
                },
                weapon_data: WeaponData {
                    arc: weapon_arc,
                    range: weapon_range,
                    damage: weapon_damage,
                    hit_chance: hit_chance,
                },
                activated: false,
            },
        ];

        // Write data to pda
        game_session::write_data(
            &[
                initializer.clone(),
                system_program.clone(),
                game_session_account.clone(),
            ],
            &game_session_data,
        )?;
    }

    // Write data to pda
    matchmaking::write_data(accounts, &matchmaking_data)
}
