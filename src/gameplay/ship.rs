use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub struct Ship {
    pub id: u32,
    pub owner: Pubkey,
    pub x_position: f32,
    pub y_position: f32,
    pub rotation: f32,
    pub health: f32,

    pub min_speed: f32,
    pub max_speed: f32,
    pub min_angle: f32,
    pub max_angle: f32,

    pub maneuver_data: ManeuverData,
    pub weapon_data: WeaponData,
    pub activated: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub struct ManeuverData {
    pub angle: f32,
    pub speed: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy)]
pub struct WeaponData {
    pub arc: f32,
    pub range: f32,
    pub damage: f32,
    pub hit_chance: f32,
}
