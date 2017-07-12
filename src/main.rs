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

pub const MAP: [u32; MAP_SIZE] =
   [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,1,
    1,0,0,0,0,0,0,0,0,0,0,1,1,1,0,1,1,1,0,1,1,1,1,0,0,0,0,0,0,1,
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
    let angle_between_rays: f64 = FIELD_OF_VIEW / projection_plane_width;

    let move_speed: f64 = 5.0;
    let rotation_speed: f64 = 180.0;
    let rotation_speed_radians: f64 = rotation_speed * (TWO_PI / 180.0) as f64;
    let mut player_x: f64 = 14.5;
    let mut player_y: f64 = 22.0;
    let mut player_rotation: f64 = 0.0;
    let mut player_direction_x: f64 = 0.0;
    let mut player_direction_y: f64 = -1.0;
    let mut player_right_x: f64 = -player_direction_y;
    let mut player_right_y: f64 = -player_direction_x;
    let mut camera_plane_x: f64 = 0.66;
    let mut camera_plane_y: f64 = 0.0;
    let wall_proportion: f64 = RENDER_HEIGHT as f64 / camera_plane_x;

    let mut texture: [u32; TEXTURE_SIZE] = [0; TEXTURE_SIZE];
    let mut debug: bool = false;
    /*
    for x in 0..TEXTURE_WIDTH {
        for y in 0..TEXTURE_HEIGHT {
            let xor_color: u32 = (x * 256 / TEXTURE_WIDTH) ^ (y * 256 / TEXTURE_HEIGHT);
            texture[((y * TEXTURE_WIDTH) + x) as usize] = 256 * xor_color;
        }
    }
    */

    let path = Path::new("wall.png");
    let wall_surface = Surface::from_file(path).unwrap();
    let color_magenta = Color { r: 255, g: 0, b: 255, a: 255 };
    let mut wall_buffer: [Color; TEXTURE_SIZE] = [color_magenta; TEXTURE_SIZE];

    wall_surface.with_lock(|surface_buffer: &[u8]| {
        for x in 0..wall_surface.width() {
            for y in 0..wall_surface.height() {
                let texture_pixel_index = 
                    (x as usize * wall_surface.pitch() as usize) + 
                    (y as usize * wall_surface.pixel_format_enum().byte_size_per_pixel());

                let color = Color {
                    r: surface_buffer[texture_pixel_index],
                    g: surface_buffer[texture_pixel_index + 1],
                    b: surface_buffer[texture_pixel_index + 2],
                    a: 255
                };

                wall_buffer[(y as usize * wall_surface.width() as usize) + x as usize] = color;
            }
        }
    });

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
        let delta_time: f64 = elapsed_time.num_milliseconds() as f64 / 1000.0;
        //let total_time = current_time - start_time;
        render_timer = render_timer + elapsed_time;

        let rotation_speed_correct = rotation_speed_radians * delta_time;

        player_right_x = -player_direction_y;
        player_right_y = player_direction_x;

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
            /*
            let new_player_x = player_x - ((player_right_x * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y - ((player_right_y * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
            */
        }
        if input_e
        {
            /*
            let new_player_x = player_x + ((player_right_x * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y + ((player_right_y * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
            */
        }
        if input_left {
            player_rotation -= rotation_speed_correct;
        }
        if input_right {
            player_rotation += rotation_speed_correct;
        }

        // Clamp player_rotation
        if player_rotation < 0.0 {
            player_rotation += TWO_PI;
        }
        else if player_rotation >= TWO_PI {
            player_rotation -= TWO_PI;
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

                    let mut ray_angle: f64 = f64::asin(ray_screen_x / ray_view_dist);
                    ray_angle += player_rotation;

                    // Clamp ray_angle
                    if ray_angle < 0.0 {
                        ray_angle += TWO_PI;
                    }
                    else if ray_angle >= TWO_PI {
                        ray_angle -= TWO_PI;
                    }

                    // Check the quadrant of the ray
                    let is_ray_right: bool = ray_angle > (TWO_PI * 0.75) || ray_angle < (TWO_PI * 0.25);
                    let is_ray_up: bool = ray_angle < 0.0 || ray_angle > std::f64::consts::PI;

                    if x == (RENDER_WIDTH / 2) as usize {
                        //println!("ray_angle = {} is_ray_right = {}, is_ray_up = {}", ray_angle * (180.0 / std::f64::consts::PI), is_ray_right, is_ray_up);
                    }

                    let mut hit_distance: f64 = 0.0; // Distance to tile we hit
                    let mut hit_x: f64 = 0.0;
                    let mut hit_y: f64 = 0.0;
                    let mut hit_map_x: i32 = 0;
                    let mut hit_map_y: i32 = 0;

                    let mut map_x: i32 = 0;
                    let mut map_y: i32 = 0;

                    let mut tile_side: i8 = 0; // Either 0 for vertical, or 1 for horizontal

                    let tile_size: f64 = 1.0;

                    // Check against vertical tile lines
                    // We do this by moving to the right or left edge of the block we're standing in,
                    // then moving, in 1 unit steps, horizontally. The amount we have to move vertically
                    // is determined by the slope of the ray.

                    let mut slope: f64 = f64::sin(ray_angle) / f64::cos(ray_angle);
                    let mut delta_x: f64 = if is_ray_right { tile_size } else { -tile_size }; // Horizontal step amount
                    let mut delta_y: f64 = delta_x * slope; // Vertical step amount

                    let mut ray_position_x: f64 = if is_ray_right { f64::ceil(player_x) } else { f64::floor(player_x) }; // Starting horizontal position, at one of the edges of the current map tile
                    let mut ray_position_y: f64 = player_y + (ray_position_x - player_x) * slope; // Starting vertical position. We add the small horizontal step we just made, multiplied by the slope.

                    // While the ray is still inside the map
                    while (ray_position_x >= 0.0) && (ray_position_x < MAP_WIDTH as f64) && (ray_position_y >= 0.0) && (ray_position_y < MAP_HEIGHT as f64) {
                        let tile_map_x: i32 = f64::floor(ray_position_x + (if is_ray_right { 0.0 } else { -tile_size })) as i32;
                        let tile_map_y: i32 = f64::floor(ray_position_y) as i32;

                        if MAP[((tile_map_y * MAP_WIDTH as i32) + tile_map_x) as usize] > 0 {
                            let distance_x: f64 = ray_position_x - player_x;
                            let distance_y: f64 = ray_position_y - player_y;
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

                    slope = f64::cos(ray_angle) / f64::sin(ray_angle);
                    delta_y = if is_ray_up { -tile_size } else { tile_size }; // Vertical step amount
                    delta_x = delta_y * slope; // Horizontal step amount

                    ray_position_y = if is_ray_up { f64::floor(player_y) } else { f64::ceil(player_y) };
                    ray_position_x = player_x + (ray_position_y - player_y) * slope;

                    // While the ray is still inside the map
                    while (ray_position_x >= 0.0) && (ray_position_x < MAP_WIDTH as f64) && (ray_position_y >= 0.0) && (ray_position_y < MAP_HEIGHT as f64) {
                        let tile_map_x: i32 = f64::floor(ray_position_x) as i32;
                        let tile_map_y: i32 = f64::floor(ray_position_y + (if is_ray_up { -tile_size } else { 0.0 })) as i32;

                        if MAP[((tile_map_y * MAP_WIDTH as i32) + tile_map_x) as usize] > 0 {
                            let distance_x: f64 = ray_position_x - player_x;
                            let distance_y: f64 = ray_position_y - player_y;
                            let tile_distance = (distance_x * distance_x) + (distance_y * distance_y); // the distance from the player to this point, squared.

                            if hit_distance == 0.0 || tile_distance < hit_distance
                            {
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

                    hit_distance = f64::sqrt(hit_distance);

                    // Adjust to remove fish eye
                    hit_distance *= f64::cos(player_rotation - ray_angle);

                    // Calculate the position and height of the wall strip.
                    // The wall height is 1 unit, the distance from the player to the screen is viewDist,
                    // thus the height on the screen is equal to
                    // wallHeight * viewDist / dist
                    let line_height: i32 = f64::round((tile_size * projection_plane_distance) / hit_distance) as i32;
                    let line_screen_start: i32 = (RENDER_HEIGHT as i32 / 2) - (line_height / 2);
                    let line_screen_end: i32 = line_screen_start + line_height;

                    let texture_x: u32 = if tile_side == 0 { 
                        f64::round(((hit_y - (hit_map_y as f64 * tile_size)) % tile_size) * TEXTURE_WIDTH as f64) as u32
                    } else {
                        f64::round(((hit_x - (hit_map_x as f64 * tile_size)) % tile_size) * TEXTURE_WIDTH as f64) as u32
                    };

                    for y in 0..(RENDER_HEIGHT as usize) {
                        // pitch is WIDTH * bytes per pixel
                        let offset = (y * pitch) + (x * 3);

                        if ((y as i32) >= line_screen_start as i32) && ((y as i32) < line_screen_end as i32) {
                            let line_y: i32 = y as i32 - line_screen_start as i32;
                            let texture_y: u32 = f64::floor((line_y as f64 / line_height as f64) * (TEXTURE_HEIGHT - 1) as f64) as u32;
                            let pixel = wall_buffer[((texture_y * wall_surface.width()) + texture_x) as usize];

                            if debug
                            {
                                println!("line_height = {}, line_screen_start = {}, line_screen_end = {}, line_y = {}, texture_x = {}, texture_y = {}", line_height, line_screen_start, line_screen_end, line_y, texture_x, texture_y);
                                debug = false;
                            }
                            
                            buffer[offset] = if tile_side == 1 { pixel.r } else { pixel.r / 2 };
                            buffer[offset + 1] = if tile_side == 1 { pixel.g } else { pixel.g / 2 };
                            buffer[offset + 2] = if tile_side == 1 { pixel.b } else { pixel.b / 2 };
                        }
                        else {
                            buffer[offset] = 0 as u8;
                            buffer[offset + 1] = 0 as u8;
                            buffer[offset + 2] = 80 as u8;
                        }
                    }
                }
            }).unwrap();

            canvas.copy_ex(&render_texture, None, None, 0.0, None, false, false).unwrap();
            canvas.present();
        }
    }
}
