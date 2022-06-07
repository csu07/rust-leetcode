/*

给定一个二叉树，返回它的中序遍历。 (1)示例
输入:[1, null, 2, 3]
输出:[1, 3, 2]
(2)链接
    https://leetcode-cn.com/problems/binary-tree-inorder-traversal
(1)递归思路
  先递归访问左子树，然后访问根节点，最后递归访问右子树，即
实现左子树→根节点→右子树的访问。
(2)非递归思路
  利用栈的数据结构特性来保存需要返回后处理的节点，优先遍历
左子树节点。在遍历过程中，将当前节点入栈，直到左子树为空。再将栈顶的节点出栈，并进入其右子树继续进行遍历。
  非递归实现的算法流程如下。
1)创建一个栈用来存放节点。
2)若当前节点非空，将当前节点入栈，并进入其左子树访问。
3)重复步骤2，直到当前节点为空。
4)将栈顶的节点出栈，访问其节点值，并进入其右子树访问。
5)重复步骤2~4，直到当前节点为空且栈为空，完成所有节点的 访问。
*/


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


pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 初始化返回结果
    let mut result = vec![];
    // 判断是不是空树
    if root.is_none() {
        return result;
    }

    // 递归函数
    recursion(root, &mut result);
    result
}

// ...
fn recursion(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 左树
            recursion(node.borrow().left.clone(), result);
            // 当前节点
            result.push(node.borrow().val);
            // 右树
            recursion(node.borrow().right.clone(), result);
        }
        None => {
            return;
        }
    }
}

