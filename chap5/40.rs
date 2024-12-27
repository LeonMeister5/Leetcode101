/*
给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

candidates 中的每个数字在每个组合中只能使用 一次 。

注意：解集不能包含重复的组合。 

 

示例 1:

输入: candidates = [10,1,2,7,6,1,5], target = 8,
输出:
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]
示例 2:

输入: candidates = [2,5,2,1,2], target = 5,
输出:
[
[1,2,2],
[5]
]
 

提示:

1 <= candidates.length <= 100
1 <= candidates[i] <= 50
1 <= target <= 30
*/



use std::collections::HashSet;

impl Solution {
    fn helper(
        candidates: &Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        target: i32,
        level: usize,
        stack_temp: &mut Vec<i32>,
    ) {
        let sum = stack_temp.iter().sum::<i32>();
        if sum == target {
            ans.push(stack_temp.clone());
            return;
        } else if sum > target {
            return;
        }

        for i in level..candidates.len() {
            if i > level && candidates[i] == candidates[i - 1] {
                continue;
            }
            stack_temp.push(candidates[i]);
            Self::helper(candidates, ans, target, i + 1, stack_temp);
            stack_temp.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        candidates.sort();
        let mut stack_temp = Vec::new();
        Self::helper(&candidates, &mut ans, target, 0, &mut stack_temp);
        ans
    }
}


/* 写成排列了，题目要的是组合
use std::collections::HashSet;
impl Solution {
    fn helper(
        candidates: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
        target: i32,
        level: usize,
    ) {
        if level == candidates.len() {
            return;
        }
        let sum = candidates[..level].iter().sum::<i32>();
        if sum == target {
            ans.push(candidates[..level].to_vec());
            return;
        }
        else if sum > target {
            return;
        }
        let mut seen = HashSet::new();
        for i in level..candidates.len() {
            if seen.contains(&candidates[i]) {
                continue;
            }
            seen.insert(candidates[i]);
            candidates.swap(level, i);
            Self::helper(candidates, ans, target, level + 1);
            candidates.swap(level, i);
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if candidates.iter().sum::<i32>() == target {
            ans.push(candidates);
            return ans;
        }
        candidates.sort();
        Self::helper(&mut candidates, &mut ans, target, 0);
        ans
    }
}
*/
