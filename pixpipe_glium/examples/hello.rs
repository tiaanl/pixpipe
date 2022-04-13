use glium::index::PrimitiveType;
use glium::{glutin, Surface};
use glium::{implement_vertex, program, uniform};
use pixpipe::{Color, PixBuf};

fn main() {
    // building the display, ie. the main object
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut pix_buf = PixBuf::with_dimensions(64, 64);
    pix_buf.fill(Color::RED);

    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        pix_buf.as_slice(),
        (pix_buf.width(), pix_buf.height()),
    );
    let opengl_texture = glium::texture::CompressedSrgbTexture2d::new(&display, image).unwrap();

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);

        glium::VertexBuffer::new(
            &display,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [1.0, 0.0],
                },
            ],
        )
            .unwrap()
    };

    // building the index buffer
    let index_buffer =
        glium::IndexBuffer::new(&display, PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3])
            .unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 matrix;

                in vec2 position;
                in vec2 tex_coords;

                out vec2 v_tex_coords;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_tex_coords;
                out vec4 f_color;

                void main() {
                    f_color = texture(tex, v_tex_coords);
                }
            "
        },

        110 => {
            vertex: "
                #version 110

                uniform mat4 matrix;

                attribute vec2 position;
                attribute vec2 tex_coords;

                varying vec2 v_tex_coords;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 110
                uniform sampler2D tex;
                varying vec2 v_tex_coords;

                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100

                uniform lowp mat4 matrix;

                attribute lowp vec2 position;
                attribute lowp vec2 tex_coords;

                varying lowp vec2 v_tex_coords;

                void main() {
                    gl_Position = matrix * vec4(position, 0.0, 1.0);
                    v_tex_coords = tex_coords;
                }
            ",

            fragment: "
                #version 100
                uniform lowp sampler2D tex;
                varying lowp vec2 v_tex_coords;

                void main() {
                    gl_FragColor = texture2D(tex, v_tex_coords);
                }
            ",
        },
    )
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },

            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },

            glutin::event::Event::MainEventsCleared => {
                // building the uniforms
                let uniforms = uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ],
                    tex: &opengl_texture
                };

                // drawing a frame
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 0.0);
                target
                    .draw(
                        &vertex_buffer,
                        &index_buffer,
                        &program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();
            }

            _ => return,
        }
    });
}
