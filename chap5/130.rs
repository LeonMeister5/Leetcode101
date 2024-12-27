/*
给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' 组成，捕获 所有 被围绕的区域：

连接：一个单元格与水平或垂直方向上相邻的单元格连接。
区域：连接所有 'O' 的单元格来形成一个区域。
围绕：如果您可以用 'X' 单元格 连接这个区域，并且区域中没有任何单元格位于 board 边缘，则该区域被 'X' 单元格围绕。
通过将输入矩阵 board 中的所有 'O' 替换为 'X' 来 捕获被围绕的区域。

 

示例 1：

输入：board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]

输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]

解释：


在上图中，底部的区域没有被捕获，因为它在 board 的边缘并且不能被围绕。

示例 2：

输入：board = [["X"]]

输出：[["X"]]

 

提示：

m == board.length
n == board[i].length
1 <= m, n <= 200
board[i][j] 为 'X' 或 'O'
*/



impl Solution {
    fn dfs_helper(
        board: &mut Vec<Vec<char>>,
        backup: &mut Vec<Vec<char>>,
        x: usize,
        y: usize,
    ) {
        if x >= board.len() 
            || y >= board[0].len()
            || backup[x][y] == 'O' {
            return;
        }
        backup[x][y] = 'O';
        let directions = [(1, 0), (0, 1), (usize::MAX, 0), (0, usize::MAX)]; 
        for &(dx, dy) in &directions {
            let new_x = x.wrapping_add(dx);
            let new_y = y.wrapping_add(dy);
            if new_x < board.len() && new_y < board[0].len() {
                if board[new_x][new_y] == 'O' {
                    Self::dfs_helper(board, backup, new_x, new_y);
                }
            }
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();
        if n == 0 || m == 0 {
            return;
        }
        let mut backup = vec![vec!['X'; m]; n];
        for i in 0..n {
            if board[i][0] == 'O' {
                Self::dfs_helper(board, &mut backup, i, 0);
            }
            if board[i][m - 1] == 'O' {
                Self::dfs_helper(board, &mut backup, i, m - 1);
            }
        }
        for i in 0..m {
            if board[0][i] == 'O' {
                Self::dfs_helper(board, &mut backup, 0, i);
            }
            if board[n - 1][i] == 'O' {
                Self::dfs_helper(board, &mut backup, n - 1, i);
            }
        }
        for i in 0..n {
            for j in 0..m {
                if backup[i][j] == 'O' {
                    board[i][j] = 'O';
                } else {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
