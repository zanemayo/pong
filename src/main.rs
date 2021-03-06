#[macro_use]
extern crate glium;
extern crate image;

#[derive(Clone, Copy)]
struct Vertex {
    position : [f32; 2],
    tex_coords: [f32; 2]
}

fn init() {
    implement_vertex!(Vertex, position, tex_coords);
}

fn load_image(name: &str) -> glium::texture::RawImage2d<u8> {
    let image = image::load(std::io::Cursor::new(&include_bytes!("assets/texture.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions)
}

fn get_vertex_shader() -> &'static str {
    r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 my_attr;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            // my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
            my_attr[0] = gl_Position[0];
            my_attr[1] = gl_Position[1];
        }
    "#
}

fn get_fragment_shader() -> &'static str {
    r#"
        #version 140

        in vec2 my_attr;
        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
             // color = vec4(my_attr, 0.0, 1.0);
             color = texture(tex, v_tex_coords);
        }
    "#
}

// pub use glutin_backend::GlutinFacade as Display;
fn load_texture(textureFilename: &str, display: &glium::glutin::GlutinFacade) -> glium::texture::Texture2d {
    let image = load_image("texture.png");
    glium::texture::Texture2d::new(&display, image).unwrap()
}

fn main() {

    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let texture = load_texture("texture.png", &display);

    let vertex_shader_src = get_vertex_shader();

    let fragment_shader_src = get_fragment_shader();

    let vertex1 = Vertex { position: [-0.5, -0.5] , tex_coords: [0.0, 0.0]};
    let vertex2 = Vertex { position: [0.0, 0.5], tex_coords: [0.0, 1.0] };
    let vertex3 = Vertex { position: [0.5, -0.25], tex_coords: [1.0, 0.0] };
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

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t, 0.0, 0.0, 1.0f32]
            ],
            tex: &texture
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,  &Default::default()).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

//    let vertex_shader_src = r#"
//        #version 140
//
//        in vec2 position;
//
//        uniform float t;
//
//        void main() {
//            vec2 pos = position;
//            // Scale
//            // pos.x *= t;
//            // pos.y *= t;
//
//            // Rotate:
//            //pos = vec2(pos.x * cos(t)  - pos.y * sin(t), pos.x * sin(t) + pos.y * cos(t));
//
//            //Skew
//            pos.x += pos.y * t;
//            gl_Position = vec4(pos, 0.0, 1.0);
//        }
//    "#;
