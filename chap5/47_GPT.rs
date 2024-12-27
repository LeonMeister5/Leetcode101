use std::collections::HashSet;

impl Solution {
    fn helper(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, level: usize) {
        if level == nums.len() {
            ans.push(nums.clone());
            return;
        }

        let mut seen = HashSet::new(); // 用于记录本层已经处理过的数字
        for i in level..nums.len() {
            if seen.contains(&nums[i]) {
                continue; // 如果当前数字在本层已经处理过，则跳过
            }
            seen.insert(nums[i]);
            nums.swap(level, i); // 选择
            Self::helper(nums, ans, level + 1); // 递归
            nums.swap(level, i); // 撤销选择
        }
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        nums.sort(); // 先排序，使得重复元素相邻
        Self::helper(&mut nums, &mut ans, 0);
        ans
    }
}
