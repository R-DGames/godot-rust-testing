use gns::GnsGlobal;
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

        let gns_global = GnsGlobal::get();

        match gns_global {
            Ok(_gns_glob) => {
                godot_print!("sys::GameNetworkingSockets_Init was called and succeeded initialization");
            }
            Err(e) => {
                godot_print!("Uh hello?");
                std::process::exit(1);
            }
        }
    }
}
