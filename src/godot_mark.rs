use gdnative::{api::Engine, prelude::*};
use rand::Rng;

struct Mover {
    position: Vector2,
    velocity: Vector2,
    texture: Ref<Texture>,
}

impl Default for Mover {
    fn default() -> Self {
        Mover {
            position: Vector2::new(5.0, 5.0),
            velocity: Vector2::zero(),
            texture: unsafe { ResourceLoader::godot_singleton().load("res://icon.png", "", false).unwrap().assume_safe().cast::<Texture>().unwrap().assume_shared() },
        }
    }
}
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct GodotMark {
    movers: Vec<Mover>,
    #[property]
    instances: i32,
    #[property]
    speed: f32,
    label: Option<Ref<Label>>,
}

impl Default for GodotMark {
    fn default() -> Self {
        GodotMark {
            movers: Vec::default(),
            instances: 1000,
            speed: 1000.0,
            label: None,
        }
    }
}

#[methods]
impl GodotMark {
    fn new(_owner: &Node2D) -> Self {
        GodotMark::default()
    }

    #[export]
    fn _ready(&mut self, _owner: &Node2D) {
        self.label = Some ( unsafe { _owner.get_node_as::<Label>("CanvasLayer/Label").unwrap().assume_shared() });
    }

    #[export]
    fn _process(&mut self, _owner: &Node2D, delta: f32) {
        if Input::godot_singleton().is_action_just_pressed("ui_accept") {
            godot_print!("ui_accept pressed!");
            self.spawn_movers();
        }

        for mut mover in self.movers.iter_mut() {
            mover.position += mover.velocity * self.speed * delta;

            if mover.position.x < 0.0 || mover.position.x > 1024.0 {
                mover.velocity.x = -mover.velocity.x;
            }

            if mover.position.y < 0.0 || mover.position.y > 600.0 {
                mover.velocity.y = -mover.velocity.y;
            }
        }

        let mut label = unsafe { self.label.unwrap().assume_safe() };

        label.set_text(format!("FPS: {}\nMovers: {}", Engine::godot_singleton().get_frames_per_second(), self.movers.len()));

        _owner.update();
    }

    #[export]
    fn _draw(&self, _owner: &Node2D) {
        for mover in self.movers.iter() {
            let texture = unsafe { mover.texture.assume_safe() };
            _owner.draw_texture(texture, mover.position, Color::rgba(1.0, 1.0, 1.0, 1.0), GodotObject::null())
        }
    }

    fn spawn_movers(&mut self) {
        let mut rng = rand::thread_rng();

        for i in 0..self.instances {
            let mut mover = Mover::default();
            mover.velocity = Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            self.movers.push(mover);
        }

        godot_print!("spawed 1K movers!");
    }
}
