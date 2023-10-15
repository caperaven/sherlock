use crate::matrix::Matrix2D;

struct Cell {
    row: usize,
    col: usize
}

fn count_errors(matrix: &Matrix2D, selected: &Matrix2D) -> Vec<Cell> {
    let mut errors: Vec<Cell> = vec![];

    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            let value = matrix.get(i, j);
            let selected_value = selected.get(i, j);

            if value != selected_value {
                errors.push(Cell { row: i, col: j });
            }
        }
    }

    return errors;
}

#[cfg(test)]
mod tests {
    use super::Matrix2D;
    use super::count_errors;

    #[test]
    fn test_count_errors_no_errors() {
        let matrix = Matrix2D::new(8, 8);
        let selected = matrix.clone();

        let result = count_errors(&matrix, &selected);
        assert!(result.len() == 0);
    }

    #[test]
    fn test_count_errors_with_errors() {
        let matrix = Matrix2D::new(8, 8);
        let mut selected = matrix.clone();

        selected.set(0, 0, -1);
        selected.set(1, 1, -1);
        selected.set(2, 2, -1);
        selected.set(3, 3, -1);
        selected.set(4, 4, -1);
        selected.set(5, 5, -1);
        selected.set(6, 6, -1);
        selected.set(7, 7, -1);

        let result = count_errors(&matrix, &selected);
        assert!(result.len() == 8);
        assert!(result[0].row == 0);
        assert!(result[0].col == 0);
        assert!(result[1].row == 1);
        assert!(result[1].col == 1);
        assert!(result[2].row == 2);
        assert!(result[2].col == 2);
        assert!(result[3].row == 3);
        assert!(result[3].col == 3);
        assert!(result[4].row == 4);
        assert!(result[4].col == 4);
        assert!(result[5].row == 5);
        assert!(result[5].col == 5);
        assert!(result[6].row == 6);
        assert!(result[6].col == 6);
        assert!(result[7].row == 7);
        assert!(result[7].col == 7);
    }
}