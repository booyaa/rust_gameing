extern crate ggez;
use ggez::*;
use ggez::event::{Keycode, Mod};
use std::env;
use std::path;

#[derive(Debug)]
enum Direction {
    Idle,
    Left,
    Right,
}

struct Player {
    x: f32,
    y: f32,
    speed: f32,
    image: graphics::Image,
    direction: Direction,
}

impl Player {
    fn new(_ctx: &mut Context) -> GameResult<Player> {
        Ok(Player {
            x: 200.0,
            y: 500.0,
            speed: 200.0,
            image: graphics::Image::new(_ctx, "/plane.png").unwrap(),
            direction: Direction::Idle,
        })
    }

    fn player_handle_input(&mut self, _ctx: &mut Context, dt: f32) {
        match self.direction {
            Direction::Left => {
                if self.x > 0.0 {
                    self.x = self.x - (self.speed * dt)
                }
            }
            Direction::Right => {
                let (screen_width, _) = graphics::get_size(_ctx);
                let bound = (screen_width - self.image.width()) as f32;

                if self.x < bound {
                    self.x = self.x + (self.speed * dt);
                }
            }

            _ => (),
        }
    }
}

struct Bullet {
    x: f32,
    y: f32,
}

impl Bullet {
    fn new(player_x: f32, player_y: f32, player_width: u32) -> GameResult<Bullet> {
        Ok(Bullet {
            x : player_x + (player_width as f32 / 2.0),
            y : player_y
        })
    }

    fn handle(&mut self, _ctx: &mut Context, dt: f32) {
        self.y += -500.0 * dt;
    }
}

struct BulletState {
    bullet_image: graphics::Image,
    can_shoot: bool,
    can_shoot_timer_max: f32,
    can_shoot_timer: f32
}

impl BulletState {
    fn new(_ctx: &mut Context) -> GameResult<BulletState> {
        Ok(BulletState {
            bullet_image: graphics::Image::new(_ctx, "/bullet.png").unwrap(),
            can_shoot: true,
            can_shoot_timer_max: 0.2,
            can_shoot_timer: 0.2,
        })
    }

    // fn handle(&mut self, _ctx: &mut Context, dt: f32) {
    //     self.can_shoot_timer = self.can_shoot_timer - (1.0 * dt);
    //         if self.can_shoot_timer < 0.0 {
    //             self.can_shoot = true;
    //         }
    //         // shots: https://github.com/AndrewRadev/rust-shooter
    //         // might help (bullets): https://github.com/keeslinp/rust_invaders
    //         for (idx, mut bullet) in self.bullets.into_iter().enumerate(){
    //             bullet.y = bullet.y - (250.0 * dt);

    //             if bullet.y < 0.0 {
    //                 self.bullets.remove(idx);
    //             }
                
    //         }
    // }
}
struct MainState {
    player: Player,
    bullet_state: BulletState,
    bullets: Vec<Bullet>,
}

impl MainState {
    fn new(mut _ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            player: Player::new(&mut _ctx)?,
            bullet_state: BulletState::new(&mut _ctx)?,
            bullets: Vec::new(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(_ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            for bullet in self.bullets.iter_mut() {
                bullet.handle(_ctx, seconds);
            }

            self.player.player_handle_input(_ctx, seconds);

            // cleanup
            self.bullets.retain(|bullet| bullet.y >= 0.0);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);

        let dest_point = graphics::Point2::new(self.player.x, self.player.y);
        let rotation = 0.0;
        graphics::draw(ctx, &self.player.image, dest_point, rotation)?;

        // bullets
        for bullet in self.bullets.iter_mut() {
            let dest_point = graphics::Point2::new(bullet.x, bullet.y);
            let rotation = 0.0;
            graphics::draw(ctx, &self.bullet_state.bullet_image, dest_point, rotation)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}, x: {} direction: {:?}",
            keycode, keymod, repeat, self.player.x, self.player.direction
        );

        match keycode {
            Keycode::Left | Keycode::A => {
                self.player.direction = Direction::Left;
            }
            Keycode::Right | Keycode::D => {
                self.player.direction = Direction::Right;
            }
            Keycode::Space => {
                let bullet = Bullet::new(self.player.x,self.player.y, self.player.image.width()).unwrap();
                self.bullets.push(bullet);
                println!("pew! {}", self.bullets.len());

            }
            Keycode::Escape => _ctx.quit().unwrap(),

            _ => (), // do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
        self.player.direction = Direction::Idle
    }
}

pub fn main() {
    let mut cb = ContextBuilder::new("PCIS", "ggez")
        .window_setup(
            conf::WindowSetup::default().title("Potential Copyright Infringment Shooters!"),
        )
        .window_mode(conf::WindowMode::default().dimensions(480, 800));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        cb = cb.add_resource_path(path);
    }

    let ctx = &mut cb.build().unwrap();

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
