extern crate glium;

use glium::*;
use glutin::*;
use std::time;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let wb = glutin::WindowBuilder::new()
        .with_title("Mandlebrot")
        .with_dimensions(dpi::LogicalSize::new(800.0, 600.0));
    let cb = glutin::ContextBuilder::new();
    let display = Display::new(wb, cb, &events_loop).unwrap();

    let mut move_dir = 0i8;
    let mut position = [0.0f32, 0.0f32, -2.5f32];
    let mut yaw = 0.0f32;

    let program = Program::from_source(
        &display,
        include_str!("mandlebrot.glslv"),
        include_str!("mandlebrot.glslf"),
        None).unwrap();

    let vertices = [
        Vertex {position: [1.0, 1.0]},
        Vertex {position: [1.0, -1.0]},
        Vertex {position: [-1.0, 1.0]},

        Vertex {position: [-1.0, -1.0]},
        Vertex {position: [1.0, -1.0]},
        Vertex {position: [-1.0, 1.0]},
    ];

    let vertex_buffer = VertexBuffer::new(&display, &vertices).unwrap();

    let mut running = true;

    while running {
        let t1 = time::Instant::now();

        let mut frame = display.draw();
        
        let res = display.get_framebuffer_dimensions(); 

        frame.draw(&vertex_buffer,
                   &index::NoIndices(index::PrimitiveType::TrianglesList),
                   &program,
                   &uniform! {iResolution: [res.0, res.1],
                              camPosition: position},
                   &Default::default()).unwrap();

        frame.finish().unwrap();

        match move_dir {
            1 => {
                position = [position[0] + yaw.sin() * 0.01, position[1], position[2] + yaw.cos() * 0.01];
            },
            2 => {

            },
            -1 => {
                position = [position[0] - yaw.sin() * 0.01, position[1], position[2] - yaw.cos() * 0.01];
            },
            -2 => {

            },
            _ => (),
        }

        events_loop.poll_events(|e| {
            match e{
                Event::WindowEvent {event, ..} => match event {
                    WindowEvent::CloseRequested => running = false,
                    _ => (),
                },
                Event::DeviceEvent {event, ..} => match event {
                    DeviceEvent::Key (KeyboardInput {virtual_keycode: key, state, ..}) => match key {
                        Some(VirtualKeyCode::W) => {
                            move_dir = 1;

                            if state == ElementState::Released && move_dir == 1 {
                                move_dir = 0;
                            }
                        },
                        Some(VirtualKeyCode::S) => {
                            move_dir = -1;

                            if state == ElementState::Released && move_dir == -1 {
                                move_dir = 0;
                            }
                        },
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        let t2 = time::Instant::now();

        //println!("Each frame took {:?}", t2.duration_since(t1)); 
    }
}
