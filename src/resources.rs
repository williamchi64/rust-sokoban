use std::fmt::{self, Display};

// resources.rs
use ggez::event::KeyCode;
use specs::World;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
}
// resources.rs
#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32
}
// resources.rs
pub enum GameplayState {
    Playing,
    Won
}
// resources.rs
impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won"
        })?;
        Ok(())
    }
}