use rand::prelude::SliceRandom;

pub struct Matrix2D {
    pub data: Vec<Vec<i32>>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix2D {
    // Constructor to create a new Matrix2D of given rows and columns
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut result = Matrix2D::new_with_default(rows, cols, 0);

        for i in 1..rows + 1 {
            for j in 1..cols + 1 {
                result.data[i - 1][j - 1] = j as i32;
            }
        }

        return result;
    }

    pub fn new_with_default(rows: usize, cols: usize, default: i32) -> Self {
        let mut result = Matrix2D {
            data: vec![vec![default; cols]; rows],
            rows,
            cols,
        };

        return result;
    }

    // Method to get a value at a particular row and column
    pub fn get(&self, row: usize, col: usize) -> Option<i32> {
        if row < self.rows && col < self.cols {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    // Method to set a value at a particular row and column
    pub fn set(&mut self, row: usize, col: usize, value: i32) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            true
        } else {
            false
        }
    }

    // Method to suffle the columns for each row so that the value positions are randomized
    pub fn shuffle(&mut self) {
        for i in 0..self.rows {
            self.data[i].shuffle(&mut rand::thread_rng());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix2D;

    #[test]
    fn test_matrix_initialization() {
        let matrix = Matrix2D::new(3, 3);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(matrix.get(i, j), Some(j as i32 + 1));
            }
        }
    }

    #[test]
    fn test_matrix_set_and_get() {
        let mut matrix = Matrix2D::new(3, 3);
        assert!(matrix.set(1, 1, 5));
        assert_eq!(matrix.get(1, 1), Some(5));
    }

    #[test]
    fn test_matrix_get_out_of_bounds() {
        let matrix = Matrix2D::new(3, 3);
        assert_eq!(matrix.get(4, 1), None);
        assert_eq!(matrix.get(1, 4), None);
    }

    #[test]
    fn test_matrix_set_out_of_bounds() {
        let mut matrix = Matrix2D::new(3, 3);
        assert!(!matrix.set(4, 1, 5));
        assert!(!matrix.set(1, 4, 5));
    }
}