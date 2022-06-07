/*
给定一个二叉树，返回它的前序遍历。
(1)示例
输入:[1, null, 2, 3]
输出:[1, 2, 3]
(2)链接
    https://leetcode-cn.com/problems/binary-tree-preorder-traversal

二叉树的遍历通常有两种思路，一种是使用递归方式，另一种是使用非递归方式。下面对这两种思路分别进行介绍。
1)递归思路
  先访问根节点，然后递归访问左子树，最后递归访问右子树，即实现根节点→左子树→右子树的访问。
2)非递归
  利用栈的数据结构特性来保存需要返回后处理的节点，优先遍历当前节点和左子树节点。在遍历过程中将当前节点入栈，
  直到左子树为空。再将栈顶的节点出栈，并进入其右子树继续进行遍历。
  非递归实现的算法流程如下：
    1)创建一个栈用来存放节点。
    2)若当前节点非空，访问当前节点值，再将当前节点入栈，并进入其左子树访问。
    3)重复步骤2，直到当前节点为空。
    4)将栈顶的节点出栈，并进入其右子树访问。
    5)重复步骤2~4，直到当前节点为空且栈为空，完成所有节点的访问。

   Rc<T>允许数据有多个所有者。对应于树中的节点，它可 能既是某个节点的子节点，又是另一个节点的父节点。
   但是，Rc<T>只 能提供数据的不可变访问，而在树中插入节点是常见的操作，
   插入节点时要求节点的左子节点和右子节点是可变的，这就需要RefCell<T> 来让节点具备内部可变性。
   因此，Rc<RefCell<T>>表面上是不可变 的，但可以通过RefCell<T>的内部可变性在需要时修改数据，
   使得节点可以有多个所有者并且能被修改。树的节点的结构体定义如下所示。

*/


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    // 左节点
    pub left: Option<Rc<RefCell<TreeNode>>>,
    // 右节点
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


pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
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

// 定义递归函数, 前序遍历 根->左->右；中序遍历 左->根->右；后序遍历 左->右->根
fn recursion(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 当前节点
            result.push(node.borrow().val);
            // 左树
            recursion(node.borrow().left.clone(), result);
            // 右树
            recursion(node.borrow().right.clone(), result);
        }
        None => {
            return;
        }
    }
}


pub fn preorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 初始化返回结果
    let mut result = vec![];
    // 判断是不是空树
    if root.is_none() {
        return result;
    }
    // 使用栈来保存需要返回后处理的节点
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();

    while r.is_none() || !stack.is_empty() {
        // 当前节点非空，访问当前节点值，将当前节点入栈，并进入其左子树访问
        while let Some(node) = r {
            result.push(node.borrow().val);
            stack.push(node.clone());
            r = node.borrow().left.clone();
        }
        // 栈顶出栈访问右子树
        r = stack.pop();
        if let Some(node) = r {
            r = node.borrow().right.clone();
        }
    }
    result
}
