//! Wrapper for hot-reloadable plugin.
use guards_vanilla_game::game::Game;
use guards_vanilla_game::game::Plugin;

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(Game::default())
}
