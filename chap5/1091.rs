/*
给你一个 n x n 的二进制矩阵 grid 中，返回矩阵中最短 畅通路径 的长度。如果不存在这样的路径，返回 -1 。

二进制矩阵中的 畅通路径 是一条从 左上角 单元格（即，(0, 0)）到 右下角 单元格（即，(n - 1, n - 1)）的路径，该路径同时满足下述要求：

路径途经的所有单元格的值都是 0 。
路径中所有相邻的单元格应当在 8 个方向之一 上连通（即，相邻两单元之间彼此不同且共享一条边或者一个角）。
畅通路径的长度 是该路径途经的单元格总数。

 

示例 1：


输入：grid = [[0,1],[1,0]]
输出：2
示例 2：


输入：grid = [[0,0,0],[1,1,0],[1,1,0]]
输出：4
示例 3：

输入：grid = [[1,0,0],[1,1,0],[1,1,0]]
输出：-1
 

提示：

n == grid.length
n == grid[i].length
1 <= n <= 100
grid[i][j] 为 0 或 1
*/



use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        // 如果起点或终点是障碍物，直接返回 -1
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        // 定义八个方向
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),         (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        // BFS 队列
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, 1)); // (x, y, path_length)

        // 标记访问
        let mut visited = vec![vec![false; n]; n];
        visited[0][0] = true;

        while let Some((x, y, path_len)) = queue.pop_front() {
            // 如果到达终点，返回路径长度
            if x == n as i32 - 1 && y == n as i32 - 1 {
                return path_len;
            }

            // 遍历八个方向
            for (dx, dy) in directions.iter() {
                let nx = x + dx;
                let ny = y + dy;

                // 检查边界和是否可行
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] == 0 && !visited[nx as usize][ny as usize] {
                    visited[nx as usize][ny as usize] = true;
                    queue.push_back((nx, ny, path_len + 1));
                }
            }
        }

        // 如果无法到达终点，返回 -1
        -1
    }
}
