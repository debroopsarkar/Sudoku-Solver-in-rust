pub struct Sudoku {
    pub board: [[u32; 9]; 9],
    pub steps: Vec<[[u32; 9]; 9]>,
}

impl Sudoku {
    pub fn new(board: [[u32; 9]; 9]) -> Self {
        Self {
            board,
            steps: vec![],
        }
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == 0 {
                    for num in 1..=9 {
                        if self.is_safe(row, col, num) {
                            self.board[row][col] = num;
                            self.steps.push(self.board.clone()); // Store each state for visualization

                            if self.solve() {
                                return true; // Recursively solve the rest of the board
                            }

                            self.board[row][col] = 0; // Backtrack
                        }
                    }
                    return false; // Trigger backtracking
                }
            }
        }
        true // Solved
    }

    fn is_safe(&self, row: usize, col: usize, num: u32) -> bool {
        // Check if num is in the row
        for x in 0..9 {
            if self.board[row][x] == num {
                return false;
            }
        }

        // Check if num is in the column
        for x in 0..9 {
            if self.board[x][col] == num {
                return false;
            }
        }

        // Check if num is in the 3x3 box
        let box_row_start = row - row % 3;
        let box_col_start = col - col % 3;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[i + box_row_start][j + box_col_start] == num {
                    return false;
                }
            }
        }
        true
    }
}
