use godot::classes::INode2D;
use godot::global::godot_print;
use godot::init::gdextension; 
use godot::init::ExtensionLibrary;
use godot::classes::Node2D;
use godot::obj::Base;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

#[derive(GodotClass)]
#[class(base = Node2D, init)]
struct DeltaTimeDebug{
    base: Base<Node2D>
}

#[godot_api]
impl INode2D for DeltaTimeDebug {
    fn process(&mut self, delta: f64){
        godot_print!("delta time: {}", delta);
    }
}