use godot::prelude::*;

mod bomb;
mod gamestate;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
