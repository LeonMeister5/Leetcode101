/*
给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。

 

示例 1：

输入：nums = [1,1,2]
输出：
[[1,1,2],
 [1,2,1],
 [2,1,1]]
示例 2：

输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 

提示：

1 <= nums.length <= 8
-10 <= nums[i] <= 10
*/



use std::collections::HashSet;
impl Solution {
    fn helper(
        nums: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        level: usize,
    ) {
        if level == nums.len() {
            ans.push(nums.clone());
            return;
        }
        for i in level..=nums.len() {
            nums.swap(level, i);
            Self::helper(nums, ans, level + 1);
            nums.swap(level, i);
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if n == 1 {
            ans.push(nums);
            return ans;
        }
        let mut nums_copy = nums.clone();
        Self::helper(&mut nums_copy, &mut ans, 0);
        let set: HashSet<_> = ans.drain(..).collect(); // 将 Vec 转为 HashSet
        ans.extend(set.into_iter());
        ans
    }
}
