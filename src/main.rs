extern crate rustcaster;
extern crate sdl2;
extern crate time;

use std::str;
use std::path::*;
use rustcaster::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::*;
use sdl2::image::*;

pub const WINDOW_TITLE: &'static str = "Rustcaster";
pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;
pub const FIELD_OF_VIEW: f64 = 90.0;

pub const TEXTURE_ID_BARREL: u32 = 0;
pub const TEXTURE_ID_STATUE: u32 = 1;
pub const TEXTURE_ID_GRAVESTONE: u32 = 2;
pub const TEXTURE_ID_WALL: u32 = 3;
pub const TEXTURE_ID_FLOOR: u32 = 4;
pub const TEXTURE_ID_CEILING: u32 = 5;

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

    canvas.set_draw_color(sdl2::pixels::Color { r: 0, g: 0, b: 0, a: 0 });

    // Create the render texture for the canvas
    let texture_creator = canvas.texture_creator();
    let mut render_texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    let start_time = time::now();
    let mut last_tick_time = start_time;
    let mut render_timer = time::Duration::zero();
    let sixty_hz = time::Duration::nanoseconds(16666667);

    let mut input_left: bool = false;
    let mut input_right: bool = false;
    let mut input_up: bool = false;
    let mut input_down: bool = false;
    let mut input_q: bool = false;
    let mut input_e: bool = false;

    let move_speed: f64 = 6.0;
    let rotation_speed: f64 = f64::to_radians(180.0);
    let mut player_x: f64 = 1.5;
    let mut player_y: f64 = 1.5;
    let mut player_rotation: f64 = 0.0;

    let mut textures: Vec<Texture> = Vec::new();
    textures.push(load_texture(TEXTURE_ID_BARREL,     "res/barrel.png"));
    textures.push(load_texture(TEXTURE_ID_STATUE,     "res/statue.png"));
    textures.push(load_texture(TEXTURE_ID_GRAVESTONE, "res/gravestone.png"));
    textures.push(load_texture(TEXTURE_ID_WALL,       "res/wall-stone.png"));
    textures.push(load_texture(TEXTURE_ID_FLOOR,      "res/floor-tile.png"));
    textures.push(load_texture(TEXTURE_ID_CEILING,    "res/ceiling-tile.png"));

    let mut map = load_map("res/maps/dungeon.png")
    .expect("Failed to load map!");

    let mut engine = Engine::new(FIELD_OF_VIEW.to_radians(), WINDOW_WIDTH, WINDOW_HEIGHT);

    // Engine loop
    let mut sdl_event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // Calculate elapsed time
        let current_time = time::now();
        let elapsed_time = current_time - last_tick_time;
        let delta_time: f64 = (elapsed_time.num_nanoseconds().unwrap() as f64) / 1_000_000_000_f64;
        render_timer = render_timer + elapsed_time;

        for event in sdl_event_pump.poll_iter() {
            use sdl2::event::*;
            use sdl2::keyboard::*;

            match event {
                // If the window is closed, or ESC is pressed, exit
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },

                // Game input
                Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    input_left = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Left), .. } | Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                    input_left = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } | Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    input_right = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Right), .. } | Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                    input_right = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    input_q = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                    input_q = false;
                },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                    input_e = true;
                },
                Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                    input_e = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    input_up = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Up), .. } | Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                    input_up = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    input_down = true;
                },
                Event::KeyUp { keycode: Some(Keycode::Down), .. } | Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                    input_down = false;
                },

                _ => {}
            }
        }

        // Calculate velocity based on input
        let mut velocity_x: f64 = 0.0;
        let mut velocity_y: f64 = 0.0;

        if input_up {
            velocity_x += player_rotation.cos() * move_speed;
            velocity_y += player_rotation.sin() * move_speed;
        }
        if input_down {
            velocity_x -= player_rotation.cos() * move_speed;
            velocity_y -= player_rotation.sin() * move_speed;
        }
        if input_q {
            velocity_x -= f64::cos(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed;
            velocity_y -= f64::sin(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed;
        }
        if input_e {
            velocity_x += f64::cos(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed;
            velocity_y += f64::sin(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed;
        }
        if input_left {
            player_rotation = wrap_angle(player_rotation - (rotation_speed * delta_time));
        }
        if input_right {
            player_rotation = wrap_angle(player_rotation + (rotation_speed * delta_time));
        }

        // Apply velocity
        if (velocity_x != 0.0) || (velocity_y != 0.0) {
            let new_position_x = player_x + (velocity_x * delta_time);
            let new_position_y = player_y + (velocity_y * delta_time);

            if map.get_cell(new_position_x.trunc() as u32, player_y.trunc() as u32).is_none() {
                player_x = new_position_x;
            }

            if map.get_cell(player_x.trunc() as u32, new_position_y.trunc() as u32).is_none() {
                player_y = new_position_y;
            }
        }

        last_tick_time = current_time;

        // Render
        if render_timer >= sixty_hz {
            render_timer = render_timer - sixty_hz;

            canvas.clear();

            render_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                engine.render(buffer, pitch, &mut map, &textures, player_x, player_y, player_rotation);
            }).unwrap();

            canvas.copy_ex(&render_texture, None, None, 0.0, None, false, false).unwrap();
            canvas.present();
        }
    }
}

pub fn load_texture(texture_id: u32, file_name: &str) -> Texture {
    let surface = Surface::from_file(Path::new(file_name)).unwrap();
    let mut pixels: Vec<Color> = Vec::new();
    pixels.resize((surface.width() * surface.height()) as usize, COLOR_MAGENTA);

    surface.with_lock(|surface_buffer: &[u8]| {
        for x in 0..surface.width() {
            for y in 0..surface.height() {
                let texture_pixel_index =
                    (y as usize * surface.pitch() as usize) +
                    (x as usize * surface.pixel_format_enum().byte_size_per_pixel());

                let color = Color {
                    r: surface_buffer[texture_pixel_index],
                    g: surface_buffer[texture_pixel_index + 1],
                    b: surface_buffer[texture_pixel_index + 2],
                    a: surface_buffer[texture_pixel_index + 3]
                };

                pixels[((y * surface.width()) + x) as usize] = color;
            }
        }
    });

    Texture::new(texture_id, surface.width(), surface.height(), pixels)
}

pub fn load_map(file_name: &str) -> std::io::Result<Map> {
    let texture: Texture = load_texture(0, file_name);

    let mut cells: Vec<Option<Cell>> = Vec::new();
    cells.resize((texture.width * texture.height) as usize, None);

    let mut sprites: Vec<Sprite> = Vec::new();

    for x in 0..texture.width {
        for y in 0..texture.height {
            let index: usize = ((y * texture.width) + x) as usize;
            let pixel: Color = texture.pixels[index];

            if pixel == COLOR_BLACK {
                let cell: Cell = Cell {x: x as u32, y: y as u32, texture_id: TEXTURE_ID_WALL}; // Wall
                cells[index] = Some(cell);
            }
            else if pixel == COLOR_RED {
                sprites.push(Sprite::new(x as f64 + 0.5, y as f64 + 0.5, TEXTURE_ID_STATUE));
            }
            else if pixel == COLOR_GREEN {
                sprites.push(Sprite::new(x as f64 + 0.5, y as f64 + 0.5, TEXTURE_ID_BARREL));
            }
            else if pixel == COLOR_BLUE {
                sprites.push(Sprite::new(x as f64 + 0.5, y as f64 + 0.5, TEXTURE_ID_GRAVESTONE));
            }
        }
    }

    Ok(Map {
        width: texture.width,
        height: texture.height,
        floor_texture_id: TEXTURE_ID_FLOOR,
        ceiling_texture_id: TEXTURE_ID_CEILING,
        cells: cells,
        sprites: sprites
    })
}
