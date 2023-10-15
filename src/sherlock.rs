use bevy::app::App;
use bevy::prelude::*;
use crate::matrix::Matrix2D;
use crate::hints::{create_hints, Hints};
use crate::settings::Settings;

#[derive(Resource)]
struct Data(Matrix2D);

#[derive(Resource)]
struct Selected(Matrix2D);

pub struct SherlockPlugin;

impl Plugin for SherlockPlugin {
    fn build(&self, app: &mut App) {
        let settings = Settings::new();

        // Make a copy of the settings values so we can assign it to the resources
        let row_tile_count: usize = settings.row_tile_count as usize;
        let col_tile_count: usize = settings.col_tile_count as usize;
        let select_row_count: usize = settings.row_hint_count as usize;
        let select_col_count: usize = settings.col_hint_count as usize;
        let hint_row_count: i32 = settings.row_hint_count;
        let hint_col_count: i32 = settings.col_hint_count; 

        let matrix = Matrix2D::new(row_tile_count, col_tile_count);
        let selection_matrix = Matrix2D::new_with_default(select_row_count, select_col_count, 0);

        app.insert_resource(settings);
        app.insert_resource(create_hints(hint_row_count, hint_col_count, &matrix));
        app.insert_resource(Data(matrix));
        app.insert_resource(Selected(selection_matrix));
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