/*
给你一个二叉树的根节点 root ，按 任意顺序 ，返回所有从根节点到叶子节点的路径。

叶子节点 是指没有子节点的节点。

 
示例 1：


输入：root = [1,2,3,null,5]
输出：["1->2->5","1->3"]
示例 2：

输入：root = [1]
输出：["1"]
 

提示：

树中节点的数目在范围 [1, 100] 内
-100 <= Node.val <= 100
*/



// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn dfs_with_recall_helper(
        ans: &mut Vec<String>,
        node: Option<Rc<RefCell<TreeNode>>>,
        path: &mut String,
    ) {
        if let Some(node_rc) = node {
            let node_ref = node_rc.borrow();

            if node_ref.left.is_none() && node_ref.right.is_none() {
                ans.push(path.clone());
                return;
            }

            let mut path1 = path.clone();

            if node_ref.left.is_some() {
                path.push_str("->");
                path.push_str(&node_ref.left.as_ref().unwrap().borrow().val.to_string());
                Self::dfs_with_recall_helper(ans, node_ref.left.clone(), path);
                path.truncate(path.len() - 2 - node_ref.left.as_ref().unwrap().borrow().val.to_string().len());
            }

            if node_ref.right.is_some() {
                path1.push_str("->");
                path1.push_str(&node_ref.right.as_ref().unwrap().borrow().val.to_string());
                Self::dfs_with_recall_helper(ans, node_ref.right.clone(), &mut path1);
            }
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();

        if root.is_none() {
            return ans;
        }

        if let Some(root_rc) = root {
            let root_ref = root_rc.borrow();
            let mut path = root_ref.val.to_string();
            Self::dfs_with_recall_helper(&mut ans, Some(root_rc.clone()), &mut path);
        }

        ans
    }
}
