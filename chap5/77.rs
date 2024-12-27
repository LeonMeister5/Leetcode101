/*
给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。

你可以按 任何顺序 返回答案。

 

示例 1：

输入：n = 4, k = 2
输出：
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]
示例 2：

输入：n = 1, k = 1
输出：[[1]]
*/



impl Solution {
    fn helper(n: i32,
        k: i32,
        level: i32,
        buf: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if level == k {
            ans.push(buf.clone());
            return;
        }
        for i in level..n {
            buf.swap(i, level);
            Self::helper(n, k, level, &mut buf, &mut ans);
            buf.swap(i, level);
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut n_mut = n;
        let mut buf: Vec<i32> = Vec::new();
        for i in 0..n {
            buf.push(i);
        }
        let mut ans: Vec<Vec<i32>> = Vec::new();
        Self::helper(n, k, 0, &mut buf, &mut ans);
        ans
    }
}
