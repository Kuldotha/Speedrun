use crate::instruction::GameInstruction;
use crate::instructions::{
    close_game, commit_maneuvers, fire_weapon, join_queue, leave_queue, skip, upgrade,
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::unpack(instruction_data)?;
    match instruction {
        GameInstruction::JoinQueue { game_id } => {
            join_queue::process(program_id, accounts, game_id)
        }
        GameInstruction::LeaveQueue {} => leave_queue::process(program_id, accounts),
        GameInstruction::CloseGame { game_id } => {
            close_game::process(program_id, accounts, game_id)
        }
        GameInstruction::CommitManeuvers {
            game_id,
            maneuver_data,
        } => commit_maneuvers::process(program_id, accounts, game_id, maneuver_data),
        GameInstruction::FireWeapon {
            game_id,
            ship_id,
            target_id,
        } => fire_weapon::process(program_id, accounts, game_id, ship_id, target_id),
        GameInstruction::Skip { game_id, ship_id } => {
            skip::process(program_id, accounts, game_id, ship_id)
        }
        GameInstruction::Upgrade {
            game_id,
            ship_id,
            upgrade_id,
        } => upgrade::process(program_id, accounts, game_id, ship_id, upgrade_id),
    }
}
