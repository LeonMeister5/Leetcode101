/*
有 n 个城市，其中一些彼此相连，另一些没有相连。如果城市 a 与城市 b 直接相连，且城市 b 与城市 c 直接相连，那么城市 a 与城市 c 间接相连。

省份 是一组直接或间接相连的城市，组内不含其他没有相连的城市。

给你一个 n x n 的矩阵 isConnected ，其中 isConnected[i][j] = 1 表示第 i 个城市和第 j 个城市直接相连，而 isConnected[i][j] = 0 表示二者不直接相连。

返回矩阵中 省份 的数量。

 

示例 1：


输入：isConnected = [[1,1,0],[1,1,0],[0,0,1]]
输出：2
示例 2：


输入：isConnected = [[1,0,0],[0,1,0],[0,0,1]]
输出：3
 

提示：

1 <= n <= 200
n == isConnected.length
n == isConnected[i].length
isConnected[i][j] 为 1 或 0
isConnected[i][i] == 1
isConnected[i][j] == isConnected[j][i]
*/



// Rust类型很抽象，用GPT狠狠改了一下类型
impl Solution {
    fn dfs_helper(
        is_connected: &Vec<Vec<i32>>,
        visited_path: &mut Vec<Vec<bool>>,
        visited_city: &mut Vec<bool>,
        visiting_city: usize, // 修改为 usize 类型
        province_count: &mut i32,
    ) {
        for path in 0..is_connected.len() {
            if is_connected[visiting_city][path] == 1 {
                visited_path[visiting_city][path] = true; // 修正访问方式
                if visited_city[path] {
                    continue;
                }
                visited_city[path] = true;
                // 递归调用时传递 visiting_city 的值（已是 usize）
                Self::dfs_helper(is_connected, visited_path, visited_city, path, province_count);
            } else {
                visited_path[visiting_city][path] = true;
            }
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len(); // 城市数量
        let mut province_count: i32 = 0;
        let mut visited_path = vec![vec![false; n]; n];
        let mut visited_city = vec![false; n];
        
        let mut visiting_city = 0; // 改为 usize 类型，表示城市的索引
        
        if n == 1 {
            return 1; // 只有一个城市，直接返回 1
        }
        
        // 开始检查城市
        while visiting_city < n {
            if visited_city[visiting_city] {
                visiting_city += 1;
                continue;
            }
            
            province_count += 1; // 找到一个新的省份
            Self::dfs_helper(&is_connected, &mut visited_path, &mut visited_city, visiting_city, &mut province_count);
            
            visiting_city += 1;
        }
        
        province_count
    }
}
