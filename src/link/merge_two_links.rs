/*
将两个有序链表合并为一个新的有序链表并返回。新链表是由给
定的两个链表的所有节点拼接组成的。
(1)示例
输入:1→2→4, 1→3→4
输出:1→1→2→3→4→4
(2)链接 https://leetcode-cn.com/problems/merge-two-sorted-lists
*/


use crate::link::ListNode;

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    match (list1, list2) {
         (Some(node1), None) => Some(node1), // list2为空，返回list1的其余节点
         (None, Some(node2)) => Some(node2), // list1为空，返回list2 的其余节点
        (Some(mut node1), Some(mut node2)) => {

            if node1.val < node2.val {
                let n = node1.next.take();
                node1.next = merge_two_lists(n, Some(node2));
                Some(node1)
            }else {
                let n = node2.next.take();
                node2.next = merge_two_lists(Some(node1), n);
                Some(node2)
            }

        }
        _ => None,
    }
}
