use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct Obstacle {
    pub id: u32,
    pub x_position: f32,
    pub y_position: f32,
    pub size: f32,
    pub health: u32,
}
