use godot::prelude::*;

mod bomb;
mod bomb_spawner;
mod gamestate;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
