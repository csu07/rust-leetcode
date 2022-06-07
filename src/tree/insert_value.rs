/*
给定二叉搜索树的根节点和要插入树中的值，将值插入二叉搜索
树，并返回插入值后二叉搜索树的根节点，保证原始二叉搜索树中不
存在新值。
(1)说明
  存在多种有效的插入方式可以返回任意有效的结果，只要树在插
入后仍为二叉搜索树即可。
(2)示例 给定二叉搜索树:
4 /\ 27
/\ 13
和 插入的值:5 可以返回这个二叉搜索树:
4 /\ 27
/\ / 1 35
或者这个树也是有效的:
5 /\ 27
/\ 13
\ 4
(3)链接
    https://leetcode-cn.com/problems/insert-into-a-binary-search-tree

(1)算法原理
如果二叉搜索树为空树，直接用val构造二叉树节点并将其作为根 节点返回。
如果要给新值找到合适的插入位置，可以将新节点作为当前二叉搜索树的某个叶子节点的子节点进行插入。插入到哪个叶子节点遵循
以下原则:
    若val > node.val，值插入右子树。
    若val < node.val，值插入左子树。
在遇到应该走向左子树而左子树为空，或者应该走向右子树而右子树为空时，该节点就是新节点的插入位置了。
2)算法流程
1)判断根节点是否为空，如果为空，直接返回TreeNode(val)。
2)继续查找节点，分为两种情况。
如果val > node.val，进入该节点的右子树查找。 如果val < node.val，进入该节点的左子树查找。
3)重复步骤2，直到节点为None，插入TreeNode(val)，返回根节点。
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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }

    fn insert(root: &Option<Rc<RefCell<TreeNode>>>, value: i32) {
        if let Some(node) = root {
            let mut n = node.borrow_mut();

            // value大于当前节点值，往右子树查找
           // value小于当前节点值，往左子树查找
            let target = if value > n.val {&mut n.right} else { &mut n.left };
            if target.is_some() {
                return insert(target, value);
            }
            *target = Some(Rc::new(RefCell::new(TreeNode::new(value))));
        }
    }


    insert(&root, val);
    root
}





