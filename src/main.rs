extern crate sdl2;
extern crate time;

use sdl2::pixels::*;

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

pub const TWO_PI: f32 = (2_f64 * std::f64::consts::PI) as f32;
pub const FIELD_OF_VIEW: f32 = (90_f64 * (std::f64::consts::PI / 180_f64)) as f32;

pub const MAP: [u32; MAP_SIZE] = 
   [2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,3,1,1,1,1,1,3,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,3,1,1,0,1,1,3,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2];

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
    let move_speed: f32 = 5.0;
    let rotation_speed: f32 = 180.0;
    let rotation_speed_radians: f32 = rotation_speed * (TWO_PI / 180.0);
    let mut player_x: f32 = 14.5;
    let mut player_y: f32 = 22.0;
    let mut player_direction_x: f32 = 0.0;
    let mut player_direction_y: f32 = -1.0;
    let mut player_right_x: f32 = -player_direction_y;
    let mut player_right_y: f32 = -player_direction_x;
    let mut camera_plane_x: f32 = 1.0;
    let mut camera_plane_y: f32 = 0.0;

    let mut texture: [u32; TEXTURE_SIZE] = [0; TEXTURE_SIZE];
    for x in 0..TEXTURE_WIDTH { 
        for y in 0..TEXTURE_HEIGHT {
            let xor_color: u32 = (x * 256 / TEXTURE_WIDTH) ^ (y * 256 / TEXTURE_HEIGHT);
            texture[((y * TEXTURE_WIDTH) + x) as usize] = 256 * xor_color;
        }
    }
    
    // DEBUG: set the start and end of the texture to white and red
    //texture[0] = 16777215;
    //texture[TEXTURE_WIDTH as usize - 1] = 16711680;

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
        let delta_time: f32 = elapsed_time.num_milliseconds() as f32 / 1000.0;
        let total_time = current_time - start_time;
        render_timer = render_timer + elapsed_time;
        //let view_distance: f32 = ((RENDER_WIDTH / 2) as f32) / f32::tan(FIELD_OF_VIEW / 2_f32);

        let rotation_speed_correct = rotation_speed_radians * delta_time;

        player_right_x = -player_direction_y;
        player_right_y = player_direction_x;

        // Tick
        if input_up
        {
            let new_player_x = player_x + ((player_direction_x * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y + ((player_direction_y * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
        }
        if input_down
        {
            let new_player_x = player_x - ((player_direction_x * move_speed) * delta_time);
            let next_tile_x = MAP[((player_y as usize * MAP_WIDTH as usize) + new_player_x as usize)];
            if next_tile_x == 0 {
                player_x = new_player_x;
            }

            let new_player_y = player_y - ((player_direction_y * move_speed) * delta_time);
            let next_tile_y = MAP[((new_player_y as usize * MAP_WIDTH as usize) + player_x as usize)];
            if next_tile_y == 0 {
                player_y = new_player_y;
            }
        }
        if input_q
        {
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
        }
        if input_e
        {
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
        }
        if input_left
        {
            // Rotating the vectors by multiplying them with the rotation matrix
            // [ cos(a) -sin(a) ]
            // [ sin(a)  cos(a) ]

            let old_direction_x = player_direction_x;
            player_direction_x = (player_direction_x * f32::cos(-rotation_speed_correct)) - (player_direction_y * f32::sin(-rotation_speed_correct));
            player_direction_y = (old_direction_x * f32::sin(-rotation_speed_correct)) + (player_direction_y * f32::cos(-rotation_speed_correct));

            let old_plane_x = camera_plane_x;
            camera_plane_x = (camera_plane_x * f32::cos(-rotation_speed_correct)) - (camera_plane_y * f32::sin(-rotation_speed_correct));
            camera_plane_y = (old_plane_x * f32::sin(-rotation_speed_correct)) + (camera_plane_y * f32::cos(-rotation_speed_correct));
        }
        if input_right
        {
            let old_direction_x = player_direction_x;
            player_direction_x = (player_direction_x * f32::cos(rotation_speed_correct)) - (player_direction_y * f32::sin(rotation_speed_correct));
            player_direction_y = (old_direction_x * f32::sin(rotation_speed_correct)) + (player_direction_y * f32::cos(rotation_speed_correct));

            let old_plane_x = camera_plane_x;
            camera_plane_x = (camera_plane_x * f32::cos(rotation_speed_correct)) - (camera_plane_y * f32::sin(rotation_speed_correct));
            camera_plane_y = (old_plane_x * f32::sin(rotation_speed_correct)) + (camera_plane_y * f32::cos(rotation_speed_correct));
        }

        last_tick_time = current_time;

        // Render
        if render_timer >= sixty_hz {
            render_timer = render_timer - sixty_hz;

            canvas.clear();

            render_texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for x in 0..(WINDOW_WIDTH as usize) {
                    // Calculate the x coordinate of the ray in screen space
                    let ray_x: f32 = 2.0 * (x as f32 / RENDER_WIDTH as f32) - 1.0;

                    // Calculate the direction that the ray needs to go
                    let ray_direction_x: f32 = player_direction_x + (camera_plane_x * ray_x);
                    let ray_direction_y: f32 = player_direction_y + (camera_plane_y * ray_x);

                    let mut ray_map_x: i32 = player_x as i32;
                    let mut ray_map_y: i32 = player_y as i32;

                    // Length of ray from current position to next tile edge
                    let mut edge_distance_x: f32;
                    let mut edge_distance_y: f32;

                    // Length of ray from one tile edge to the next tile edge
                    let edge_delta_distance_x: f32 = f32::sqrt(1.0 + (ray_direction_y * ray_direction_y) / (ray_direction_x * ray_direction_x));
                    let edge_delta_distance_y: f32 = f32::sqrt(1.0 + (ray_direction_x * ray_direction_x) / (ray_direction_y * ray_direction_y));
                    let perp_edge_distance: f32;

                    // Information about the ray hit
                    let mut is_wall_hit: bool = false;
                    let mut wall_side: u8 = 0;

                    // Calculate the step and initial edge distance
                    let step_direction_x: i8;
                    let step_direction_y: i8;
                    if ray_direction_x < 0.0 {
                        step_direction_x = -1;
                        edge_distance_x = (player_x - ray_map_x as f32) * edge_delta_distance_x;
                    }
                    else {
                        step_direction_x = 1;
                        edge_distance_x = ((ray_map_x + 1) as f32 - player_x) * edge_delta_distance_x;
                    }
                    if ray_direction_y < 0.0 {
                        step_direction_y = -1;
                        edge_distance_y = (player_y - ray_map_y as f32) * edge_delta_distance_y;
                    }
                    else {
                        step_direction_y = 1;
                        edge_distance_y = ((ray_map_y + 1) as f32 - player_y) * edge_delta_distance_y;
                    }

                    // Perform DDA
                    while !is_wall_hit {
                        if edge_distance_x < edge_distance_y {
                            edge_distance_x += edge_delta_distance_x;
                            ray_map_x += step_direction_x as i32;
                            wall_side = 0;
                        }
                        else {
                            edge_distance_y += edge_delta_distance_y;
                            ray_map_y += step_direction_y as i32;
                            wall_side = 1;
                        }

                        is_wall_hit = MAP[((ray_map_y * MAP_WIDTH as i32) + ray_map_x) as usize] > 0;
                    }

                    // Calculate distance to ray hit, projected on camera plane (fixes the fish-eye effect)
                    if wall_side == 0 {
                        perp_edge_distance = (ray_map_x as f32 - player_x + (1.0 - step_direction_x as f32) / 2.0) / ray_direction_x;
                    }
                    else {
                        perp_edge_distance = (ray_map_y as f32 - player_y + (1.0 - step_direction_y as f32) / 2.0) / ray_direction_y;
                    }
                    
                    // Calculate the coordinates and height of the line that we need to render.
                    let line_height: f32 = RENDER_HEIGHT as f32 / perp_edge_distance;

                    let mut line_screen_start: f32 = (RENDER_HEIGHT as f32 / 2.0) - (line_height / 2.0);
                    //if line_screen_start < 0.0 { line_screen_start = 0.0; }

                    let mut line_screen_end: f32 = line_screen_start + line_height;
                    //if line_screen_end >= RENDER_HEIGHT as f32 { line_screen_end = RENDER_HEIGHT as f32 - 1.0; }
                    //println!("line_screen_start = {}, line_screen_end = {}", line_screen_start, line_screen_end);

                    //let tile: u32 = MAP[((ray_map_y * MAP_WIDTH as i32) + ray_map_x) as usize];

                    // Calculate the hit point on the edge
                    let mut wall_hit_point: f32;
                    if wall_side == 0 {
                        wall_hit_point = player_y + perp_edge_distance * ray_direction_y;
                    }
                    else {
                        wall_hit_point = player_x + perp_edge_distance * ray_direction_x;
                    }

                    // Convert to local edge coordinate
                    wall_hit_point -= f32::floor(wall_hit_point);

                    // Calculate the texture x coordinate
                    let mut texture_x: u32 = (wall_hit_point * (TEXTURE_WIDTH as f32 - 1.0)) as u32;
                    if ((wall_side == 0) && (ray_direction_x < 0.0)) || ((wall_side == 1) && (ray_direction_y > 0.0)) {
                        texture_x = TEXTURE_WIDTH - texture_x - 1;
                    }

                    for y in 0..(WINDOW_HEIGHT as usize) {
                        // pitch is WIDTH * bytes per pixel
                        let offset = (y * pitch) + (x * 3);

                        if ((y as i32) >= line_screen_start as i32) && ((y as i32) < line_screen_end as i32) {
                            let line_y: i32 = y as i32 - line_screen_start as i32;
                            let texture_y: u32 = ((line_y as f32 / line_height) * (TEXTURE_HEIGHT - 1) as f32) as u32;
                            //if texture_y >= 63 { texture_y = 63;}

                            let texture_pixel_index = (texture_y as usize * TEXTURE_WIDTH as usize) + texture_x as usize;
                            //println!("TEXTURE_SIZE = {}, line_y = {}, line_height = {}, texture_y = {}, texture_pixel_index = {}", TEXTURE_SIZE, line_y, line_height, texture_y, texture_pixel_index);

                            let texture_pixel = texture[texture_pixel_index];

                            // Extract 24 bit color components (r, g, b)
                            let red: u8 = (texture_pixel >> 16) as u8;
                            let green: u8 = (texture_pixel >> 8) as u8;
                            let blue: u8 = texture_pixel as u8;

                            buffer[offset] = red;
                            buffer[offset + 1] = green;
                            buffer[offset + 2] = blue;
                        }
                        else {
                            buffer[offset] = 0 as u8;
                            buffer[offset + 1] = 0 as u8;
                            buffer[offset + 2] = 128 as u8;
                        }

                        // Texture test
                        if x < TEXTURE_WIDTH as usize {
                            if y < TEXTURE_HEIGHT as usize {
                                let offset = (y * pitch) + (x * 3);
                                let texture_pixel_index = (y * TEXTURE_WIDTH as usize) + x;
                                let texture_pixel = texture[texture_pixel_index];

                                // Extract 24 bit color components (r, g, b)
                                let red: u8 = (texture_pixel >> 16) as u8;
                                let green: u8 = (texture_pixel >> 8) as u8;
                                let blue: u8 = texture_pixel as u8;

                                buffer[offset] = red;
                                buffer[offset + 1] = green;
                                buffer[offset + 2] = blue;
                            }
                        }
                    }
                }
            }).unwrap();

            canvas.copy_ex(&render_texture, None, None, 0.0, None, false, false).unwrap();
            canvas.present();
        }
    }
}