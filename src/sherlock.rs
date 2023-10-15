use bevy::app::App;
use bevy::prelude::*;
use rand::Rng;
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
        let hint_row_count: i32 = settings.row_hint_count;
        let hint_col_count: i32 = settings.col_hint_count; 

        let mut matrix = Matrix2D::new(row_tile_count, col_tile_count);
        matrix.shuffle();

        let mut selection_matrix = matrix.clone();
        set_selection_gaps(&mut selection_matrix, settings.gap_count);

        app.insert_resource(settings);
        app.insert_resource(create_hints(hint_row_count, hint_col_count, &matrix));
        app.insert_resource(Data(matrix));
        app.insert_resource(Selected(selection_matrix));
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(_commands: Commands, data: Res<Data>, hints: Res<Hints>, selection: Res<Selected>) {
    println!("setup_game");
    let matrix = &data.0;
    let selected_matrix = &selection.0;

    print!("Matrix: \n");
    for i in 0..matrix.rows {
        println!("{:?}", matrix.data[i]);
    }

    print!("Selected: \n");
    for i in 0..selected_matrix.rows {
        println!("{:?}", selected_matrix.data[i]);
    }

    // let hints_collection = &hints.0;
    // for hint in hints_collection {
    //     println!("{:?}", hint);
    // }
}

fn set_selection_gaps(selection: &mut Matrix2D, count: i32) {
    let mut rnd = rand::thread_rng();

    let row_count = selection.rows - 1;
    let col_count = selection.cols- 1;

    for _ in 0..count {
        let row = rnd.gen_range(0..=row_count);
        let col = rnd.gen_range(0..=col_count);
        selection.set(row, col, 0);
    }
}