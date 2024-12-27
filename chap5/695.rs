/*
给你一个大小为 m x n 的二进制矩阵 grid 。

岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在 水平或者竖直的四个方向上 相邻。你可以假设 grid 的四个边缘都被 0（代表水）包围着。

岛屿的面积是岛上值为 1 的单元格的数目。

计算并返回 grid 中最大的岛屿面积。如果没有岛屿，则返回面积为 0 。

 

示例 1：


输入：grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
输出：6
解释：答案不应该是 11 ，因为岛屿只能包含水平或垂直这四个方向上的 1 。
示例 2：

输入：grid = [[0,0,0,0,0,0,0,0]]
输出：0
*/

impl Solution {
    fn dfs_helper(
        grid: &Vec<Vec<i32>>,       // 不可变引用：表示不会修改原 grid
        visited: &mut Vec<Vec<bool>>, // 可变引用：表示可以修改 visited
        x: usize,                  // 不可变的 x 坐标
        y: usize,                  // 不可变的 y 坐标
        current_area: &mut i32,         // 当前已有的点的数量
    ) {
        // 函数体中实现逻辑
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x < 0 || new_x >= grid.len() as isize || new_y < 0 || new_y >= grid[0].len() as isize {
                continue;
            }
            else if visited[new_x as usize][new_y as usize] {
                continue;
            }
            else {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                visited[new_x][new_y] = true;
                if grid[new_x][new_y] == 1 {
                    *current_area += 1;
                    Self::dfs_helper(grid, visited, new_x, new_y, current_area);
                }
            }
        }
    }
    
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        let mut max: i32 = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;
                let mut temp_max: i32 = 0;
                if grid[i][j] == 1 {
                    temp_max += 1;
                    Self::dfs_helper(&grid, &mut visited, i, j, &mut temp_max);
                }
                if temp_max > max {
                    max = temp_max;
                }
            }
        }
        max
    }
}
