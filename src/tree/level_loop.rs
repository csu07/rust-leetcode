/*
给定一个二叉树，返回其按层次遍历的节点值。 (1)说明 层次遍历即逐层地从左到右访问所有节点。 (2)示例
给定二叉树:[3,9,20,null,null,15,7]
 3
/ \
9 20
  / \
  15 7
返回其层次遍历结果:
[
[3],
[9, 20],
[15, 7] ]
(3)链接
    https://leetcode-cn.com/problems/binary-tree-level-order-traversal

要进行自上而下、从左到右的层次遍历，我们可以利用队列的结构特性来保存需要返回后处理的节点。
从根节点开始，按照层次依次将节点放入队列中。先将根节点入队列，当队列不为空时，重复执
行:队列头部的节点出队列，访问该节点，如果该节点有左子节点，
则将左子节点入队列;如果该节点有右子节点，则将右子节点入队
列。也就是说，在将队列中所有节点出队列的同时，将下一层的节点
入队列。
(2)算法流程
1)创建一个队列用来存放节点，并先将根节点入队列。
2)计算当前层中的元素个数(即队列长度)，开始遍历队列。
3)让队列头部的节点出队列，访问该节点，同时将该节点的左、 右子节点入队列。
4)重复步骤3，直到遍历完当前层的节点。 5)重复步骤2~4，直到队列为空，完成所有节点的访问。
    
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
use std::collections::VecDeque;

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    // 初始化返回值
    let mut result: Vec<Vec<i32>> = vec![];
    // 判断是否是空树
    if root.is_none() {
        return result;
    }

    // 初始化一个队列
    let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

    // 将root 节点放入队列
    deque.push_back(root);

    // 遍历队列
    while !deque.is_empty() {
        // 当前层
        let mut current_level = vec![];
        // 当前层中的个数
        let level_num = deque.len();
        for _ in 0..level_num {
            let n = deque.pop_front();

            if let Some(Some(node)) = n {
                // 添加当前节点
                current_level.push(node.borrow().val);

                // 将当前节点的左右节点加入队列
                if node.borrow().left.is_some() {
                    deque.push_back(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    deque.push_back(node.borrow().right.clone());
                }
            }
        }

        result.push(current_level);
    }


    result
}