pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod macros;
pub mod processor;

pub mod instructions {
    pub mod close_game;
    pub mod commit_maneuvers;
    pub mod fire_weapon;
    pub mod join_queue;
    pub mod leave_queue;
    pub mod skip;
    pub mod upgrade;
}

pub mod utils {
    pub mod dictionary;
    pub mod random;
    pub mod vec2;
}

pub mod gameplay {
    pub mod game_session;
    pub mod matchmaking;
    pub mod obstacle;
    pub mod ship;
}
