use rand::prelude::SliceRandom;

#[derive(Eq, PartialEq, Debug, Clone)]
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
        let result = Matrix2D {
            data: vec![vec![default; cols]; rows],
            rows,
            cols,
        };

        return result;
    }

    pub fn get_row(&self, row: usize) -> &Vec<i32> {
        &self.data[row]
    }

    pub fn get_column(&self, col: usize) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        
        for i in 0..self.rows {
            result.push(self.data[i][col]);
        }

        result
    }

    // Method to get a value at a particular row and column
    pub fn get(&self, row: usize, col: usize) -> i32 {
        self.data[row][col]
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

    // Search the matrix and if any of the cells is empty return false
    pub fn is_complete(&self) -> bool {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = self.get(i, j);
                if value == 0 {
                    return false;
                }
            }
        }

        return true;
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
                assert_eq!(matrix.get(i, j), j as i32 + 1);
            }
        }
    }

    #[test]
    fn test_matrix_set_and_get() {
        let mut matrix = Matrix2D::new(3, 3);
        assert!(matrix.set(1, 1, 5));
        assert_eq!(matrix.get(1, 1), 5);
    }

    #[test]
    fn test_matrix_set_out_of_bounds() {
        let mut matrix = Matrix2D::new(3, 3);
        assert!(!matrix.set(4, 1, 5));
        assert!(!matrix.set(1, 4, 5));
    }

    #[test]
    fn test_is_complete() {
        let mut matrix = Matrix2D::new(3, 3);
        assert_eq!(matrix.is_complete(), true);

        matrix.set(0, 0, 0);
        assert_eq!(matrix.is_complete(), false);
    }
}