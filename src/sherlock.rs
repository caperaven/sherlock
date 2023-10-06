use bevy::app::App;
use bevy::prelude::*;
use crate::matrix::Matrix2D;

#[derive(Resource)]
struct Data(Matrix2D);

pub struct SherlockPlugin;

impl Plugin for SherlockPlugin {
    fn build(&self, app: &mut App) {
        println!("SherlockPlugin build");
        app.insert_resource(Data(Matrix2D::new(8, 8)));
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(_commands: Commands, data: ResMut<Data>) {
    println!("setup_game");
    let matrix = &data.0;
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            print!("{}", j);
        }
        println!();
    }
}