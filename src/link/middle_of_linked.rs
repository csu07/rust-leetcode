/*
给定一个带有头节点head的非空单链表，返回链表的中间节点。 如果有两个中间节点，则返回第二个中间节点。
(1)说明
给定链表的节点数介于1和100之间。
(2)示例
输入:[1, 2, 3, 4, 5]
输出:此列表中的节点3(序列化形式:[3, 4, 5])
输入:[1, 2, 3, 4, 5, 6]
输出:此列表中的节点4(序列化形式:[4, 5, 6])

https://leetcode-cn.com/problems/middle-of-the-linked-list
*/

use crate::link::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast_p = &head; // 快指针
    let mut slow_p = &head; // 慢指针


    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some() {
        slow_p = &slow_p.as_ref().unwrap().next;
        fast_p = &fast_p.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow_p.clone()
}