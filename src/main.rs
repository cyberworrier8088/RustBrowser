mod app;
mod dom;
mod net;
mod render;

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

    app.new_tab("https://www.rust-lang.org");

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
        Key::Character(text) if text == "/" => app.start_typing(),

        Key::Named(NamedKey::ArrowLeft) => {
            app.go_back();
        }

        Key::Named(NamedKey::ArrowRight) => {
            app.go_forward();
        }

        _ => {}
    }
}
