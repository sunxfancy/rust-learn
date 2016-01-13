extern crate sdl2;
use std::thread;
use sdl2::event::{Event};
use sdl2::rect::{Rect};
use sdl2::render::{Renderer};

fn draw(renderer: &mut Renderer) {
    // Set the drawing color to a light blue.
    renderer.set_draw_color(sdl2::pixels::Color::RGB(101, 208, 246));

    // Clear the buffer, using the light blue color set above.
    renderer.clear();

    // Set the drawing color to a darker blue.
    renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 153, 204));

    // Create centered Rect, draw the outline of the Rect in our dark blue color.
    let border_rect = Rect::new(320-64, 240-64, 128, 128).unwrap().unwrap();
    renderer.draw_rect(border_rect);

    // Create a smaller centered Rect, filling it in the same dark blue.
    let inner_rect = Rect::new(320-60, 240-60, 120, 120).unwrap().unwrap();
    renderer.fill_rect(inner_rect);

    // Swap our buffer for the present buffer, displaying it.
    renderer.present();
}


fn main() {
    let ctx = sdl2::init().ok().expect("Failed to init SDL2");
    let video_ctx = ctx.video().unwrap();

    // Create a window
    let window = match video_ctx.window("Hello-SDL2", 640, 480)
        .position_centered().opengl().build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    // Create a rendering context
    let mut renderer = match window.renderer().build() {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };

    // draw something on the renderer
    draw(&mut renderer);

    let mut events = ctx.event_pump().unwrap();

    // loop until we receive a QuitEvent
    'event : loop {
        // poll_event returns the most recent event or NoEvent if nothing has happened
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _               => continue
            }
        }
        thread::sleep_ms(1);
    }
}
