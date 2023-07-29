use borsh::BorshDeserialize;
use crate::gameplay::ship::ManeuverData;
use crate::utils::dictionary::Dictionary;
use solana_program::program_error::ProgramError;

pub enum GameInstruction {
    JoinQueue {
        game_id: u64,
    },
    LeaveQueue {},
    CloseGame {
        game_id: u64,
    },
    CommitManeuvers {
        game_id: u64,
        maneuver_data: Dictionary<u32, ManeuverData>,
    },
    FireWeapon {
        game_id: u64,
        ship_id: u32,
        target_id: u32,
    },
    Upgrade {
        game_id: u64,
        ship_id: u32,
        upgrade_id: u32,
    },
    Skip {
        game_id: u64,
        ship_id: u32,
    },
}

#[derive(BorshDeserialize)]
struct GameManagementPayload {
    game_id: u64,
}

#[derive(BorshDeserialize)]
struct CommitManeuversPayload {
    game_id: u64,
    maneuver_data: Dictionary<u32, ManeuverData>,
}

#[derive(BorshDeserialize)]
struct FireWeaponPayload {
    game_id: u64,
    ship_id: u32,
    target_id: u32,
}

#[derive(BorshDeserialize)]
struct UpgradePayload {
    game_id: u64,
    ship_id: u32,
    upgrade_id: u32,
}

#[derive(BorshDeserialize)]
struct SkipPayload {
    game_id: u64,
    ship_id: u32,
}

impl GameInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            11 => {
                let payload = GameManagementPayload::try_from_slice(rest).unwrap();
                Ok(Self::JoinQueue {
                    game_id: payload.game_id,
                })
            }
            12 => Ok(Self::LeaveQueue {}),
            21 => {
                let payload = GameManagementPayload::try_from_slice(rest).unwrap();
                Ok(Self::CloseGame {
                    game_id: payload.game_id,
                })
            }
            22 => {
                let payload = UpgradePayload::try_from_slice(rest).unwrap();
                Ok(Self::Upgrade {
                    game_id: payload.game_id,
                    ship_id: payload.ship_id,
                    upgrade_id: payload.upgrade_id,
                })
            }
            23 => {
                let payload = CommitManeuversPayload::try_from_slice(rest).unwrap();
                Ok(Self::CommitManeuvers {
                    game_id: payload.game_id,
                    maneuver_data: payload.maneuver_data,
                })
            }
            24 => {
                let payload = FireWeaponPayload::try_from_slice(rest).unwrap();
                Ok(Self::FireWeapon {
                    game_id: payload.game_id,
                    ship_id: payload.ship_id,
                    target_id: payload.target_id,
                })
            }
            25 => {
                let payload = SkipPayload::try_from_slice(rest).unwrap();
                Ok(Self::Skip {
                    game_id: payload.game_id,
                    ship_id: payload.ship_id,
                })
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    }
}
