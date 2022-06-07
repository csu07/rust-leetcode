/*
https://leetcode-cn.com/problems/sliding-window-maximum
给定一个数组nums，有一个大小为 k 的滑动窗口从数组的最左侧移 动到数组的最右侧。
在滑动过程中，只能看到在滑动窗口内的 k 个数字，滑动窗口每次只向右移动一位，要求返回滑动窗口中的最大值。
假设 k总是有效的，在输入数组不为空的情况下，1≤ k ≤输入数组的大小。

原理：
    使用双端队列实现一个单调递减队列，元素从队尾压入，从队尾或者队首弹出，直接取队首元素即可得到最大值。
算法流程
    1)给定的数组nums为空或者 k 为1，直接返回空数组。
    2)处理前 k 个元素，初始化单调递减队列。
    3)遍历数组，区间[k, nums.len())，在每一步执行3个操作。
        清理队列，弹出所有小于当前值的元素(它们不可能是最大值)，维持队列的单调递减。
        将当前元素从队尾压入队列。
        将最大值加入输出数组。
    4)单调递减队列的处理函数如下。
        push函数:当队尾元素小于当前值，弹出队尾元素，重复此步 骤，直到队列为空，然后再将当前值从队尾压入。
        pop函数:当队首元素等于传入元素，弹出队首元素。 max函数:返回队列中的最大值，即队首元素。
*/

use std::collections::VecDeque;

fn push(deque: &mut VecDeque<i32>, n: i32) {

    // 当前队列不为空且队尾元素小于当前值时，弹出队尾元素，直到队列为空
    // 1)loop:重复执行、永远不会结束的循环。
    // 2)while:在条件表达式的值为true时永远执行的循环。
    // 3)for:重复执行指定次数的循环。
    while  !deque.is_empty() && *deque.back().unwrap() < n{
        deque.pop_back();
    }
    // 队列为空时，从队尾添加该元素
    deque.push_back(n);
}

fn pop(deque: &mut VecDeque<i32>, n: i32) {
    // 当前队列不为空且队首元素等于传入元素，弹出队首元素
    if !deque.is_empty() && *deque.front().unwrap() == n {
        deque.pop_front();
    }

}

fn max(deque: &VecDeque<i32>) -> i32 {
    // 返回队列中的最大值，即队首元素
    *deque.front().unwrap()

}


pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // 数组为空或者k=1 返回数组
    if nums.is_empty() || k == 1 {
        return nums;
    }
    let mut res: Vec<i32> = vec![];
    let mut deque: VecDeque<i32> = VecDeque::new();

    for i in 0..nums.len() {
        // 弹出队列中所有小于当前值的元素，再将当前值从队尾压入
        push(&mut deque, nums[i]);

        if (i as i32) > k - 1 {
            // 弹出队首元素，让滑动窗口内保持k数字
            pop(&mut deque, nums[i - k as usize]);
            // 将最大值加入输出数组
            res.push(max(&deque));
        }else if (i as i32) == k-1 {
            // 将前k个元素的最大值加入输出数组
            res.push(max(&deque));
        }
    }
    res
}


























