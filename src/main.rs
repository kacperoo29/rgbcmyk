mod camera;
mod color_picker;
mod cube;

use crate::color_picker::ColorPicker;
use crate::cube::Cube;

use gloo_events::EventListener;
use nalgebra_glm::Vec3;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::prelude::*;

enum Msg {
    KeyDown(KeyboardEvent),
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
}

struct App {
    camera: camera::Camera,
    is_mouse_down: bool,
    last_mouse_pos: (f32, f32),
    cube_rotation: (f32, f32),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let keydown_callback = ctx.link().callback(Msg::KeyDown);

        EventListener::new(&window().unwrap(), "keydown", move |event| {
            let keyboard_event = event.clone().dyn_into::<KeyboardEvent>().unwrap();

            keydown_callback.emit(keyboard_event)
        })
        .forget();

        Self {
            camera: camera::Camera::new(),
            is_mouse_down: false,
            last_mouse_pos: (0.0, 0.0),
            cube_rotation: (0.0, 0.0),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove_callback = ctx.link().callback(Msg::MouseMove);
        let onmousedown_callback = ctx.link().callback(Msg::MouseDown);
        let onmouseup_callback = ctx.link().callback(Msg::MouseUp);

        html! {
            <div class="container">
                <ColorPicker />
                <Cube
                    view={self.camera.calculate_view_matrix()}
                    projection={self.camera.calculate_projection_matrix()}
                    onmousemove={onmousemove_callback}
                    onmousedown={onmousedown_callback}
                    onmouseup={onmouseup_callback}
                    x_rotation={self.cube_rotation.0}
                    y_rotation={self.cube_rotation.1}
                />
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(key_event) => match key_event.key().as_str() {
                "w" => self.camera.move_dir(Vec3::new(0.0, 0.0, 1.0)),
                "s" => self.camera.move_dir(Vec3::new(0.0, 0.0, -1.0)),
                "a" => self.camera.move_dir(Vec3::new(-1.0, 0.0, 0.0)),
                "d" => self.camera.move_dir(Vec3::new(1.0, 0.0, 0.0)),
                _ => return false,
            },
            Msg::MouseMove(event) => {
                let (x, y) = (event.offset_x() as f32, event.offset_y() as f32);
                //if self.is_mouse_down {
                self.cube_rotation = (x - self.last_mouse_pos.0, y - self.last_mouse_pos.1);
                // }

                self.last_mouse_pos = (x, y);
            }
            Msg::MouseDown(_) => {
                self.is_mouse_down = true;
            }
            Msg::MouseUp(_) => {
                self.is_mouse_down = false;
            }
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
