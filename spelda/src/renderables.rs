use crate::prelude::*;

pub fn load_renderables(mut renderables: Models<Renderables>) -> Models<Renderables> {
    use CreatureRenderables::*;

    renderables
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Renderables {
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CreatureRenderables { 
}