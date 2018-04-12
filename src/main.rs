extern crate ggez;
use ggez::*;
use std::env;
use std::path;

extern crate first_ggez;
use first_ggez::state;

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

    let state = &mut state::MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
