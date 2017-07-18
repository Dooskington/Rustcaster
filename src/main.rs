extern crate sdl2;
extern crate time;

use std::path::*;
use sdl2::pixels::*;
use sdl2::surface::*;
use sdl2::image::*;

pub const WINDOW_TITLE: &'static str = "Rustcaster";
pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 480;
pub const RENDER_WIDTH: u32 = 640;
pub const RENDER_HEIGHT: u32 = 480;

pub const MAP_WIDTH: u32 = 30;
pub const MAP_HEIGHT: u32 = 30;
pub const MAP_SIZE: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

pub const TEXTURE_WIDTH: u32 = 64;
pub const TEXTURE_HEIGHT: u32 = 64;
pub const TEXTURE_SIZE: usize = (TEXTURE_WIDTH * TEXTURE_HEIGHT) as usize;

pub const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
pub const FIELD_OF_VIEW: f64 = 90.0 * (std::f64::consts::PI / 180.0);

pub const COLOR_BLACK: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const COLOR_MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };

pub const MAP: [u32; MAP_SIZE] =
   [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,1,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,1,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,1,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,0,0,0,1,1,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,
    1,1,0,0,0,1,1,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,1,
    1,1,0,0,0,1,1,0,0,0,0,1,0,0,0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,1,
    1,1,0,0,0,1,1,0,0,0,0,1,0,0,0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,1,
    1,1,0,0,0,1,1,0,0,0,0,1,1,1,0,1,1,1,0,1,1,1,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,1,1,0,1,1,1,0,0,0,0,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,1,1,0,1,1,1,0,0,0,0,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,1,1,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,1,
    1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1];

struct RayIntersection {
    pub x: f64,
    pub y: f64,
    pub cell_x: i32,
    pub cell_y: i32,
    pub cell_side: u8,
    pub distance: f64
}

struct Entity {
    pub x: f64,
    pub y: f64,
    pub texture_id: u32
}

impl Entity {
    pub fn new(x: f64, y: f64, texture_id: u32) -> Entity {
        Entity {
            x: x,
            y: y,
            texture_id: texture_id
        }
    }
}

struct Texture {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
    pub pitch: u32
}

impl Texture {
    pub fn load(texture_id: u32, file_name: &str) -> Texture {
        let path = Path::new(file_name);
        let surface = Surface::from_file(path).unwrap();
        let mut pixels: Vec<Color> = Vec::with_capacity((surface.width() * surface.height()) as usize);
        for i in 0..(surface.width() * surface.height()) as usize {
            pixels.push(COLOR_MAGENTA);
        }

        let bytes_per_pixel = surface.pixel_format_enum().byte_size_per_pixel();
        println!("Loading texture {} with {} bytes per pixel...", file_name, bytes_per_pixel);

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

        Texture {
            texture_id: texture_id,
            width: surface.width(),
            height: surface.height(),
            pixels: pixels,
            pitch: surface.pitch()
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.pixels[((y * self.width) + x) as usize]
    }
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

    canvas.set_draw_color(COLOR_BLACK);

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

    // Setup
    let projection_plane_width: f64 = RENDER_WIDTH as f64;
    let projection_plane_height: f64 = RENDER_HEIGHT as f64;
    let projection_plane_distance: f64 = (projection_plane_width / 2.0) / f64::tan(FIELD_OF_VIEW / 2.0);

    let move_speed: f64 = 4.0;
    let rotation_speed: f64 = 145.0;
    let rotation_speed_radians: f64 = rotation_speed * (TWO_PI / 180.0) as f64;
    let mut player_x: f64 = 3.5;
    let mut player_y: f64 = 10.5;
    let mut player_rotation: f64 = 0.0;

    let mut debug: bool = false;

    let mut textures: Vec<Texture> = Vec::new();
    textures.push(Texture::load(0, "barrel-0.png"));
    textures.push(Texture::load(1, "statue-0.png"));
    textures.push(Texture::load(2, "shane-transparent.png"));
    textures.push(Texture::load(3, "gravestone.png"));
    textures.push(Texture::load(4, "wall-stone.png"));
    textures.push(Texture::load(5, "floor-tile.png"));
    textures.push(Texture::load(6, "ceiling-tile.png"));
    textures.push(Texture::load(7, "fists.png"));

    let mut entities: Vec<Entity> = Vec::new();
    entities.push(Entity::new(2.5, 2.5, 2));
    entities.push(Entity::new(4.5, 7.5, 1));
    entities.push(Entity::new(2.5, 7.5, 1));
    entities.push(Entity::new(2.5, 10.5, 0));

    // TODO:
    // Use texture IDs instead, to prevent copying?

    let mut depth_buffer: [f64; RENDER_WIDTH as usize] = [-1.0; RENDER_WIDTH as usize];

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
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    debug = true;
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

        let rotation_speed_correct = rotation_speed_radians * delta_time;

        // Tick
        if input_up
        {
            let new_player_x = player_x + ((f64::cos(player_rotation) * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y + ((f64::sin(player_rotation) * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
        }
        if input_down
        {
            let new_player_x = player_x - ((f64::cos(player_rotation) * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y - ((f64::sin(player_rotation) * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
        }
        if input_q
        {
            let new_player_x = player_x - ((f64::cos(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y - ((f64::sin(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
        }
        if input_e
        {
            let new_player_x = player_x + ((f64::cos(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y + ((f64::sin(player_rotation + (std::f64::consts::PI / 2.0)) * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
        }
        if input_left {
            player_rotation -= rotation_speed_correct;
            player_rotation = wrap_angle(player_rotation);
        }
        if input_right {
            player_rotation += rotation_speed_correct;
            player_rotation = wrap_angle(player_rotation);
        }

        last_tick_time = current_time;

        // Render
        if render_timer >= sixty_hz {
            render_timer = render_timer - sixty_hz;

            canvas.clear();

            render_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for x in 0..(RENDER_WIDTH as usize) {
                    // Where on the screen the ray goes through
                    let ray_screen_x: f64 = -(RENDER_WIDTH as f64) / 2.0 + x as f64;

                    // The distance from the viewer to the point on the screen
                    let ray_view_dist = f64::sqrt((ray_screen_x * ray_screen_x) + (projection_plane_distance * projection_plane_distance));

                    let ray_angle: f64 = f64::asin(ray_screen_x / ray_view_dist) + player_rotation;
                    let intersection: RayIntersection = cast_ray(player_x, player_y, ray_angle);

                    let hit_distance = intersection.distance.sqrt() * f64::cos(player_rotation - ray_angle);

                    let tile_side = intersection.cell_side;

                    let tile_size = 1.0;

                    // Store the distance in the depth buffer
                    depth_buffer[x] = hit_distance;

                    // Calculate the position and height of the wall strip.
                    // The wall height is 1 unit, the distance from the player to the screen is viewDist,
                    // thus the height on the screen is equal to
                    // wallHeight * viewDist / dist
                    let line_height: i32 = f64::round((tile_size * projection_plane_distance) / hit_distance) as i32;
                    let line_screen_start: i32 = (RENDER_HEIGHT as i32 / 2) - (line_height / 2);
                    let line_screen_end: i32 = line_screen_start + line_height;

                    let ref wall_texture: Texture = textures[4];
                    let wall_texture_x: u32 = if tile_side == 0 {
                        f64::round(((intersection.y - (intersection.cell_y as f64 * tile_size)) % tile_size) * (wall_texture.width - 1) as f64) as u32
                    } else {
                        f64::round(((intersection.x - (intersection.cell_x as f64 * tile_size)) % tile_size) * (wall_texture.width - 1) as f64) as u32
                    };

                    for y in 0..(RENDER_HEIGHT as usize) {
                        // pitch is WIDTH * bytes per pixel
                        let offset = (y * pitch) + (x * 4);

                        if (y as i32) < line_screen_start {
                            // Ceiling casting

                            let player_height: f64 = 0.5;
                            let ceiling_row: i32 = (y as i32) - (RENDER_HEIGHT as i32 / 2);

                            let ceiling_straight_distance = (player_height / ceiling_row as f64) * projection_plane_distance;
                            let angle_beta_radians = player_rotation - ray_angle;

                            let floor_actual_distance = ceiling_straight_distance / f64::cos(angle_beta_radians);

                            let mut ceiling_hit_x: f64 = player_x - (floor_actual_distance * f64::cos(ray_angle));
                            let mut ceiling_hit_y: f64 = player_y - (floor_actual_distance * f64::sin(ray_angle));

                            ceiling_hit_x -= ceiling_hit_x.floor();
                            ceiling_hit_y -= ceiling_hit_y.floor();

                            let ref texture: Texture = textures[6];
                            let texture_x: u32 = f64::floor(ceiling_hit_x * (texture.width - 1) as f64) as u32;
                            let texture_y: u32 = f64::floor(ceiling_hit_y * (texture.height - 1) as f64) as u32;

                            // TODO: We are getting these textures for every pixel... can't we cache them somehow?
                            let pixel = texture.get_pixel(texture_x, texture_y);

                            buffer[offset] = pixel.a;
                            buffer[offset + 1] = pixel.b;
                            buffer[offset + 2] = pixel.g;
                            buffer[offset + 3] = pixel.r;
                        }
                        else if ((y as i32) >= line_screen_start) && ((y as i32) < line_screen_end) {
                            // Wall casting

                            let line_y: i32 = y as i32 - line_screen_start;
                            let texture_y: u32 = f64::floor((line_y as f64 / line_height as f64) * (wall_texture.height - 1) as f64) as u32;
                            let pixel = wall_texture.get_pixel(wall_texture_x, texture_y);

                            buffer[offset] = pixel.a;
                            buffer[offset + 1] = if tile_side == 1 { pixel.b } else { pixel.g / 2 };
                            buffer[offset + 2] = if tile_side == 1 { pixel.g } else { pixel.b / 2 };
                            buffer[offset + 3] = if tile_side == 1 { pixel.r } else { pixel.r / 2 };
                        }
                        else if (y as i32) >= line_screen_end {
                            // Floor casting

                            let player_height: f64 = 0.5;
                            let floor_row: i32 = (y as i32) - (RENDER_HEIGHT as i32 / 2);

                            let floor_straight_distance = (player_height / floor_row as f64) * projection_plane_distance;
                            let angle_beta_radians = player_rotation - ray_angle;

                            let floor_actual_distance = floor_straight_distance / f64::cos(angle_beta_radians);

                            let mut floor_hit_x: f64 = player_x + (floor_actual_distance * f64::cos(ray_angle));
                            let mut floor_hit_y: f64 = player_y + (floor_actual_distance * f64::sin(ray_angle));

                            floor_hit_x -= floor_hit_x.floor();
                            floor_hit_y -= floor_hit_y.floor();

                            let ref texture: Texture = textures[5];
                            let texture_x: u32 = f64::floor(floor_hit_x * (texture.width - 1) as f64) as u32;
                            let texture_y: u32 = f64::floor(floor_hit_y * (texture.height - 1) as f64) as u32;
                            let pixel = texture.get_pixel(texture_x, texture_y);

                            buffer[offset] = pixel.a;
                            buffer[offset + 1] = pixel.b;
                            buffer[offset + 2] = pixel.g;
                            buffer[offset + 3] = pixel.r;
                        }
                        else {
                            let pixel = COLOR_MAGENTA;
                            buffer[offset] = pixel.a;
                            buffer[offset + 1] = pixel.b;
                            buffer[offset + 2] = pixel.g;
                            buffer[offset + 3] = pixel.r;
                        }
                    }
                }

                // Sort entities (far to near)
                entities.sort_by(|a, b| {
                    let a_distance: f64 = (a.x - player_x).powi(2) + (a.y - player_y).powi(2);
                    let b_distance: f64 = (b.x - player_x).powi(2) + (b.y - player_y).powi(2);

                    b_distance.partial_cmp(&a_distance).unwrap()
                });

                // Render entities
                for entity in entities.iter() {
                    let distance_x: f64 = entity.x - player_x;
                    let distance_y: f64 = entity.y - player_y;

                    // The angle between the player and the sprite
                    let mut theta: f64 = f64::atan2(distance_y, distance_x);
                    theta = wrap_angle(theta);

                    // The angle between the player and the sprite, relative to the player rotation
                    let mut gamma: f64 = theta - player_rotation;
                    gamma = wrap_angle(gamma);

                    let sprite_distance: f64 = f64::sqrt(distance_x.powi(2) + distance_y.powi(2)) * f64::cos(player_rotation - theta);

                    // The number of pixels to offset from the center of the screen
                    let sprite_pixel_offset: f64 = f64::tan(gamma) * projection_plane_distance;
                    let sprite_screen_x: i32 = f64::round((RENDER_WIDTH as f64 / 2.0) + sprite_pixel_offset) as i32;

                    let sprite_height: i32 = (f64::round(projection_plane_distance / sprite_distance) as i32).abs();
                    let sprite_width: i32 = (f64::round(projection_plane_distance / sprite_distance) as i32).abs();

                    let sprite_screen_start_x: i32 = sprite_screen_x - (sprite_width / 2);
                    let sprite_screen_end_x: i32 = sprite_screen_x + (sprite_width / 2);
                    let sprite_screen_start_y: i32 = -(sprite_height / 2) + (RENDER_HEIGHT as i32 / 2);
                    let sprite_screen_end_y: i32 = (sprite_height / 2) + (RENDER_HEIGHT as i32 / 2);

                    let mut camera_min_angle: f64 = -FIELD_OF_VIEW / 2.0;
                    camera_min_angle = wrap_angle(camera_min_angle);

                    let mut camera_max_angle: f64 = FIELD_OF_VIEW / 2.0;
                    camera_max_angle = wrap_angle(camera_max_angle);

                    let ref texture: Texture = textures[entity.texture_id as usize];

                    for sprite_screen_row in sprite_screen_start_x..sprite_screen_end_x {
                        if (sprite_screen_row < 0) || (sprite_screen_row >= WINDOW_WIDTH as i32) {
                            continue;
                        }

                        // If the sprite is not visible, don't render it.
                        if ((gamma < camera_min_angle) && (gamma > camera_max_angle)) ||
                            (depth_buffer[sprite_screen_row as usize] < sprite_distance) {
                            continue;
                        }

                        for sprite_screen_col in sprite_screen_start_y..sprite_screen_end_y {
                            if (sprite_screen_col < 0) || (sprite_screen_col >= WINDOW_HEIGHT as i32) {
                                continue;
                            }

                            let sprite_row = sprite_screen_row - sprite_screen_start_x;
                            let sprite_col = sprite_screen_col - sprite_screen_start_y;

                            let texture_x: u32 = f64::round((sprite_row as f64 / sprite_width as f64) * (texture.width - 1) as f64) as u32;
                            let texture_y: u32 = f64::round((sprite_col as f64 / sprite_height as f64) * (texture.height - 1) as f64) as u32;

                            let offset = ((sprite_screen_col * pitch as i32) + (sprite_screen_row * 4)) as usize;
                            let pixel = texture.get_pixel(texture_x, texture_y);

                            // if pixel is transparent, don't draw it
                            if pixel.a == 0 {
                                continue;
                            }

                            buffer[offset] = pixel.a;
                            buffer[offset + 1] = pixel.b;
                            buffer[offset + 2] = pixel.g;
                            buffer[offset + 3] = pixel.r;
                        }
                    }
                }

                // Render weapon
                /*
                let ref texture: Texture = textures[7];

                let weapon_screen_start_x: i32 = (projection_plane_width as i32 / 2) - (texture.width as i32 / 2);
                let weapon_screen_end_x: i32 = (projection_plane_width as i32 / 2) + (texture.width as i32 / 2);
                let weapon_screen_start_y: i32 = projection_plane_height as i32 - texture.height as i32;
                let weapon_screen_end_y: i32 = projection_plane_height as i32;

                for weapon_row in weapon_screen_start_x..weapon_screen_end_x {
                    if (weapon_screen_start_x < 0) || (weapon_screen_end_x >= projection_plane_width as i32) {
                        continue;
                    }

                    for weapon_col in weapon_screen_start_y..weapon_screen_end_y {
                        if (weapon_col < 0) || (weapon_col >= WINDOW_HEIGHT as i32) {
                            continue;
                        }

                        let row: u32 = (weapon_row - weapon_screen_start_x) as u32;
                        let col: u32 = (weapon_col - weapon_screen_start_y) as u32;

                        //let texture_x: u32 = f64::round((sprite_row as f64 / sprite_width as f64) * (texture.width - 1) as f64) as u32;
                        //let texture_y: u32 = f64::round((sprite_col as f64 / sprite_height as f64) * (texture.height - 1) as f64) as u32;

                        let pixel = texture.get_pixel(row, col);

                        // if pixel is transparent, don't draw it
                        if pixel.a == 0 {
                            continue;
                        }

                        let offset = ((weapon_col * pitch as i32) + (weapon_row * 4)) as usize;
                        buffer[offset] = pixel.a;
                        buffer[offset + 1] = pixel.b;
                        buffer[offset + 2] = pixel.g;
                        buffer[offset + 3] = pixel.r;
                    }
                }
                */
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

fn cast_ray(origin_x: f64, origin_y: f64, angle: f64) -> RayIntersection {
    let angle = wrap_angle(angle);

    // Check the quadrant of the ray
    let is_ray_right: bool = angle > (TWO_PI * 0.75) || angle < (TWO_PI * 0.25);
    let is_ray_up: bool = angle < 0.0 || angle > std::f64::consts::PI;

    let mut hit_distance: f64 = 0.0; // Distance to tile we hit
    let mut hit_x: f64 = 0.0;
    let mut hit_y: f64 = 0.0;
    let mut hit_map_x: i32 = 0;
    let mut hit_map_y: i32 = 0;

    let mut tile: u32 = 0;
    let mut tile_side: u8 = 0; // Either 0 for vertical, or 1 for horizontal

    let tile_size: f64 = 1.0;

    // Check against vertical tile lines
    // We do this by moving to the right or left edge of the block we're standing in,
    // then moving, in 1 unit steps, horizontally. The amount we have to move vertically
    // is determined by the slope of the ray.

    let mut slope: f64 = f64::sin(angle) / f64::cos(angle);
    let mut delta_x: f64 = if is_ray_right { tile_size } else { -tile_size }; // Horizontal step amount
    let mut delta_y: f64 = delta_x * slope; // Vertical step amount

    let mut ray_position_x: f64 = if is_ray_right { f64::ceil(origin_x) } else { f64::floor(origin_x) }; // Starting horizontal position, at one of the edges of the current map tile
    let mut ray_position_y: f64 = origin_y + (ray_position_x - origin_x) * slope; // Starting vertical position. We add the small horizontal step we just made, multiplied by the slope.

    // While the ray is still inside the map
    while (ray_position_x >= 0.0) && (ray_position_x < MAP_WIDTH as f64) && (ray_position_y >= 0.0) && (ray_position_y < MAP_HEIGHT as f64) {
        let tile_map_x: i32 = f64::floor(ray_position_x + (if is_ray_right { 0.0 } else { -tile_size })) as i32;
        let tile_map_y: i32 = f64::floor(ray_position_y) as i32;
        tile = MAP[((tile_map_y * MAP_WIDTH as i32) + tile_map_x) as usize];

        if tile == 1 {
            let distance_x: f64 = ray_position_x - origin_x;
            let distance_y: f64 = ray_position_y - origin_y;
            hit_distance = (distance_x * distance_x) + (distance_y * distance_y); // the distance from the player to this point, squared.

            tile_side = 0;

            hit_map_x = tile_map_x;
            hit_map_y = tile_map_y;

            hit_x = ray_position_x;
            hit_y = ray_position_y;

            break;
        }

        ray_position_x += delta_x;
        ray_position_y += delta_y;
    }

    // Check against horizontal tile lines.
    // The only difference here is that once we hit a tile, we check if there was also one
    // found there in the vertical run. If so, we only register this hit if this distance is smaller.

    slope = f64::cos(angle) / f64::sin(angle);
    delta_y = if is_ray_up { -tile_size } else { tile_size }; // Vertical step amount
    delta_x = delta_y * slope; // Horizontal step amount

    ray_position_y = if is_ray_up { f64::floor(origin_y) } else { f64::ceil(origin_y) };
    ray_position_x = origin_x + (ray_position_y - origin_y) * slope;

    // While the ray is still inside the map
    while (ray_position_x >= 0.0) && (ray_position_x < MAP_WIDTH as f64) && (ray_position_y >= 0.0) && (ray_position_y < MAP_HEIGHT as f64) {
        let tile_map_x: i32 = f64::floor(ray_position_x) as i32;
        let tile_map_y: i32 = f64::floor(ray_position_y + (if is_ray_up { -tile_size } else { 0.0 })) as i32;
        tile = MAP[((tile_map_y * MAP_WIDTH as i32) + tile_map_x) as usize];

        if tile == 1 {
            let distance_x: f64 = ray_position_x - origin_x;
            let distance_y: f64 = ray_position_y - origin_y;
            let tile_distance = (distance_x * distance_x) + (distance_y * distance_y); // the distance from the player to this point, squared.

            if hit_distance == 0.0 || tile_distance < hit_distance {
                hit_distance = tile_distance;

                tile_side = 1;

                hit_map_x = tile_map_x;
                hit_map_y = tile_map_y;

                hit_x = ray_position_x;
                hit_y = ray_position_y;
            }

            break;
        }

        ray_position_x += delta_x;
        ray_position_y += delta_y;
    }

    RayIntersection {
        x: hit_x,
        y: hit_y,
        cell_x: hit_map_x,
        cell_y: hit_map_y,
        cell_side: tile_side,
        distance: hit_distance
    }
}
