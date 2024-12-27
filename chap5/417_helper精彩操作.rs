/*
有一个 m × n 的矩形岛屿，与 太平洋 和 大西洋 相邻。 “太平洋” 处于大陆的左边界和上边界，而 “大西洋” 处于大陆的右边界和下边界。

这个岛被分割成一个由若干方形单元格组成的网格。给定一个 m x n 的整数矩阵 heights ， 
heights[r][c] 表示坐标 (r, c) 上单元格 高于海平面的高度 。

岛上雨水较多，如果相邻单元格的高度 小于或等于 当前单元格的高度，雨水可以直接向北、南、东、西流向相邻单元格。
水可以从海洋附近的任何单元格流入海洋。

返回网格坐标 result 的 2D 列表 ，其中 result[i] = [ri, ci] 表示雨水从单元格 (ri, ci) 流动 既可流向太平洋也可流向大西洋 。

 

示例 1：



输入: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
输出: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
示例 2：

输入: heights = [[2,1],[1,2]]
输出: [[0,0],[0,1],[1,0],[1,1]]
 

提示：

m == heights.length
n == heights[r].length
1 <= m, n <= 200
0 <= heights[r][c] <= 105
*/



impl Solution {
    fn helper(
        heights: &Vec<Vec<i32>>,
        visited_ocean: &mut Vec<Vec<bool>>,
        to_ocean: &mut Vec<Vec<bool>>,
        index_a: usize,
        index_b: usize,
    ) {
        if index_a >= heights.len() 
            || index_b >= heights[0].len() 
            || visited_ocean[index_a][index_b] {
            return;
        }
        visited_ocean[index_a][index_b] = true;
        let directions = [(1, 0), (0, 1), (usize::MAX, 0), (0, usize::MAX)]; 
        // 使用 usize 溢出表示 -1
        for &(da, db) in &directions {
            let new_a = index_a.wrapping_add(da); // 使用 wrapping_add 处理溢出
            let new_b = index_b.wrapping_add(db);
            if new_a < heights.len() && new_b < heights[0].len() {
                if heights[new_a][new_b] >= heights[index_a][index_b] {
                    to_ocean[new_a][new_b] = true;
                    Self::helper(heights, visited_ocean, to_ocean, new_a, new_b)
                }
            }
        }
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut visited_pacific = vec![vec![false; n]; m];
        let mut visited_atlantic = vec![vec![false; n]; m];
        let mut to_pacific = vec![vec![false; n]; m];
        let mut to_atlantic = vec![vec![false; n]; m];
        for i in 0..m {
            to_pacific[i][0] = true;
            to_atlantic[i][n - 1] = true; 
        }
        for i in 0..n {
            to_pacific[0][i] = true;
            to_atlantic[m - 1][i] = true;
        }
        for i in 0..m {
            Self::helper(&heights, &mut visited_pacific, &mut to_pacific, i, 0);
            Self::helper(&heights, &mut visited_atlantic, &mut to_atlantic, i, n - 1);
        }
        for i in 0..n {
            Self::helper(&heights, &mut visited_pacific, &mut to_pacific, 0, i);
            Self::helper(&heights, &mut visited_atlantic, &mut to_atlantic, m - 1, i);
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..m {
            for j in 0..n{
                if to_pacific[i][j] && to_atlantic[i][j] {
                    let temp = vec![i as i32, j as i32];
                    ans.push(temp);
                }
            }
        }
        ans
    }
}
