use bevy::app::App;
use bevy::prelude::*;
use crate::matrix::Matrix2D;
use crate::hints::{create_hints, Hints};

#[derive(Resource)]
struct Data(Matrix2D);

#[derive(Resource)]
struct Selected(Matrix2D);

pub struct SherlockPlugin;

impl Plugin for SherlockPlugin {
    fn build(&self, app: &mut App) {
        println!("SherlockPlugin build");
        let matrix = Matrix2D::new(8, 8);

        app.insert_resource(create_hints(3, 3, &matrix));
        app.insert_resource(Data(matrix));
        app.insert_resource(Selected(Matrix2D::new_with_default(8, 8, 0)));
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(_commands: Commands, mut data: ResMut<Data>, hints: ResMut<Hints>) {
    println!("setup_game");
    let matrix = &mut data.0;
    matrix.shuffle();

    for i in 0..matrix.rows {
        println!("{:?}", matrix.data[i]);
    }

    let hints_collection = &hints.0;
    for hint in hints_collection {
        println!("{:?}", hint);
    }
}