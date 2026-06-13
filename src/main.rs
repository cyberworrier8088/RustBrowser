// src/main.rs :)

// this is a browser made from scratch
// this made for learning rust and how working web browser
// this is not for production (*_*)
// this is for fun
// enjoy my code :)

//////////////////////
// top of file
/////////////////////

// import modules from other files
mod app;
mod dom;
mod net;
mod render;



// useing libraries
use app::App;
use pixels::{Pixels, SurfaceTexture};
use render::{HEIGHT, WIDTH};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    window::WindowBuilder,
};


//  main function
fn main() {
    println!("Creating EventLoop...");
    let event_loop = EventLoop::new().unwrap();

    println!("Initializing Window and Pixels...");
    let window = WindowBuilder::new()
        .with_title("Mini Browser")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let pixels = {
        let window_size = window.inner_size();
        let surface = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface).unwrap()
    };

    let mut app = App::new(window, pixels, "https://example.com");

    app.new_tab(r"src/main.html");

    println!("Running app...");
    event_loop
        .run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        app.mouse_x = position.x as i32;
                        app.mouse_y = position.y as i32;
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        app.click_link();
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        app.scroll(delta);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            handle_key(&mut app, event.logical_key);
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        app.draw();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    app.window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}

// handle keyboard input
fn handle_key(app: &mut App, key: Key) {
    if app.typing {
        match key {
            Key::Named(NamedKey::Enter) => app.submit_url(),
            Key::Named(NamedKey::Escape) => app.cancel_typing(),
            Key::Named(NamedKey::Backspace) => {
                app.typing_url.pop();
            }
            Key::Character(text) => {
                if text != "\r" && text != "\n" {
                    app.typing_url.push_str(&text);
                }
            }
            _ => {}
        }

        return;
    }

    match key {

        // this for user type  "/" key to open address bar and type
        Key::Character(text) if text == "/" => app.start_typing(),

        // this for go back for user can easly go back page
        Key::Named(NamedKey::ArrowLeft) => {
            app.go_back();
        }

        // this for go forward for user can easly go forward page
        Key::Named(NamedKey::ArrowRight) => {
            app.go_forward();
        }

        _ => {}
    }
}






/////////////////////////
// End of file
////////////////////////