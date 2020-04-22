use crate::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct StateInstance {
    engine: RPopsEngine<Renderables>,
}

#[methods]
impl StateInstance {
    fn _init(owner: Node) -> Self {
        let mut instance = StateInstance { engine: RPopsEngine::<Renderables>::new(owner) };
        // Add systems
        instance.engine.set_systems(create_systems());
        
        // Add resources
        let renderables = Models::<Renderables>::default();
        instance.engine.resources.insert(load_renderables(renderables));
        
        instance
    }

    #[export]
    fn _ready(&mut self, _owner: Node) {
        // Call engine _ready
        self.engine._ready(_owner);
        let renderables = self.engine.resources.get::<Models<Renderables>>().unwrap();
        let player = renderables.data_from_t(&Renderables::Player).unwrap();

        self.engine.world.insert(
            (), 
            vec![(
                Renderable { index: player.1, template: player.0 }, 
                GDSpatial, 
                Position { x: 240f32, y: 450f32, rotation: euclid::Angle::radians(0f32) }
            )]
        );
    }

    #[export]
    fn _physics_process(&mut self, owner: Node, delta: f64) {
        self.engine._physics_process(owner, delta);
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<StateInstance>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();