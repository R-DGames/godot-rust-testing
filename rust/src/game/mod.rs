use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct GameInit {
    base: Base<Node>,
}

#[godot_api]
impl INode for GameInit {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        // This runs when the node enters the scene tree
        godot_print!("GameInit ready!");
    }
}
