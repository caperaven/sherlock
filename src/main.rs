mod matrix;
mod sherlock;
mod hints;
mod settings;
mod count_errors;

use bevy::prelude::*;
use crate::sherlock::SherlockPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SherlockPlugin))
        .run();
}