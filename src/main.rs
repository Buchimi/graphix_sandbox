#[macro_use]
extern crate glium;

use glium::glutin::event_loop::EventLoop;
use glium::glutin::{event::WindowEvent, event_loop::ControlFlow};
use glium::{implement_vertex, Display, Surface};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);
fn setup() -> (EventLoop<()>, Display) {
    // 1. The **winit::EventsLoop** for handling events.
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    (event_loop, display)
}

fn create_program(display: &Display) -> glium::Program {
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform float x;
        void main() {
            vec2 pos = position;
            pos.x += x;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;
    let fragment_shader_src = r#"
    #version 140

    out vec4 color;
    uniform float x;
    void main() {
        
        color = vec4(1.0 * x, 1.0 * x * 0.5, 1 * (x * 0.2), 1.0);
    }
"#;
    let program =
        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();
    program
}
fn main() {

    let (event_loop, display) = setup();
    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = create_program(&display);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);

    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();

    frame.finish().unwrap();
    let mut t: f32 = 0.0;
    
    event_loop.run(move |event, _window_target, control_flow|{ 
        
        match event {
        glium::glutin::event::Event::WindowEvent {
            window_id: _,
            event,
        } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        },
        glium::glutin::event::Event::RedrawEventsCleared =>{
            display.gl_window().window().request_redraw();
        }, 
        glium::glutin::event::Event::RedrawRequested(_) =>{
            t += 0.002;
            // We use the sine of t as an offset, this way we get a nice smooth animation
            let x_off = t.sin() * 0.5;

            // let shape = vec![
            //     Vertex { position: [-0.5 + x_off, -0.5] },
            //     Vertex { position: [ 0.0 + x_off,  0.5] },
            //     Vertex { position: [ 0.5 + x_off, -0.25] }
            // ];
            let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(&vertex_buffer, &indices, &program, &uniform! {x : x_off},
                    &Default::default()).unwrap();
            target.finish().unwrap();

        }
        _ => (),
    }})
}
