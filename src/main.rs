#[macro_use]
extern crate glium;
extern crate cgmath;

use cgmath::{Matrix4, Point3, Vector3, Matrix};
use std::time::Instant;

// partially based on
// https://github.com/gfx-rs/gfx/blob/master/examples/performance/main.rs

#[derive(Copy, Clone)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

implement_vertex!(Vertex, pos, color);


const CLEAR_COLOR: (f32, f32, f32, f32) = (0.0, 0.0, 0.4, 0.0);

pub const VERTEX_DATA: [Vertex; 36] = [
    Vertex { pos: [-0.5, -0.5, -0.5], color: [0.583,  0.771,  0.014,] }, // triangle 1 : begin
    Vertex { pos: [-0.5, -0.5, 0.5,], color: [0.609,  0.115,  0.436,] },
	Vertex { pos: [-0.5, 0.5, 0.5,], color: [0.327,  0.483,  0.844,] }, // triangle 1 : end
	Vertex { pos: [0.5, 0.5,-0.5,], color: [0.822,  0.569,  0.201,] }, // triangle 2 : begin
	Vertex { pos: [-0.5,-0.5,-0.5,], color: [0.435,  0.602,  0.223,] },
	Vertex { pos: [-0.5, 0.5,-0.5,], color: [0.310,  0.747,  0.185,] }, // triangle 2 : end
	Vertex { pos: [0.5,-0.5, 0.5,], color: [0.597,  0.770,  0.761,] },
	Vertex { pos: [-0.5,-0.5,-0.5,], color: [0.559,  0.436,  0.730,] },
	Vertex { pos: [0.5,-0.5,-0.5,], color: [0.359,  0.583,  0.152,] },
	Vertex { pos: [0.5, 0.5,-0.5,], color: [0.483,  0.596,  0.789,] },
	Vertex { pos: [0.5,-0.5,-0.5,], color: [0.559,  0.861,  0.639,] },
	Vertex { pos: [-0.5,-0.5,-0.5,], color: [0.195,  0.548,  0.859,] },
	Vertex { pos: [-0.5,-0.5,-0.5,], color: [0.014,  0.184,  0.576,] },
	Vertex { pos: [-0.5, 0.5, 0.5,], color: [0.771,  0.328,  0.970,] },
	Vertex { pos: [-0.5, 0.5,-0.5,], color: [0.406,  0.615,  0.116,] },
	Vertex { pos: [0.5,-0.5, 0.5,], color: [0.676,  0.977,  0.133,] },
	Vertex { pos: [-0.5,-0.5, 0.5,], color: [0.971,  0.572,  0.833,] },
	Vertex { pos: [-0.5,-0.5,-0.5,], color: [0.140,  0.616,  0.489,] },
	Vertex { pos: [-0.5, 0.5, 0.5,], color: [0.997,  0.513,  0.064,] },
	Vertex { pos: [-0.5,-0.5, 0.5,], color: [0.945,  0.719,  0.592,] },
	Vertex { pos: [0.5,-0.5, 0.5,], color: [0.543,  0.021,  0.978,] },
	Vertex { pos: [0.5, 0.5, 0.5,], color: [0.279,  0.317,  0.505,] },
	Vertex { pos: [0.5,-0.5,-0.5,], color: [0.167,  0.620,  0.077,] },
	Vertex { pos: [0.5, 0.5,-0.5,], color: [0.347,  0.857,  0.137,] },
	Vertex { pos: [0.5,-0.5,-0.5,], color: [0.055,  0.953,  0.042,] },
	Vertex { pos: [0.5, 0.5, 0.5,], color: [0.714,  0.505,  0.345,] },
	Vertex { pos: [0.5,-0.5, 0.5,], color: [0.783,  0.290,  0.734,] },
	Vertex { pos: [0.5, 0.5, 0.5,], color: [0.722,  0.645,  0.174,] },
	Vertex { pos: [0.5, 0.5,-0.5,], color: [0.302,  0.455,  0.848,] },
	Vertex { pos: [-0.5, 0.5,-0.5,], color: [0.225,  0.587,  0.040,] },
	Vertex { pos: [0.5, 0.5, 0.5,], color: [0.517,  0.713,  0.338,] },
	Vertex { pos: [-0.5, 0.5,-0.5,], color: [0.053,  0.959,  0.120,] },
	Vertex { pos: [-0.5, 0.5, 0.5,], color: [0.393,  0.621,  0.362,] },
	Vertex { pos: [0.5, 0.5, 0.5,], color: [0.673,  0.211,  0.457,] },
	Vertex { pos: [-0.5, 0.5, 0.5,], color: [0.820,  0.883,  0.371,] },
	Vertex { pos: [0.5,-0.5, 0.5,], color: [0.982,  0.099,  0.879] },
];

fn main() {
    use glium::{DisplayBuild, Surface};
    let now = Instant::now();
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();

    let positions = glium::VertexBuffer::new(&display, &VERTEX_DATA).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_src = r#"
        #version 330 core

        // Input vertex data, different for all executions of this shader.
        in vec3 pos;
        in vec3 color;

        // Output data ; will be interpolated for each fragment.
        out vec3 fragmentColor;
        // Values that stay constant for the whole mesh.
        uniform mat4 MVP;

        void main(){
            // Output position of the vertex, in clip space : MVP * position
            gl_Position =  MVP * vec4(pos,1);

            // The color of each vertex will be interpolated
            // to produce the color of each fragment
            fragmentColor = color;
        }
    "#;

    let fragment_src = r#"
        #version 330 core

        // Interpolated values from the vertex shaders
        in vec3 fragmentColor;

        // Ouput data
        out vec3 color;

        void main(){
            // Output color = color specified in the vertex shader,
            // interpolated between all 3 surrounding vertices
            color = fragmentColor;
        }
    "#;


    let program = glium::Program::from_source(&display, vertex_src, fragment_src, None).unwrap();
    let aspect = 800.0f32 / 600.00f32;

    let perspective = cgmath::perspective(cgmath::Deg(45.0f32), aspect, 0.1f32, 100.0f32);
    let transform = perspective * default_view();

    loop {
        let mut target = display.draw();
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
//        let rotate = rotate_y(&transform, 1.0);
        let sine = (now.elapsed().as_secs() as f32).sin();
        let cosine = (now.elapsed().as_secs() as f32).cos();
        let mvp: [[f32; 4]; 4] = cgmath::Matrix4::new(
            cosine,          -sine,           transform[0][2], transform[0][3],
            sine,            cosine,          transform[1][2], transform[1][3],
            transform[2][0], transform[2][1], transform[2][2], transform[2][3],
            transform[3][0], transform[3][1], transform[3][2], transform[3][3],
        ).transpose().into();
        target.clear_color_and_depth(CLEAR_COLOR, 1.0);
        target.draw(&positions, &indices, &program, &uniform! { MVP: mvp }, &params)
            .unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

fn default_view() -> Matrix4<f32> {
    Matrix4::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0f32, 0.0, 0.0),
        Vector3::unit_z(),
    )
}
