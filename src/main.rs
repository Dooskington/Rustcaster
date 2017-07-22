extern crate rustcaster;
extern crate sdl2;
extern crate time;

use std::path::*;
use rustcaster::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::*;
use sdl2::image::*;

pub const WINDOW_TITLE: &'static str = "Rustcaster";
pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;
pub const FIELD_OF_VIEW: f64 = 90.0;

pub fn load_texture(texture_id: u32, file_name: &str) -> Texture {
    let path_str: &str = &format!("res/{}", file_name);
    let path = Path::new(path_str);
    let surface = Surface::from_file(path).unwrap();

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

    Texture::new(texture_id, surface.width(), surface.height(), &pixels)
}

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
    let rotation_speed: f64 = f64::to_radians(145.0);
    let mut player_x: f64 = 1.5;
    let mut player_y: f64 = 1.5;
    let mut player_rotation: f64 = 0.0;

    let mut textures: Vec<Texture> = Vec::new();
    textures.push(load_texture(0, "barrel-0.png"));
    textures.push(load_texture(1, "statue-0.png"));
    textures.push(load_texture(2, "shane-transparent.png"));
    textures.push(load_texture(3, "gravestone.png"));
    textures.push(load_texture(4, "wall-stone.png"));
    textures.push(load_texture(5, "floor-tile.png"));
    textures.push(load_texture(6, "ceiling-tile.png"));
    textures.push(load_texture(7, "fists.png")); 

    let mut sprites: Vec<Sprite> = Vec::new();
    sprites.push(Sprite::new(2.5, 2.5, 2));
    sprites.push(Sprite::new(4.5, 7.5, 1));
    sprites.push(Sprite::new(2.5, 7.5, 1));
    sprites.push(Sprite::new(2.5, 10.5, 0));

    let mut map: Vec<Option<Cell>> = Vec::new();
    map.resize(MAP_SIZE * MAP_SIZE, None);
    
    // Construct map
    for x in 0..MAP_SIZE {
        for y in 0..MAP_SIZE {
            if x == 0 || x == (MAP_SIZE - 1) || y == 0 || y == (MAP_SIZE - 1) {
                let cell: Cell = Cell {x: x as i32, y: y as i32, texture_id: 4};
                map[((y * MAP_SIZE) + x)] = Some(cell);
            }
        }
    }

    let mut engine = Engine::new(FIELD_OF_VIEW.to_radians(), WINDOW_WIDTH, WINDOW_HEIGHT);

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

        // Calculate elapsed time
        let current_time = time::now();
        let elapsed_time = current_time - last_tick_time;
        let delta_time: f64 = (elapsed_time.num_nanoseconds().unwrap() as f64) / 1_000_000_000_f64;
        let total_time = current_time - start_time;
        render_timer = render_timer + elapsed_time;

        // Tick
        if input_up
        {
            let new_player_x = player_x + ((f64::cos(player_rotation) * move_speed) * delta_time);
            let next_tile_x = map[((player_y as usize * MAP_SIZE) + new_player_x as usize)];
            if next_tile_x.is_none() {
                player_x = new_player_x;
            }

            let new_player_y = player_y + ((f64::sin(player_rotation) * move_speed) * delta_time);
            let next_tile_y = map[((new_player_y as usize * MAP_SIZE) + player_x as usize)];
            if next_tile_y.is_none() {
                player_y = new_player_y;
            }
        }
        if input_down
        {
            let new_player_x = player_x - ((f64::cos(player_rotation) * move_speed) * delta_time);
            let next_tile_x = map[((player_y as usize * MAP_SIZE) + new_player_x as usize)];
            if next_tile_x.is_none() {
                player_x = new_player_x;
            }

            let new_player_y = player_y - ((f64::sin(player_rotation) * move_speed) * delta_time);
            let next_tile_y = map[((new_player_y as usize * MAP_SIZE) + player_x as usize)];
            if next_tile_y.is_none() {
                player_y = new_player_y;
            }
        }
        if input_q
        {
            let new_player_x = player_x - ((f64::cos(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_x = map[((player_y as usize * MAP_SIZE) + new_player_x as usize)];
            if next_tile_x.is_none() {
                player_x = new_player_x;
            }

            let new_player_y = player_y - ((f64::sin(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_y = map[((new_player_y as usize * MAP_SIZE) + player_x as usize)];
            if next_tile_y.is_none() {
                player_y = new_player_y;
            }
        }
        if input_e
        {
            let new_player_x = player_x + ((f64::cos(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_x = map[((player_y as usize * MAP_SIZE) + new_player_x as usize)];
            if next_tile_x.is_none() {
                player_x = new_player_x;
            }

            let new_player_y = player_y + ((f64::sin(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_y = map[((new_player_y as usize * MAP_SIZE) + player_x as usize)];
            if next_tile_y.is_none() {
                player_y = new_player_y;
            }
        }
        if input_left {
            player_rotation = wrap_angle(player_rotation - (rotation_speed * delta_time));
        }
        if input_right {
            player_rotation = wrap_angle(player_rotation + (rotation_speed * delta_time));
        }

        last_tick_time = current_time;

        // Render
        if render_timer >= sixty_hz {
            render_timer = render_timer - sixty_hz;

            canvas.clear();

            render_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                engine.render(buffer, &map, &sprites, &textures, player_x, player_y, player_rotation);
            }).unwrap();

            canvas.copy_ex(&render_texture, None, None, 0.0, None, false, false).unwrap();
            canvas.present();
        }
    }
}

fn wrap_angle(angle: f64) -> f64 {
    if angle < 0.0 {
        return angle + TWO_PI;
    }
    else if angle >= TWO_PI {
        return angle - TWO_PI;
    }

    angle
}