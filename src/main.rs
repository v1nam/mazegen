use macroquad::prelude::*;
use macroquad::rand::{srand, ChooseRandom};

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze 3D".to_owned(),
        window_width: 1260,
        window_height: 730,
        ..Default::default()
    }
}

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

#[macroquad::main(window_conf)]
async fn main() {
    const SIZE: usize = 31;
    let mut grid: [[bool; SIZE]; SIZE] = [[true; SIZE]; SIZE]; // false means wall
    let mut stack = vec![[0, 0]];
    let mut visited = vec![[0, 0]];
    let mut constructed = false;
    for (y, row) in grid.iter_mut().enumerate() {
        if y % 2 == 0 {
            *row = [false; SIZE];
        } else {
            for (x, col) in row.iter_mut().enumerate() {
                if x % 2 == 0 {
                    *col = false;
                }
            }
        }
    }
    srand(macroquad::miniquad::date::now() as u64);
    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();
    let mut right = front.cross(world_up).normalize();
    let mut up;

    let mut position = vec3(13.5, 1.0, 13.5);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);
    loop {
        if !constructed && !stack.is_empty() {
            let mut neighbours: Vec<[usize; 2]> = Vec::new();
            let current_cell = stack.pop().unwrap();
            let adjacents = [[-1, 0], [1, 0], [0, -1], [0, 1]];
            for [x, y] in &adjacents {
                let neighbour = [current_cell[0] as i32 + x, current_cell[1] as i32 + y];
                if [-1, 15].contains(&neighbour[0]) || [-1, 15].contains(&neighbour[1]) {
                    continue;
                }
                let neighbour = [neighbour[0] as usize, neighbour[1] as usize];
                if !visited.contains(&neighbour) {
                    neighbours.push(neighbour);
                }
            }
            if neighbours.is_empty() {
                continue;
            } else {
                stack.push(current_cell);
                let chosen_cell = *neighbours.choose().unwrap();
                let cc = [2 * current_cell[0] + 1, 2 * current_cell[1] + 1];
                let nc = [2 * chosen_cell[0] + 1, 2 * chosen_cell[1] + 1];
                let wc = [
                    nc[0] as i32 + (cc[0] as i32 - nc[0] as i32) / 2,
                    nc[1] as i32 + (cc[1] as i32 - nc[1] as i32) / 2,
                ];
                grid[wc[1] as usize][wc[0] as usize] = true;
                stack.push(chosen_cell);
                visited.push(chosen_cell);
            }
            draw_text(
                "GENERATING",
                screen_width() / 2. - 80.,
                screen_height() / 2.,
                40.,
                RED,
            );
        } else {
            constructed = true;
            let delta = get_frame_time();

            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            if is_key_pressed(KeyCode::Tab) {
                grabbed = !grabbed;
                set_cursor_grab(grabbed);
                show_mouse(!grabbed);
            }

            if is_key_down(KeyCode::W) {
                position += front * MOVE_SPEED;
            }
            if is_key_down(KeyCode::S) {
                position -= front * MOVE_SPEED;
            }
            if is_key_down(KeyCode::A) {
                position -= right * MOVE_SPEED;
            }
            if is_key_down(KeyCode::D) {
                position += right * MOVE_SPEED;
            }
            position.y = 1.0;
            let mouse_position: Vec2 = mouse_position().into();
            let mouse_delta = mouse_position - last_mouse_position;
            last_mouse_position = mouse_position;

            yaw += mouse_delta.x * delta * LOOK_SPEED;
            pitch += mouse_delta.y * delta * -LOOK_SPEED;

            pitch = if pitch > 1.5 { 1.5 } else { pitch };
            pitch = if pitch < -1.5 { -1.5 } else { pitch };

            front = vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            )
            .normalize();

            right = front.cross(world_up).normalize();
            up = right.cross(front).normalize();

            x += if switch { 0.04 } else { -0.04 };
            if x >= bounds || x <= -bounds {
                switch = !switch;
            }

            clear_background(LIGHTGRAY);

            set_camera(&Camera3D {
                position,
                up,
                target: position + front,
                ..Default::default()
            });

            draw_grid(31, 1., BLACK, GRAY);
            for (x_, row) in grid.iter().enumerate() {
                for (z, column) in row.iter().enumerate() {
                    let mut color = WHITE;
                    if z == 1 && x_ == 1 {
                        color = RED;
                    } else if x_ == 29 && z == 29 {
                        color = YELLOW;
                    }
                    if !*column {
                        color = DARKGRAY;
                    }
                    if color != WHITE {
                        draw_cube(
                            vec3(15. - x_ as f32, 1.75, 15. - z as f32),
                            vec3(1., 3.5, 1.),
                            None,
                            color,
                        );
                    }
                }
            }

            set_default_camera();
        }
        next_frame().await
    }
}
