use glium::{glutin, Surface};
use pixpipe::{Color, PixBuf};
use winit::event_loop::ControlFlow;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut pix_buf = PixBuf::with_dimensions(320, 200);
    pix_buf.fill(Color::BLUE);
    pix_buf.set(10, 10, Color::WHITE);

    let mut pipeline = match pixpipe_glium::Pipeline::new(&display) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            eprintln!("Could not create pipeline {:?}", err);
            return;
        }
    };

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

                glutin::event::WindowEvent::Resized(size) => {
                    pipeline.resize(size.width, size.height);
                }

                _ => return,
            },

            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },

            glutin::event::Event::MainEventsCleared => {
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                if !pipeline.draw(&display, &mut target, &pix_buf).is_ok() {
                    *control_flow = ControlFlow::Exit;
                }

                target.finish().unwrap();
            }

            _ => return,
        }
    });
}
