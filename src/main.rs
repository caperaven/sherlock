mod matrix;
mod sherlock;
mod hints;

use bevy::prelude::*;
use crate::sherlock::SherlockPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SherlockPlugin))
        .run();
}