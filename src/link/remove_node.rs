/*
给定一个链表，删除链表的倒数第 个节点，并且返回链表的头节 点。
(1)说明
给定的 保证是有效的。
(2)示例
给定一个链表:1→2→3→4→5和 =2
当删除了倒数第二个节点后，链表变为1→2→3→5
(3)链接
    https://leetcode-cn.com/problems/remove-nth-node-from-end-of-list
*/

use crate::link::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));

    let mut slow_p = &mut dummy;
    let mut fast_p = &mut slow_p.clone();
    // fast_p向后移动n+1个节点，以使得fast_p与slow_p之间间隔n个节点
    for _ in 1..=n + 1 {
        fast_p = &mut fast_p.as_mut().unwrap().next;
    }

    while fast_p.is_some() {
        fast_p = &mut fast_p.as_mut().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
    }
    let next = &slow_p.as_mut().unwrap().next.as_mut().unwrap().next;
    slow_p.as_mut().unwrap().next = next.clone();

    dummy.unwrap().next
}