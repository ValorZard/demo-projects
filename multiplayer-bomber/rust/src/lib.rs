use godot::prelude::*;


mod gamestate;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
