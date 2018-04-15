use ggez::{graphics, Context, GameResult};

extern crate rand;
use rand::*;

#[derive(Debug)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: Direction,
    pub alive: bool,
}

impl Player {
    pub fn new(_ctx: &mut Context) -> GameResult<Player> {
        Ok(Player {
            x: 200.0,
            y: 700.0,
            speed: 200.0,
            // faster speed: 350.0,
            // slower speed: 50.0,
            direction: Direction::Idle,
            alive: true,
        })
    }

    pub fn player_handle_input(&mut self, _ctx: &mut Context, dt: f32, player_width: u32) {
        match self.direction {
            Direction::Left => {
                if self.x > 0.0 {
                    self.x = self.x - (self.speed * dt)
                }
            }
            Direction::Right => {
                let (screen_width, _) = graphics::get_size(_ctx);
                let bound = (screen_width - player_width) as f32;

                if self.x < bound {
                    self.x = self.x + (self.speed * dt);
                }
            }

            _ => (),
        }
    }
}
#[derive(Debug)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
}

impl Bullet {
    pub fn new(player_x: f32, player_y: f32, player_width: u32) -> GameResult<Bullet> {
        Ok(Bullet {
            x: player_x + (player_width as f32 / 2.0),
            y: player_y,
            alive: true,
        })
    }

    pub fn handle(&mut self, _ctx: &mut Context, dt: f32) {
        self.y += -500.0 * dt;
    }
}

#[derive(Debug)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub alive: bool,
}

impl Enemy {
    pub fn new(ctx: &mut Context) -> GameResult<Enemy> {
        let (_, width) = graphics::get_size(ctx);
        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(10.0, width as f32) - 50.0;

        Ok(Enemy {
            x: x_pos,
            y: -10.0,
            alive: true,
        })
    }
}

pub struct Assets {
    pub player: graphics::Image,
    pub bullet: graphics::Image,
    pub enemy: graphics::Image,
}

impl Assets {
    pub fn new(_ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            player: graphics::Image::new(_ctx, "/plane.png").unwrap(),
            bullet: graphics::Image::new(_ctx, "/bullet.png").unwrap(),
            enemy: graphics::Image::new(_ctx, "/enemy.png").unwrap(),
        })
    }
}
