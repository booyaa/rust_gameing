extern crate ggez;
use ggez::{graphics, Context, GameResult};

#[derive(Debug)]
pub enum Direction {
    Idle,
    Left,
    Right,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: Direction,
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

pub struct Bullet {
    pub x: f32,
    pub y: f32,
}

impl Bullet {
    pub fn new(player_x: f32, player_y: f32, player_width: u32) -> GameResult<Bullet> {
        Ok(Bullet {
            x: player_x + (player_width as f32 / 2.0),
            y: player_y,
        })
    }

    pub fn handle(&mut self, _ctx: &mut Context, dt: f32) {
        self.y += -500.0 * dt;
    }
}

pub struct Assets {
    pub player: graphics::Image,
    pub bullet: graphics::Image,
}

impl Assets {
    pub fn new(_ctx: &mut Context) -> GameResult<Assets> {
        Ok(Assets {
            player: graphics::Image::new(_ctx, "/plane.png").unwrap(),
            bullet: graphics::Image::new(_ctx, "/bullet.png").unwrap(),
        })
    }
}
