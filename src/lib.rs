pub const MAP_SIZE: usize = 30;
pub const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
pub const COLOR_BLACK: Color = Color { r: 255, g: 255, b: 255, a: 255 };
pub const COLOR_MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };

#[derive(Copy, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub texture_id: u32
}

pub struct RayIntersection {
    pub x: f64,
    pub y: f64,
    pub cell_x: i32,
    pub cell_y: i32,
    pub cell_side: u8,
    pub distance: f64
}

pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub texture_id: u32
}

impl Sprite {
    pub fn new(x: f64, y: f64, texture_id: u32) -> Sprite {
        Sprite {
            x: x,
            y: y,
            texture_id: texture_id
        }
    }
}

pub struct Texture {
    pub texture_id: u32,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>
}

impl Texture {
    pub fn new(texture_id: u32, width: u32, height: u32, pixels: &Vec<Color>) -> Texture {
        Texture {
            texture_id: texture_id,
            width: width,
            height: height,
            pixels: pixels.clone()
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        self.pixels[((y * self.width) + x) as usize]
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

pub struct Engine {
    pub field_of_view: f64,
    pub projection_width: u32,
    pub projection_height: u32,
    pub projection_distance: f64,
    pub depth_buffer: Vec<f64>
}

impl Engine {
    pub fn new(field_of_view: f64, projection_width: u32, projection_height: u32) -> Engine {
        Engine {
            field_of_view: field_of_view,
            projection_width: projection_width,
            projection_height: projection_height,
            projection_distance: (projection_width as f64 / 2.0) / f64::tan(field_of_view / 2.0),
            depth_buffer: Vec::with_capacity(projection_width as usize)
        }
    }

    pub fn render(&mut self, render_buffer: &mut [u8], cells: &Vec<Option<Cell>>, sprites: &Vec<Sprite>, textures: &Vec<Texture>, origin_x: f64, origin_y: f64, rotation: f64) {
        // pitch is WIDTH * bytes per pixel
        let pitch: usize = (self.projection_width as usize) * 4;

        for x in 0..(self.projection_width as usize) {
            // Where on the screen the ray goes through
            let ray_screen_x: f64 = -(self.projection_width as f64) / 2.0 + x as f64;

            // The distance from the viewer to the point on the screen
            let ray_view_dist = (ray_screen_x.powi(2) + self.projection_distance.powi(2)).sqrt();

            let ray_angle: f64 = (ray_screen_x / ray_view_dist).asin() + rotation;
            let intersection: RayIntersection = self.cast_ray(origin_x, origin_y, ray_angle, cells);

            let hit_distance = intersection.distance.sqrt() * (rotation - ray_angle).cos();

            let tile_side = intersection.cell_side;

            let tile_size = 1.0;

            // Store the distance in the depth buffer
            self.depth_buffer.push(hit_distance);

            // Calculate the position and height of the wall strip.
            // The wall height is 1 unit, the distance from the player to the screen is viewDist,
            // thus the height on the screen is equal to
            // wallHeight * viewDist / dist
            let line_height: i32 = ((tile_size * self.projection_distance) / hit_distance).round() as i32;
            let line_screen_start: i32 = (self.projection_height as i32 / 2) - (line_height / 2);
            let line_screen_end: i32 = line_screen_start + line_height;

            let ref wall_texture: Texture = textures[4];
            let wall_texture_x: u32 = if tile_side == 0 {
                (((intersection.y - (intersection.cell_y as f64 * tile_size)) % tile_size) * (wall_texture.width - 1) as f64).round() as u32
            } else {
                (((intersection.x - (intersection.cell_x as f64 * tile_size)) % tile_size) * (wall_texture.width - 1) as f64).round() as u32
            };

            for y in 0..(self.projection_height as usize) {
                let offset: usize = (y * pitch) + (x * 4);

                if (y as i32) < line_screen_start {
                    // Ceiling casting

                    let player_height: f64 = 0.5;
                    let ceiling_row: i32 = (y as i32) - (self.projection_height as i32 / 2);

                    let ceiling_straight_distance = (player_height / ceiling_row as f64) * self.projection_distance;
                    let angle_beta_radians = rotation - ray_angle;

                    let floor_actual_distance = ceiling_straight_distance / angle_beta_radians.cos();

                    let mut ceiling_hit_x: f64 = origin_x - (floor_actual_distance * ray_angle.cos());
                    let mut ceiling_hit_y: f64 = origin_y - (floor_actual_distance * ray_angle.sin());

                    ceiling_hit_x -= ceiling_hit_x.floor();
                    ceiling_hit_y -= ceiling_hit_y.floor();

                    // TODO: We are getting these textures for every pixel... can't we cache them somehow?
                    let ref texture: Texture = textures[6];
                    let texture_x: u32 = f64::floor(ceiling_hit_x * (texture.width - 1) as f64) as u32;
                    let texture_y: u32 = f64::floor(ceiling_hit_y * (texture.height - 1) as f64) as u32;

                    let pixel = texture.get_pixel(texture_x, texture_y);

                    render_buffer[offset] = pixel.a;
                    render_buffer[offset + 1] = pixel.b;
                    render_buffer[offset + 2] = pixel.g;
                    render_buffer[offset + 3] = pixel.r;
                }
                else if ((y as i32) >= line_screen_start) && ((y as i32) < line_screen_end) {
                    // Wall casting

                    let line_y: i32 = y as i32 - line_screen_start;
                    let texture_y: u32 = f64::floor((line_y as f64 / line_height as f64) * (wall_texture.height - 1) as f64) as u32;
                    let pixel = wall_texture.get_pixel(wall_texture_x, texture_y);

                    render_buffer[offset] = pixel.a;
                    render_buffer[offset + 1] = if tile_side == 1 { pixel.b } else { pixel.g / 2 };
                    render_buffer[offset + 2] = if tile_side == 1 { pixel.g } else { pixel.b / 2 };
                    render_buffer[offset + 3] = if tile_side == 1 { pixel.r } else { pixel.r / 2 };
                }
                else if (y as i32) >= line_screen_end {
                    // Floor casting

                    let player_height: f64 = 0.5;
                    let floor_row: i32 = (y as i32) - (self.projection_height as i32 / 2);

                    let floor_straight_distance = (player_height / floor_row as f64) * self.projection_distance;
                    let angle_beta_radians = rotation - ray_angle;

                    let floor_actual_distance = floor_straight_distance / f64::cos(angle_beta_radians);

                    let mut floor_hit_x: f64 = origin_x + (floor_actual_distance * f64::cos(ray_angle));
                    let mut floor_hit_y: f64 = origin_y + (floor_actual_distance * f64::sin(ray_angle));

                    floor_hit_x -= floor_hit_x.floor();
                    floor_hit_y -= floor_hit_y.floor();

                    let ref texture: Texture = textures[5];
                    let texture_x: u32 = f64::floor(floor_hit_x * (texture.width - 1) as f64) as u32;
                    let texture_y: u32 = f64::floor(floor_hit_y * (texture.height - 1) as f64) as u32;
                    let pixel = texture.get_pixel(texture_x, texture_y);

                    render_buffer[offset] = pixel.a;
                    render_buffer[offset + 1] = pixel.b;
                    render_buffer[offset + 2] = pixel.g;
                    render_buffer[offset + 3] = pixel.r;
                }
                else {
                    let pixel = COLOR_MAGENTA;
                    render_buffer[offset] = pixel.a;
                    render_buffer[offset + 1] = pixel.b;
                    render_buffer[offset + 2] = pixel.g;
                    render_buffer[offset + 3] = pixel.r;
                }
            }
        }

        // Sort sprites (far to near)
        /*
        sprites.sort_by(|a, b| {
            let a_distance: f64 = (a.x - origin_x).powi(2) + (a.y - origin_y).powi(2);
            let b_distance: f64 = (b.x - origin_x).powi(2) + (b.y - origin_y).powi(2);

            b_distance.partial_cmp(&a_distance).unwrap()
        });
        */

        // Render sprites
        for sprite in sprites.iter() {
            let distance_x: f64 = sprite.x - origin_x;
            let distance_y: f64 = sprite.y - origin_y;

            // The angle between the player and the sprite
            let mut theta: f64 = f64::atan2(distance_y, distance_x);
            theta = self.wrap_angle(theta);

            // The angle between the player and the sprite, relative to the player rotation
            let mut gamma: f64 = theta - rotation;
            gamma = self.wrap_angle(gamma);

            let sprite_distance: f64 = f64::sqrt(distance_x.powi(2) + distance_y.powi(2)) * f64::cos(rotation - theta);

            // The number of pixels to offset from the center of the screen
            let sprite_pixel_offset: f64 = f64::tan(gamma) * self.projection_distance;
            let sprite_screen_x: i32 = f64::round((self.projection_width as f64 / 2.0) + sprite_pixel_offset) as i32;

            let sprite_height: i32 = (f64::round(self.projection_distance / sprite_distance) as i32).abs();
            let sprite_width: i32 = (f64::round(self.projection_distance / sprite_distance) as i32).abs();

            let sprite_screen_start_x: i32 = sprite_screen_x - (sprite_width / 2);
            let sprite_screen_end_x: i32 = sprite_screen_x + (sprite_width / 2);
            let sprite_screen_start_y: i32 = -(sprite_height / 2) + (self.projection_height as i32 / 2);
            let sprite_screen_end_y: i32 = (sprite_height / 2) + (self.projection_height as i32 / 2);

            let mut camera_min_angle: f64 = -self.field_of_view / 2.0;
            camera_min_angle = self.wrap_angle(camera_min_angle);

            let mut camera_max_angle: f64 = self.field_of_view / 2.0;
            camera_max_angle = self.wrap_angle(camera_max_angle);

            let ref texture: Texture = textures[sprite.texture_id as usize];

            for sprite_screen_row in sprite_screen_start_x..sprite_screen_end_x {
                if (sprite_screen_row < 0) || (sprite_screen_row >= self.projection_width as i32) {
                    continue;
                }

                // If the sprite is not visible, don't render it.
                if ((gamma < camera_min_angle) && (gamma > camera_max_angle)) ||
                    (self.depth_buffer[sprite_screen_row as usize] < sprite_distance) {
                    continue;
                }

                for sprite_screen_col in sprite_screen_start_y..sprite_screen_end_y {
                    if (sprite_screen_col < 0) || (sprite_screen_col >= self.projection_height as i32) {
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

                    render_buffer[offset] = pixel.a;
                    render_buffer[offset + 1] = pixel.b;
                    render_buffer[offset + 2] = pixel.g;
                    render_buffer[offset + 3] = pixel.r;
                }
            }
        }

        self.depth_buffer.clear();
    }

    fn wrap_angle(&self, angle: f64) -> f64 {
        if angle < 0.0 {
            return angle + TWO_PI;
        }
        else if angle >= TWO_PI {
            return angle - TWO_PI;
        }

        angle
    }

    fn cast_ray(&self, origin_x: f64, origin_y: f64, angle: f64, map: &Vec<Option<Cell>>) -> RayIntersection {
        let mut intersection_distance: f64 = 0.0; // Distance from origin to intersection
        let mut x: f64 = 0.0; // Intersection point x
        let mut y: f64 = 0.0; // Intersection point y
        let mut cell_x: i32 = 0; // Intersected cell x
        let mut cell_y: i32 = 0; // Intersected cell y
        let mut cell_edge: u8 = 0; // 0 for y-axis, or 1 for x-axis

        let cell_size: f64 = 1.0;

        // Calculate the quadrant up the ray
        let angle = self.wrap_angle(angle);
        let is_ray_right: bool = angle > (TWO_PI * 0.75) || angle < (TWO_PI * 0.25);
        let is_ray_up: bool = angle < 0.0 || angle > std::f64::consts::PI;

        // Check for vertical (y axis) intersections

        let mut slope: f64 = angle.sin() / angle.cos();
        let mut delta_x: f64 = if is_ray_right { cell_size } else { -cell_size }; // Horizontal step amount
        let mut delta_y: f64 = delta_x * slope; // Vertical step amount

        // Calculate the ray starting position
        let mut ray_position_x: f64 = if is_ray_right { f64::ceil(origin_x) } else { f64::floor(origin_x) };
        let mut ray_position_y: f64 = origin_y + (ray_position_x - origin_x) * slope;

        while (ray_position_x >= 0.0) && (ray_position_x < MAP_SIZE as f64) && (ray_position_y >= 0.0) && (ray_position_y < MAP_SIZE as f64) {
            let tile_map_x: i32 = f64::floor(ray_position_x + (if is_ray_right { 0.0 } else { -cell_size })) as i32;
            let tile_map_y: i32 = f64::floor(ray_position_y) as i32;

            if let Some(cell) = map[((tile_map_y * MAP_SIZE as i32) + tile_map_x) as usize] {
                let distance_x: f64 = ray_position_x - origin_x;
                let distance_y: f64 = ray_position_y - origin_y;
                intersection_distance = distance_x.powi(2) + distance_y.powi(2);

                cell_edge = 0;

                cell_x = cell.x;
                cell_y = cell.y;

                x = ray_position_x;
                y = ray_position_y;

                break;
            }

            ray_position_x += delta_x;
            ray_position_y += delta_y;
        }

        // Check for horizontal (x axis) intersections

        slope = angle.cos() / angle.sin();
        delta_y = if is_ray_up { -cell_size } else { cell_size }; // Vertical step amount
        delta_x = delta_y * slope; // Horizontal step amount

        // Calculate the ray starting position
        ray_position_y = if is_ray_up { f64::floor(origin_y) } else { f64::ceil(origin_y) };
        ray_position_x = origin_x + (ray_position_y - origin_y) * slope;

        while (ray_position_x >= 0.0) && (ray_position_x < MAP_SIZE as f64) && (ray_position_y >= 0.0) && (ray_position_y < MAP_SIZE as f64) {
            let tile_map_x: i32 = f64::floor(ray_position_x) as i32;
            let tile_map_y: i32 = f64::floor(ray_position_y + (if is_ray_up { -cell_size } else { 0.0 })) as i32;

            if let Some(cell) = map[((tile_map_y * MAP_SIZE as i32) + tile_map_x) as usize] {
                let distance_x: f64 = ray_position_x - origin_x;
                let distance_y: f64 = ray_position_y - origin_y;
                let x_intersection_distance = distance_x.powi(2) + distance_y.powi(2);

                if (intersection_distance == 0.0) || (x_intersection_distance < intersection_distance) {
                    intersection_distance = x_intersection_distance;
                    cell_edge = 1;

                    cell_x = cell.x;
                    cell_y = cell.y;

                    x = ray_position_x;
                    y = ray_position_y;
                }

                break;
            }

            ray_position_x += delta_x;
            ray_position_y += delta_y;
        }

        RayIntersection {
            x: x,
            y: y,
            cell_x: cell_x,
            cell_y: cell_y,
            cell_side: cell_edge,
            distance: intersection_distance
        }
    }
}