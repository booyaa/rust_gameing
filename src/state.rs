use ggez::*;
use ggez::event::{Keycode, Mod};

pub struct BulletState {
    pub can_shoot: bool,
    pub can_shoot_timer_max: f32,
    pub can_shoot_timer: f32,
}

impl BulletState {
    pub fn new(_ctx: &mut Context) -> GameResult<BulletState> {
        Ok(BulletState {
            can_shoot: true,
            can_shoot_timer_max: 0.2,
            can_shoot_timer: 0.2,
        })
    }
}

pub struct MainState {
    assets: super::entity::Assets,
    player: super::entity::Player,
    bullet_state: BulletState,
    bullets: Vec<super::entity::Bullet>,
}

impl MainState {
    pub fn new(mut _ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            assets: super::entity::Assets::new(&mut _ctx)?,
            player: super::entity::Player::new(&mut _ctx)?,
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

            self.bullet_state.can_shoot_timer = self.bullet_state.can_shoot_timer - (1.0 * seconds);
            if self.bullet_state.can_shoot_timer < 0.0 {
                self.bullet_state.can_shoot = true;
            }

            for bullet in self.bullets.iter_mut() {
                bullet.handle(_ctx, seconds);
            }

            self.player
                .player_handle_input(_ctx, seconds, self.assets.player.width());

            // remove bullets that have vanished off screen
            self.bullets.retain(|bullet| bullet.y >= 0.0);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);

        let dest_point = graphics::Point2::new(self.player.x, self.player.y);
        let rotation = 0.0;
        graphics::draw(ctx, &self.assets.player, dest_point, rotation)?;

        // bullets
        for bullet in self.bullets.iter_mut() {
            let dest_point = graphics::Point2::new(bullet.x, bullet.y);
            let rotation = 0.0;
            graphics::draw(ctx, &self.assets.bullet, dest_point, rotation)?;
        }

        //TODO add feature toggle
        let fps = timer::get_fps(ctx);
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 8)?;
        let text = graphics::Text::new(ctx, &format!("Current FPS: {:.0}", fps), &font)?;
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &text, dest_point, 0.0)?;

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
                self.player.direction = super::entity::Direction::Left;
            }
            Keycode::Right | Keycode::D => {
                self.player.direction = super::entity::Direction::Right;
            }
            Keycode::Space => {
                let bullet = super::entity::Bullet::new(
                    self.player.x,
                    self.player.y,
                    self.assets.player.width(),
                ).unwrap();
                self.bullets.push(bullet);
                self.bullet_state.can_shoot = false;
                self.bullet_state.can_shoot_timer = self.bullet_state.can_shoot_timer_max;
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
        self.player.direction = super::entity::Direction::Idle
    }
}
