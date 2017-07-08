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

        // Calculate elapsed time
        let current_time = time::now();
        let delta_time = current_time - last_tick_time;
        let total_time = current_time - start_time;
        render_timer = render_timer + delta_time;

        // Setup
        let player_x: f32 = 14.5;
        let player_y: f32 = 22.0;
        let player_direction_x: f32 = 0.0;
        let player_direction_y: f32 = -1.0;
        let camera_plane_x: f32 = 1.0;
        let camera_plane_y: f32 = 0.0;
        //let view_distance: f32 = ((RENDER_WIDTH / 2) as f32) / f32::tan(FIELD_OF_VIEW / 2_f32);

        // Tick
        // TODO

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
                    if line_screen_start < 0.0 { line_screen_start = 0.0; }

                    let mut line_screen_end: f32 = line_screen_start + line_height;
                    if line_screen_end >= RENDER_HEIGHT as f32 { line_screen_end = RENDER_HEIGHT as f32 - 1.0; }

                    let tile: u32 = MAP[((ray_map_y * MAP_WIDTH as i32) + ray_map_x) as usize];

                    let mut color: u32 = match tile {
                        1 => { 255 },
                        2 => { 200 },
                        3 => { 150 },
                        _ => { 0 }
                    };

                    if wall_side == 1
                    {
                        color /= 2;
                    }

                    for y in (line_screen_start as i32)..(line_screen_end as i32) {
                        let offset = (y as usize * pitch) + (x * 3);

                        buffer[offset] = color as u8;
                        buffer[offset + 1] = 0 as u8;
                        buffer[offset + 2] = 0 as u8;
                    }

                    /*
                    for y in 0..(WINDOW_HEIGHT as usize) {
                        let offset = (y * pitch) + (x * 3);

                        buffer[offset] = color as u8;
                        buffer[offset + 1] = 0 as u8;
                        buffer[offset + 2] = 0 as u8;
                    }
                    */
                }
            }).unwrap();

            canvas.copy_ex(&render_texture, None, None, 0.0, None, false, false).unwrap();
            canvas.present();
        }
    }
}