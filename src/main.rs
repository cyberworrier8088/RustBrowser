use winit::{event_loop::EventLoop, window::WindowBuilder};
use pixels::{Pixels, SurfaceTexture};

fn main() {
    // event 
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).expect("Failed to build window");
    
    // init
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(800, 600, surface_texture).expect("Pixels failed")
    };

    // 3. Modern winit event loop syntax
    event_loop.run(move |event, _, control_flow| {
        // ---------
    });
}