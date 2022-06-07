/*

给定一个二叉树，返回它的后序遍历。 (1)示例
输入:[1, null, 2, 3]
输出:[3, 2, 1]
(2)链接
    https://leetcode-cn.com/problems/binary-tree-postorder-traversal
 */


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 定义返回值
    let mut result = vec![];
    // 判断是否是空树
    if root.is_none() {
        return result;
    }

    fn recursion(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match root {
            Some(node) => {
                // 遍历左树
                recursion(node.borrow().left.clone(), result);
                // 右树
                recursion(node.borrow().right.clone(), result);
                // 当前节点
                result.push(node.borrow().val);
            }
            None => {
                return;
            }
        }
    }
    recursion(root, &mut result);
    result
}