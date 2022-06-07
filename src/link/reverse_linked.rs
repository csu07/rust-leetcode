/*
反转一个单链表。
(1)示例
输入:1→2→3→4→5→NULL
输出:5→4→3→2→1→NULL

(2)链接
https://leetcode-cn.com/problems/reverse-linked-list
*/


use crate::link::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut pre = None;
    let mut curr = head;

    while let Some(mut curr_node) = curr.take() {
        // 保存当前节点的下一个节点
        let next_temp = curr_node.next.take();
        // 将当前节点指向pre节点
        curr_node.next = pre.take();
        // pre和curr分别往后移动一个节点，即
         // 把当前节点curr_node赋值给pre
         // 把之前保存的当前节点的下一个节点next_temp赋值给 curr
        pre = Some(curr_node);
        curr = next_temp;

    }
    pre
}