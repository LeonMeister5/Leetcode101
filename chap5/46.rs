impl Solution {
    fn helper(
        n: i32,
        k: i32,
        start: i32,
        buf: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if buf.len() == k as usize {
            ans.push(buf.clone());
            return;
        }

        for i in start..=n {
            buf.push(i);
            Self::helper(n, k, i + 1, buf, ans); // 递归从下一个数字开始
            buf.pop(); // 回溯
        }
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut buf: Vec<i32> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        Self::helper(n, k, 1, &mut buf, &mut ans); // 从 1 开始
        ans
    }
}
