use ggez::*;
use ggez::event::{Keycode, Mod};

fn is_collision(x1: f32, y1: f32, w1: f32, h1: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
    x1 < x2 + w2 && x2 < x1 + w1 && y1 < y2 + h2 && y2 < y1 + h1
}

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

pub struct EnemyState {
    pub create_timer: f32,
    pub create_timer_max: f32,
}

impl EnemyState {
    pub fn new(_ctx: &mut Context) -> GameResult<EnemyState> {
        Ok(EnemyState {
            // create_timer: 0.2, // very fast
            // create_timer_max: 0.2,
            create_timer: 0.4,
            create_timer_max: 0.4,
            // create_timer: 2.0, // slow
            // create_timer_max: 2.0,
        })
    }
}

pub struct MainState {
    assets: super::entity::Assets,
    player: super::entity::Player,
    bullet_state: BulletState,
    bullets: Vec<super::entity::Bullet>,
    enemy_state: EnemyState,
    enemies: Vec<super::entity::Enemy>,
    score: u32,
    restart: bool,
}

impl MainState {
    pub fn new(mut _ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            assets: super::entity::Assets::new(&mut _ctx)?,
            player: super::entity::Player::new(&mut _ctx)?,
            bullet_state: BulletState::new(&mut _ctx)?,
            bullets: Vec::new(),
            enemy_state: EnemyState::new(&mut _ctx)?,
            enemies: Vec::new(),
            score: 0,
            restart: false,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(_ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

            // set timer for how far apart our shots can be
            self.bullet_state.can_shoot_timer = self.bullet_state.can_shoot_timer - (1.0 * seconds);
            if self.bullet_state.can_shoot_timer < 0.0 {
                self.bullet_state.can_shoot = true;
            }

            // set timer for how many enemies to spawn
            self.enemy_state.create_timer = self.enemy_state.create_timer - (1.0 * seconds);
            if self.enemy_state.create_timer < 0.0 {
                self.enemy_state.create_timer = self.enemy_state.create_timer_max;
                let enemy = super::entity::Enemy::new(_ctx).unwrap(); // FIXME: randomize pos
                self.enemies.push(enemy);
                println!("enemies: {}", self.enemies.len());
            }

            // update the position of the bullets
            for bullet in self.bullets.iter_mut() {
                bullet.handle(_ctx, seconds);
            }

            // update the position of the enemies
            for enemy in self.enemies.iter_mut() {
                enemy.y = enemy.y + (200.0 * seconds);
            }

            // check if enemies against bullets
            for enemy in self.enemies.iter_mut() {
                for bullet in self.bullets.iter_mut() {
                    if is_collision(
                        enemy.x,
                        enemy.y,
                        self.assets.enemy.width() as f32,
                        self.assets.enemy.height() as f32,
                        bullet.x,
                        bullet.y,
                        self.assets.bullet.width() as f32,
                        self.assets.bullet.height() as f32
                    ) {
                        enemy.alive = false;
                        bullet.alive = false;
                        self.score = self.score + 1;
                    }
                }

                if is_collision(
                    enemy.x,
                    enemy.y,
                    self.assets.enemy.width() as f32,
                    self.assets.enemy.height() as f32,
                    self.player.x,
                    self.player.y,
                    self.assets.player.width() as f32,
                    self.assets.player.height() as f32,
                ) {
                    enemy.alive = false;
                    self.player.alive = false;
                }
            }

            

            
            // clean up vecs if bullets or enemies have flown off screen or collided
            self.bullets
                .retain(|bullet| bullet.alive && bullet.y >= 0.0);
            self.enemies.retain(|enemy| enemy.alive && enemy.y < 850.0);

            if ! self.player.alive && self.restart {
                self.bullets.clear();
                self.enemies.clear();
                self.bullet_state.can_shoot_timer = self.bullet_state.can_shoot_timer_max;
                self.enemy_state.create_timer = self.enemy_state.create_timer_max;
                self.player.x = 50.0;
                self.player.y = 710.0;
                self.player.alive = true;
                self.score = 0;
                self.restart = false;
            }

            self.player
                .player_handle_input(_ctx, seconds, self.assets.player.width());
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);

        // bullets
        for bullet in self.bullets.iter_mut() {
            let dest_point = graphics::Point2::new(bullet.x, bullet.y);
            let rotation = 0.0;
            graphics::draw(ctx, &self.assets.bullet, dest_point, rotation)?;
        }

        // enemies
        for enemy in self.enemies.iter_mut() {
            let dest_point = graphics::Point2::new(enemy.x, enemy.y);
            let rotation = 0.0;
            graphics::draw(ctx, &self.assets.enemy, dest_point, rotation)?;
        }

        //TODO add feature toggle
        let fps = timer::get_fps(ctx);
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 8)?;
        let text = graphics::Text::new(ctx, &format!("Current FPS: {:.0}", fps), &font)?;
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &text, dest_point, 0.0)?;

        let text = graphics::Text::new(ctx, &format!("SCORE: {:.0}", self.score), &font)?;
        let dest_point = graphics::Point2::new(400.0, 10.0);
        graphics::draw(ctx, &text, dest_point, 0.0)?;

        if self.player.alive {
            let dest_point = graphics::Point2::new(self.player.x, self.player.y);
            let rotation = 0.0;
            graphics::draw(ctx, &self.assets.player, dest_point, rotation)?;
        } else {
            let (screen_width, screen_height) = graphics::get_size(ctx);
            let text = graphics::Text::new(ctx, "Press 'R' to restart", &font)?;
            let dest_point = graphics::Point2::new((screen_width / 2 - 50) as f32, (screen_height/2 - 10) as f32);
            graphics::draw(ctx, &text, dest_point, 0.0)?;
        }
        


        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        // println!(
        //     "Key pressed: {:?}, modifier {:?}, repeat: {}, x: {} direction: {:?}",
        //     keycode, keymod, repeat, self.player.x, self.player.direction
        // );

        match keycode {
            Keycode::Left | Keycode::A => {
                self.player.direction = super::entity::Direction::Left;
            }
            Keycode::Right | Keycode::D => {
                self.player.direction = super::entity::Direction::Right;
            }
            Keycode::Space => {
                if ! self.player.alive {
                    return;
                }
                let bullet = super::entity::Bullet::new(
                    self.player.x,
                    self.player.y,
                    self.assets.player.width(),
                ).unwrap();
                self.bullets.push(bullet);
                self.bullet_state.can_shoot = false;
                self.bullet_state.can_shoot_timer = self.bullet_state.can_shoot_timer_max;
            }
            Keycode::R => {
                self.restart = true;
            }
                
            Keycode::Escape => _ctx.quit().unwrap(),

            _ => (), // do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        // println!(
        //     "Key pressed: {:?}, modifier {:?}, repeat: {}",
        //     keycode, keymod, repeat
        // );
        self.player.direction = super::entity::Direction::Idle
    }
}
