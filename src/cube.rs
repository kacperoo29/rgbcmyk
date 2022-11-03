use js_sys::{Float32Array, Uint32Array};
use nalgebra::{Unit, UnitQuaternion};
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject};
use yew::prelude::*;

//https://www.cubic.org/docs/3dclip.htm#ma4
//https://glbook.gamedev.net/GLBOOK/glbook.gamedev.net/moglgp/advclip.html

use nalgebra_glm::{Mat4x4, Vec3};

const VERT_SHADER: &str = r#"#version 300 es

    in vec3 a_position;
    in vec3 a_color;

    uniform mat4 u_model;
    uniform mat4 u_view;
    uniform mat4 u_projection;

    out vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = u_projection * u_view * u_model * vec4(a_position, 1.0);
    }
"#;

const FRAG_SHADER: &str = r#"#version 300 es

    precision mediump float;

    in vec3 v_color;

    out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
"#;

const CROSS_VERT_SHADER: &str = r#"#version 300 es
    
        in vec2 a_position;
        in vec3 a_color;

        uniform mat4 u_model;
        uniform mat4 u_view;
        uniform mat4 u_projection;

        out vec3 v_color;

        void main() {
            vec4 color = u_view * u_model * vec4(a_color, 1.0);
            v_color = color.xyz;
            gl_Position = u_projection * vec4(a_position, -1.0, 1.0);
        }
"#;

const CROSS_FRAG_SHADER: &str = r#"#version 300 es

    precision mediump float;

    in vec3 v_color;

    out vec4 color;

    void main() {
        color = vec4(v_color, 1.0);
    }
"#;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub view: Mat4x4,
    pub projection: Mat4x4,
    pub onmousemove: Callback<MouseEvent>,
    pub onmousedown: Callback<MouseEvent>,
    pub onmouseup: Callback<MouseEvent>,
    pub x_rotation: f32,
    pub y_rotation: f32,
}

pub enum Msg {
    PosChanged(Vec3),
}

pub struct Cube {
    canvas: NodeRef,
    gl: Option<WebGl2RenderingContext>,
    shader_program: Option<WebGlProgram>,
    va: Option<WebGlVertexArrayObject>,
    view: Mat4x4,

    crosssection: NodeRef,
    crosssection_ctx: Option<WebGl2RenderingContext>,
    crosssection_shader_program: Option<WebGlProgram>,
    crosssection_va: Option<WebGlVertexArrayObject>,
    crossection_pos: Vec3,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            canvas: NodeRef::default(),
            gl: None,
            shader_program: None,
            va: None,
            view: Mat4x4::identity(),
            
            crosssection: NodeRef::default(),
            crosssection_ctx: None,
            crosssection_shader_program: None,
            crosssection_va: None,
            crossection_pos: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn rotate(&mut self, angle: f32, axis: Vec3) {
        let rotation = Mat4x4::from_axis_angle(&Unit::new_normalize(axis), angle);
        self.view = rotation * self.view;
    }
}

impl Component for Cube {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove_callback = ctx.props().onmousemove.clone();
        let onmousedown_callback = ctx.props().onmousedown.clone();
        let onmouseup_callback = ctx.props().onmouseup.clone();

        html! {
            <div>
                <canvas
                    width="400"
                    height="300"
                    ref={self.canvas.clone()}
                    onmousemove={onmousemove_callback}
                    onmousedown={onmousedown_callback}
                    onmouseup={onmouseup_callback.clone()}
                    onmouseleave={onmouseup_callback.clone()}
                />
                <canvas 
                    width="400"
                    height="300"
                    ref={self.crosssection.clone()}
                    onwheel={ctx.link().callback(|e: WheelEvent| {
                        e.prevent_default();
                        Msg::PosChanged(Vec3::new(0.0, 0.0, e.delta_y() as f32 / 100.0))
                    })}
                />
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PosChanged(pos) => {
                self.crossection_pos += pos;

                true
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        const ROTATION_SPEED: f32 = 0.02;

        self.rotate(
            ctx.props().x_rotation * ROTATION_SPEED,
            Vec3::new(0.0, 1.0, 0.0),
        );
        self.rotate(
            ctx.props().y_rotation * ROTATION_SPEED,
            Vec3::new(1.0, 0.0, 0.0),
        );

        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap();
            let gl = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap();

            gl.enable(WebGl2RenderingContext::DEPTH_TEST);
            self.gl = Some(gl);
            let gl = self.gl.as_ref().unwrap();

            let vert_shader =
                compile_shader(&gl, VERT_SHADER, WebGl2RenderingContext::VERTEX_SHADER).unwrap();
            let frag_shader =
                compile_shader(&gl, FRAG_SHADER, WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
            let shader_program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

            let va = gl.create_vertex_array();

            gl.bind_vertex_array(va.as_ref());
            // Cube vertices with RGB colors
            let vertices = Float32Array::from(
                [
                    // front
                    -0.5, -0.5, 0.5, 0.0, 1.0, 0.0, // bottom left
                    0.5, -0.5, 0.5, 0.0, 1.0, 1.0, // bottom right
                    0.5, 0.5, 0.5, 1.0, 1.0, 1.0, // top right
                    -0.5, 0.5, 0.5, 1.0, 1.0, 0.0, // top left
                    // back
                    -0.5, -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
                    0.5, -0.5, -0.5, 0.0, 0.0, 1.0, // bottom right
                    0.5, 0.5, -0.5, 1.0, 0.0, 1.0, // top right
                    -0.5, 0.5, -0.5, 1.0, 0.0, 0.0, // top left
                ]
                .as_slice(),
            );
            let buffer = gl.create_buffer();
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            // Create index buffer and fill in with cube index data
            let indices = Uint32Array::from(
                [
                    0, 1, 2, 2, 3, 0, // front
                    4, 5, 6, 6, 7, 4, // back
                    0, 4, 7, 7, 3, 0, // left
                    1, 5, 6, 6, 2, 1, // right
                    3, 2, 6, 6, 7, 3, // top
                    0, 1, 5, 5, 4, 0, // bottom
                ]
                .as_slice(),
            );

            let index_buffer = gl.create_buffer();
            gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                index_buffer.as_ref(),
            );
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            // Set up vertex attributes
            let position_attribute_location = gl.get_attrib_location(&shader_program, "a_position");
            gl.enable_vertex_attrib_array(position_attribute_location as u32);
            gl.vertex_attrib_pointer_with_i32(
                position_attribute_location as u32,
                3,
                WebGl2RenderingContext::FLOAT,
                false,
                24,
                0,
            );

            let color_attribute_location = gl.get_attrib_location(&shader_program, "a_color");
            gl.enable_vertex_attrib_array(color_attribute_location as u32);
            gl.vertex_attrib_pointer_with_i32(
                color_attribute_location as u32,
                3,
                WebGl2RenderingContext::FLOAT,
                false,
                24,
                12,
            );

            self.shader_program = Some(shader_program);
            self.va = va;

            let crosssection = self.crosssection.cast::<web_sys::HtmlCanvasElement>().unwrap();
            let crosssection_gl = crosssection
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>()
                .unwrap();

            self.crosssection_ctx = Some(crosssection_gl);
            
            let gl = self.crosssection_ctx.as_ref().unwrap();
            let vert_shader =
                compile_shader(&gl, CROSS_VERT_SHADER, WebGl2RenderingContext::VERTEX_SHADER).unwrap();
            let frag_shader =
                compile_shader(&gl, CROSS_FRAG_SHADER, WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
            let shader_program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

            let crosssection_va = gl.create_vertex_array();
            gl.bind_vertex_array(crosssection_va.as_ref());
            // Square vertices with RGB colors
            let vertices = Float32Array::from(
                [
                    // front
                    -0.5, -0.5, 0.0, 1.0, 0.0, // bottom left
                    0.5, -0.5, 0.0, 1.0, 1.0, // bottom right
                    0.5, 0.5, 1.0, 1.0, 1.0, // top right
                    -0.5, 0.5, 1.0, 1.0, 0.0, // top left
                ]
                .as_slice(),
            );

            let buffer = gl.create_buffer();
            gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            // Create index buffer and fill in with cube index data
            let indices = Uint32Array::from(
                [
                    0, 1, 2, 2, 3, 0, // front
                ]
                .as_slice(),
            );

            let index_buffer = gl.create_buffer();
            gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                index_buffer.as_ref(),
            );

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices,
                WebGl2RenderingContext::STATIC_DRAW,
            );

            // Set up vertex attributes
            let position_attribute_location = gl.get_attrib_location(&shader_program, "a_position");
            gl.enable_vertex_attrib_array(position_attribute_location as u32);
            gl.vertex_attrib_pointer_with_i32(
                position_attribute_location as u32,
                2,
                WebGl2RenderingContext::FLOAT,
                false,
                20,
                0,
            );

            let color_attribute_location = gl.get_attrib_location(&shader_program, "a_color");
            gl.enable_vertex_attrib_array(color_attribute_location as u32);
            gl.vertex_attrib_pointer_with_i32(
                color_attribute_location as u32,
                3,
                WebGl2RenderingContext::FLOAT,
                false,
                20,
                8,
            );

            self.crosssection_va = crosssection_va;
            self.crosssection_shader_program = Some(shader_program);
        }

        let gl = self.gl.as_ref().unwrap();
        let shader_program = self.shader_program.as_ref().unwrap();
        gl.use_program(Some(shader_program));

        // Set up uniforms
        let model_uniform_location = gl.get_uniform_location(&shader_program, "u_model");
        let view_uniform_location = gl.get_uniform_location(&shader_program, "u_view");
        let projection_uniform_location = gl.get_uniform_location(&shader_program, "u_projection");

        gl.uniform_matrix4fv_with_f32_array(
            model_uniform_location.as_ref(),
            false,
            &self.view.as_slice(),
        );

        gl.uniform_matrix4fv_with_f32_array(
            view_uniform_location.as_ref(),
            false,
            &ctx.props().view.as_slice(),
        );

        gl.uniform_matrix4fv_with_f32_array(
            projection_uniform_location.as_ref(),
            false,
            &ctx.props().projection.as_slice(),
        );

        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        gl.clear(WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        gl.bind_vertex_array(self.va.as_ref());
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            36,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );

        gl.bind_vertex_array(None);

        // draw crossection of cube
        let crosssection_gl = self.crosssection_ctx.as_ref().unwrap();
        let shader_program = self.crosssection_shader_program.as_ref().unwrap();
        crosssection_gl.use_program(Some(shader_program));
        
        let u_view = crosssection_gl.get_uniform_location(&shader_program, "u_view");
        crosssection_gl.uniform_matrix4fv_with_f32_array(
            u_view.as_ref(),
            false,
            &self.view.as_slice(),
        );

        let u_projection = crosssection_gl.get_uniform_location(&shader_program, "u_projection");
        let proj = nalgebra_glm::ortho(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
        crosssection_gl.uniform_matrix4fv_with_f32_array(
            u_projection.as_ref(),
            false,
            &proj.as_slice(),
        );

        let u_model = crosssection_gl.get_uniform_location(&shader_program, "u_model");
        crosssection_gl.uniform_matrix4fv_with_f32_array(
            u_model.as_ref(),
            false,
            &self.view.as_slice(),
        );

        crosssection_gl.clear_color(0.0, 0.0, 0.0, 1.0);
        crosssection_gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        crosssection_gl.bind_vertex_array(self.crosssection_va.as_ref());
        crosssection_gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            6,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );

    }

    fn destroy(&mut self, ctx: &Context<Self>) {}
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    source: &str,
    shader_type: u32,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
