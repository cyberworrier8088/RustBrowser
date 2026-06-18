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
mod css;
mod dom;
mod downloads;
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

    let mut app = App::new(window, pixels, "https://google.com");

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
                        if app.selecting {
                            if app.selection_start.1 >= render::ADDRESS_BAR_HEIGHT {
                                app.selection_end = (app.mouse_x, app.mouse_y.max(render::ADDRESS_BAR_HEIGHT));
                                app.update_selection();
                                println!("Selection Updated");
                            } else {
                                app.selection_end = (app.mouse_x, app.mouse_y);
                            }
                        }
                    }
                    WindowEvent::MouseInput {
                        state,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if state == ElementState::Pressed {
                            app.selecting = true;
                            app.selection_active = false;
                            app.selected_text.clear();
                            app.selection_start = (app.mouse_x, app.mouse_y);
                            app.selection_end = (app.mouse_x, app.mouse_y);
                            if app.selection_start.1 >= render::ADDRESS_BAR_HEIGHT {
                                println!("Selection Started");
                            }
                        } else if state == ElementState::Released {
                            if app.selecting {
                                app.selecting = false;
                                let dx = (app.selection_start.0 - app.selection_end.0).abs();
                                let dy = (app.selection_start.1 - app.selection_end.1).abs();
                                if dx * dx + dy * dy < 25 {
                                    app.selection_active = false;
                                    app.selected_text.clear();
                                    app.click_link();
                                } else {
                                    if app.selection_start.1 >= render::ADDRESS_BAR_HEIGHT {
                                        app.selection_active = true;
                                        app.update_selection();
                                        println!("Selection Finished");
                                        println!("Selected text: {}", app.selected_text);
                                    } else {
                                        app.selection_active = false;
                                        app.selected_text.clear();
                                    }
                                }
                            }
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        app.scroll(delta);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        let is_pressed = event.state == ElementState::Pressed;
                        
                        match &event.logical_key {
                            Key::Named(NamedKey::Control) => {
                                app.ctrl_pressed = is_pressed;
                            }
                            _ => {}
                        }

                        if is_pressed {
                            let mut handled = false;
                            if app.ctrl_pressed {
                                match &event.logical_key {
                                    Key::Character(text) if text.to_lowercase() == "c" || text == "\u{3}" => {
                                        app.copy_selection_to_clipboard();
                                        handled = true;
                                    }
                                    _ => {}
                                }
                            }
                            if !handled {
                                handle_key(&mut app, event.logical_key);
                            }
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

        // this for bookmark for user can easly bookmark page
        Key::Character(text) if text == "b" => {
            app.add_bookmark();
        }

        _ => {}
    }
}






/////////////////////////
// End of file
////////////////////////