use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 200.0;
const BALL_RADIUS: f32 = 32.0;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}

impl Shape {
    fn collides_with_circle(&self, circle: &Shape) -> bool {
        let half = self.size / 2.0;
        let dx = (self.x - circle.x).abs().max(half) - half;
        let dy = (self.y - circle.y).abs().max(half) - half;
        dx * dx + dy * dy <= circle.size * circle.size / 4.0
    }
}

#[macroquad::main("Mitt spel")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let colors: Vec<Color> = vec![BLACK, RED, BLUE, GREEN, PINK, SKYBLUE, DARKBLUE];

    let mut squares = vec![];
    let mut circle = Shape {
        size: BALL_RADIUS * 2.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: DARKPURPLE,
    };

    let mut game_over = false;

    loop {
        clear_background(GOLD);
        let delta_time = get_frame_time();
        let movement = delta_time * MOVEMENT_SPEED;

        if game_over && is_key_pressed(KeyCode::Space) {
            squares.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            game_over = false;
        }

        if !game_over {
            if is_key_down(KeyCode::Right) {
                circle.x += movement;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= movement;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += movement;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= movement;
            }

            circle.x = circle
                .x
                .min(screen_width() - BALL_RADIUS)
                .max(0.0 + BALL_RADIUS);
            circle.y = circle
                .y
                .min(screen_height() - BALL_RADIUS)
                .max(0.0 + BALL_RADIUS);

            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: *colors.choose().unwrap(),
                });
            }

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }

            if squares
                .iter()
                .any(|square| square.collides_with_circle(&circle))
            {
                game_over = true;
            }

            squares.retain(|square| square.y < screen_width() + square.size);
        }

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);

        if game_over {
            let game_over_text = "GAME OVER! Press space to restart.";
            let text_dimensions = measure_text(game_over_text, None, 32, 1.0);

            let text_x = (screen_width() - text_dimensions.width) / 2.0;
            let text_y = screen_height() / 2.0 - text_dimensions.offset_y + text_dimensions.height;

            draw_text(game_over_text, text_x, text_y, 32.0, WHITE);
        }

        next_frame().await
    }
}
