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



impl Solution {
    fn find_deepest_leaves_with_bfs(
        root: i32,
        n: i32,
        edges: &Vec<Vec<i32>>,
        leaves: &mut Vec<i32>>,
        height: i32,
        deepest: &mut i32,
    ) {
        for i in 0..n as usize - 1 {
            let mut the_other: i32 = -1;
            if edges[j][0] == root {
                the_other = edges[j][1];
            }
            else if edges[j][1] == root {
                the_other = edges[j][0]
            }
            if (the_other != -1) {
                if height < deepest - 1 {
                    return;
                }
                else if height == deepest - 1 {
                    leaves.push(the_other);
                    find_deepest_leaves_with_bfs(the_other, n, edges, leaves, height, deepest);
                }
                else {
                    deepest = height;
                    leaves.clear();
                    leaves.push(the_other);
                    find_deepest_leaves_with_bfs(the_other, n, edges, leaves, height, deepest);
                }
            }
        }
    }

    /*
    fn find_height(
        root: i32,
        n: i32,
        edges: &Vec<Vec<i32>>,
        height: i32,
    ) -> i32 {
        let mut height_ans: i32 = 0;
        for j in 0..n as usize - 1 {
            if edges[j][0] == root {
                let mut this_height: i32 = 0;
                this_height = Self::find_height(edges[j][1], n, edges, height + 1);
                if this_height > height_ans {
                    height_ans = this_height;
                }
            }
            else if edges[j][1] == root {
                let mut this_height: i32 = 0;
                this_height = Self::find_height(edges[j][0], n, edges, height + 1);
                if this_height > height_ans {
                    height_ans = this_height;
                }
            }
        }
        height + height_ans
    } 
    */

    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // min height tree的root应该在最长的一条通路上面
        // 先找到这个通路，然后遍历通路，得到每个节点做根的height
        // 是不是edges数量大于n-1就有环了？

        /* 首先可以通过两次 BFS（或 DFS）来找到树的直径。第一次 BFS 从任意节点开始，
        找到离它最远的节点，第二次 BFS 从这个最远的节点开始，找到离它最远的节点，
        这两个最远的节点之间的路径就是树的直径 */

        let mut ans: Vec<i32> = Vec::new();
        let mut heights: Vec<i32> = Vec::new();
        let mut leaves: Vec<i32> = Vec::new();
        let mut deepest: i32 = 0;
        Self::find_deepest_leaves_with_bfs(0, n, &edges, &mut leaves, 0, deepest);

        ans
    }
}
