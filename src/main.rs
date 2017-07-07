extern crate sdl2;

use sdl2::pixels::*;

pub const WINDOW_TITLE: &'static str = "Rustcaster";
pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;

fn main() {
    // Initialize SDL2
    let sdl_context = ::sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();

    // Create window
    let sdl_window = sdl_video.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // Get the window canvas
    let mut canvas = sdl_window
        .into_canvas()
        .target_texture()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    // Create the render texture for the canvas
    let texture_creator = canvas.texture_creator();
    let mut render_texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    // Engine loop
    let mut sdl_event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in sdl_event_pump.poll_iter() {
            use sdl2::event::*;
            use sdl2::keyboard::*;

            match event {
                // If the window is closed, or ESC is pressed, exit
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas.clear();

        render_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y*pitch + x*3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        }).unwrap();

        canvas.copy_ex(&render_texture, None, None, 0.0, None, false, false).unwrap();
        canvas.present();
    }

}
