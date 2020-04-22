use crate::prelude::*;

pub fn load_renderables(mut renderables: Models<Renderables>) -> Models<Renderables> {
    use Renderables::*;

    renderables.insert(Some("Player"), Some(Player), load_scene("player"), Template::ASprite(AnimSprite::default()));

    renderables
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Renderables {
  Player
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CreatureRenderables { 
}