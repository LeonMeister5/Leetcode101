/*
树是一个无向图，其中任何两个顶点只通过一条路径连接。 换句话说，任何一个没有简单环路的连通图都是一棵树。

给你一棵包含 n 个节点的树，标记为 0 到 n - 1 。给定数字 n 和一个有 n - 1 条无向边的 edges 列表（每一个边都是一对标签），其中 edges[i] = [ai, bi] 表示树中节点 ai 和 bi 之间存在一条无向边。

可选择树中任何一个节点作为根。当选择节点 x 作为根节点时，设结果树的高度为 h 。在所有可能的树中，具有最小高度的树（即，min(h)）被称为 最小高度树 。

请你找到所有的 最小高度树 并按 任意顺序 返回它们的根节点标签列表。

树的 高度 是指根节点和叶子节点之间最长向下路径上边的数量。
 

示例 1：


输入：n = 4, edges = [[1,0],[1,2],[1,3]]
输出：[1]
解释：如图所示，当根是标签为 1 的节点时，树的高度是 1 ，这是唯一的最小高度树。
示例 2：


输入：n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
输出：[3,4]
 

提示：

1 <= n <= 2 * 104
edges.length == n - 1
0 <= ai, bi < n
ai != bi
所有 (ai, bi) 互不相同
给定的输入 保证 是一棵树，并且 不会有重复的边
*/



use std::collections::{VecDeque, HashMap};
impl Solution { // 不解释了看笔记p1
    fn find_deepest_leaf_with_bfs(
        root: i32,
        n: i32,
        adj: &Vec<Vec<i32>>,
    ) -> (i32, Vec<i32>) {
        let mut dist = vec![-1; n as usize];
        let mut parent = vec![-1; n as usize];
        let mut queue = VecDeque::new();
        queue.push_back(root);
        dist[root as usize] = 0;  
        while let Some(node) = queue.pop_front() {
            for &neighbor in &adj[node as usize] {
                if dist[neighbor as usize] == -1 {
                    dist[neighbor as usize] = dist[node as usize] + 1;
                    parent[neighbor as usize] = node;
                    queue.push_back(neighbor);
                }
            }
        }  
        let mut max_dist = -1;
        let mut deepest_leaf = -1;
        for i in 0..n {
            if dist[i as usize] > max_dist {
                max_dist = dist[i as usize];
                deepest_leaf = i;
            }
        }
        let mut path = Vec::new();
        let mut current = deepest_leaf;
        while current != -1 {
            path.push(current);
            current = parent[current as usize];
        }
        path.reverse();
        (deepest_leaf, path)
    }

    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut adj = vec![Vec::new(); n as usize];
        for edge in edges {
            adj[edge[0] as usize].push(edge[1]);
            adj[edge[1] as usize].push(edge[0]);
        }
        let (one_end, _) = Self::find_deepest_leaf_with_bfs(0, n, &adj);
        let (_, path) = Self::find_deepest_leaf_with_bfs(one_end, n, &adj);
        let mut ans: Vec<i32> = Vec::new();
        let len = path.len();
        if len % 2 == 0 {
            ans.push(path[len/2 - 1]);
            ans.push(path[len/2]);
        }
        else {
            ans.push(path[len/2]);
        }
        ans
    }
}
