use bevy::prelude::Resource;
use rand::Rng;
use crate::matrix::Matrix2D;

#[derive(Resource)]
struct Hints(Vec<HintItem>);

enum HintType {
    Row,
    Column
}

// The structure of the hint type
struct HintItem {
    pub hint_type: HintType,
    // what is the row or column index
    pub index: i32,
    // an array of indexes that are part of the hint
    pub values: Vec<i32>
}

// This is the main function that generates hints.
// When generating hints for rows it will take a row from the matrix and generate a hint for it
// The items in the hint is in the right order as it is found in the matrix.
// There can however be gaps in the hint items.
// Lets say that we have an array of [0, 4, 2, 9, 8, 1, 3, 7, 6, 5]
// An row hint that is two items could be [0, 4] or [4, 1] ... etc
// An rwo hint that is three items could be [2, 1, 3] or [3, 7, 6] ... etc
// In order to do generate this we will need to use random numbers that follow a certain pattern
fn create_hint(hint_type: HintType, matrix: Matrix2D) -> HintItem {
    // 1. get a random number between 0 and 8 as a reference to the matrix.
    let mut rnd = rand::thread_rng();
    let index = rnd.gen_range(0..=7);
    let size = rnd.gen_range(2..=5);

    match hint_type {
        HintType::Row => create_row_hint(index, size, matrix),
        HintType::Column => create_column_hint(index, size, matrix)
    }
}

// create a row hint
// size indicates how many items the hint should have
fn create_row_hint(row_index: i32, size: i32, matrix: Matrix2D) -> HintItem {
    let result = HintItem {
        hint_type: HintType::Row,
        index: row_index,
        values: Vec::new()
    };

    return result;
}

// create column hint
// size indicates how many items the hint should have
fn create_column_hint(col_index: i32, size: i32, matrix: Matrix2D) -> HintItem {
    let result = HintItem {
        hint_type: HintType::Column,
        index: col_index,
        values: Vec::new()
    };

    return result;
}

fn get_random_indexes(size: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut rnd = rand::thread_rng();

    let mut min: i32 = 0;
    let mut max: i32 = 4;
    let mut size_offset: i32 = size - 1;

    for _ in 0..size {
        let index = rnd.gen_range(min..=max);
        result.push(index);

        size_offset -= 1;
        min = index + 1;
        max = 7 - size_offset;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_indexes() {
        let result2 = get_random_indexes(2);
        assert_eq!(result2.len(), 2);
        assert!(result2[0] < result2[1]);

        let result3 = get_random_indexes(3);
        assert_eq!(result3.len(), 3);
        assert!(result3[0] < result3[1] && result3[1] < result3[2]);
    }
}