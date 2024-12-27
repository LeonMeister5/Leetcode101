/*
编写一个程序，通过填充空格来解决数独问题。

数独的解法需 遵循如下规则：

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）
数独部分空格内已填入了数字，空白格用 '.' 表示。

 

示例 1：


输入：board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
输出：[["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
解释：输入的数独如上图所示，唯一有效的解决方案如下所示：


 

提示：

board.length == 9
board[i].length == 9
board[i][j] 是一位数字或者 '.'
题目数据 保证 输入数独仅有一个解
*/



impl Solution {
    fn fill_a_number_recursively(
        board: &mut Vec<Vec<char>>,
        filled_row: &mut Vec<Vec<bool>>,
        filled_col: &mut Vec<Vec<bool>>,
        filled_square: &mut Vec<Vec<bool>>,
        mut unfilled: Vec<usize>,
    ) -> bool {
        if unfilled.is_empty() {
            return true;
        }
        let id = unfilled.pop().unwrap();
        let i = id / 9;
        let j = id % 9;
        for num in 0..9 {
            if !filled_row[i][num] && !filled_col[j][num] && !filled_square[(i / 3) * 3 + (j / 3)][num] {
                board[i][j] = (num + 1 + '0' as usize) as u8 as char;
                filled_row[i][num] = true; 
                filled_col[j][num] = true; 
                filled_square[(i / 3) * 3 + (j / 3)][num] = true;
                if Self::fill_a_number_recursively(board, filled_row, filled_col, filled_square, unfilled.clone()) {
                    return true;
                }
                filled_row[i][num] = false;
                filled_col[j][num] = false;
                filled_square[(i / 3) * 3 + (j / 3)][num] = false;
                board[i][j] = '.';
            }
        }
        false
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut filled_row = vec![vec![false; 9]; 9];
        let mut filled_col = vec![vec![false; 9]; 9];
        let mut filled_square = vec![vec![false; 9]; 9];
        let mut unfilled: Vec<usize> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let num = (board[i][j] as usize) - ('1' as usize);
                    filled_row[i][num] = true;
                    filled_col[j][num] = true;
                    filled_square[(i / 3) * 3 + (j / 3)][num] = true;
                } else {
                    unfilled.push(i * 9 + j);
                }
            }
        }
        Self::fill_a_number_recursively(board, &mut filled_row, &mut filled_col, &mut filled_square, unfilled.clone());
    }
}
