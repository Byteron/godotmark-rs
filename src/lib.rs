use gdnative::prelude::*;
use godot_mark::GodotMark;

mod godot_mark;

fn init(handle: InitHandle) {
    handle.add_class::<GodotMark>();
}

godot_init!(init);