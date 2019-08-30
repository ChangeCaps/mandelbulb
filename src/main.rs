extern crate glium;

use glium::*;
use glutin::*;
use std::{time, thread};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn distance_mandelbulb(x: f32, z: f32, y: f32) -> f32 {
    let mut zx = x;
    let mut zy = y;
    let mut zz = z;
    
    let mut dr = 1.0;
    let mut r = 0.0;

    let power = 8.0;

    for _i in 0..10 {
        r = (zx * zx + zy * zy + zz * zz).sqrt();
        if r > 2.0 {
            break;
        };

        let mut theta = (zz/r).acos();
        let mut phi = zy.atan2(zx);
        dr = r.powf(power-1.0)*power*dr + 1.0;

        let zr = r.powf(power);
        theta *= power;
        phi *= power;

        zx = zr*theta.sin()*phi.cos() + x;
        zy = zr*phi.sin()*theta.sin() + y;
        zz = zr*theta.cos() + z;
    }

    return 0.5 * r.ln() * r / dr;
}

fn main() {
    let mut events_loop = EventsLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Mandelbulb");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &events_loop).unwrap();

    display.gl_window().window().grab_cursor(true).unwrap();
    display.gl_window().window().hide_cursor(true);

    let mut move_mouse = true;
    let mut move_dir = 0i8;
    let mut position = [0.0f32, 0.0f32, -2.5f32];
    let mut yaw = 0.0f32;
    let mut pitch = 0.0f32;
    let mut speed;

    let program = Program::from_source(
        &display,
        include_str!("mandelbulb.glslv"),
        include_str!("mandelbulb.glslf"),
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

        speed = (distance_mandelbulb(position[0], position[1], position[2]) * 0.02).min(0.01).max(0.0000000001);

        let mut frame = display.draw();
        
        let res = display.get_framebuffer_dimensions(); 

        frame.draw(&vertex_buffer,
                   &index::NoIndices(index::PrimitiveType::TrianglesList),
                   &program,
                   &uniform! {iResolution: [res.0, res.1],
                              camPosition: position,
                              camRotation: [yaw, pitch]},
                   &Default::default()).unwrap();

        frame.finish().unwrap();

        match move_dir {
            1 => {
                position = [(position[0] + yaw.sin() * speed * pitch.cos()).min(5.0).max(-5.0), (position[1] + pitch.sin() * speed).min(5.0).max(-5.0), (position[2] + yaw.cos() * speed * pitch.cos()).min(5.0).max(-5.0)];
            },
            2 => {

            },
            -1 => {
                position = [(position[0] - yaw.sin() * speed * pitch.cos()).min(5.0).max(-5.0), (position[1] - pitch.sin() * speed).min(5.0).max(-5.0), (position[2] - yaw.cos() * speed * pitch.cos()).min(5.0).max(-5.0)];
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
                        Some(VirtualKeyCode::Escape) => {
                            display.gl_window().window().grab_cursor(false).unwrap();
                            display.gl_window().window().hide_cursor(false);
                            move_mouse = false;
                        },
                        _ => (),
                    },
                    DeviceEvent::MouseMotion {delta} => {
                        if move_mouse {
                            yaw += delta.0 as f32 * 0.001;
                            pitch += delta.1 as f32 * 0.001;
                        }
                    },
                    DeviceEvent::Button {..} => {
                        display.gl_window().window().grab_cursor(true).unwrap();
                        display.gl_window().window().hide_cursor(true);
                        move_mouse = true;
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        let t2 = time::Instant::now();

        let d1 = t2.duration_since(t1);

        let d2 = time::Duration::from_millis(16).checked_sub(d1);

        if let Some(d3) = d2 {
            thread::sleep(d3);
            //println!("This frame took: {:?}", d3);
        } else {
            //println!("This frame took: {:?}", d1);
        }
    }
}
