use godot::prelude::*;

mod player;
mod game;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
