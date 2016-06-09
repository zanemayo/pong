#[macro_use]
extern crate glium;

#[derive(Clone, Copy)]
struct Vertex {
    position : [f32; 2],
}

fn main() {
    implement_vertex!(Vertex, position);

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float t;

        void main() {
            vec2 pos = position;
            // Scale
            // pos.x *= t;
            // pos.y *= t;

            // Rotate:
            // pos = vec2(pos.x * cos(t)  - pos.y * sin(t), pos.x * sin(t) + pos.y * cos(t));
            gl_Position = vec4(pos, 0.0, 1.0);

            //Skew
            pos.x += pos.y * t;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    
    let mut t: f32 = -0.5;
    loop {

        t += 0.01;
        if t > 0.5 {
            t = -0.5;
        }

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniform! {t: t},  &Default::default()).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

