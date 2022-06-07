/*
https://leetcode-cn.com/problems/move-zeroes
给定一个数组nums， 编写一个函数将数组中所有的0移动到数组末尾，同时保持非零元素的顺序的相对顺序。
要求：
    1. 必须在原数组上操作，不能复制额外的数组
    2. 尽量减少操作次数
示例：
    输入:[0, 1, 0, 3, 12]
    输出:[1, 3, 12, 0, 0]

思路：
假设数组中非零的数有k个， 遍历数组，将数组中非零的元素移动到0~k-1的位置，然后将k 到末尾的元素都替换成零

*/

pub  fn move_zero(nums: &mut Vec<i32>) {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums[k] = nums[i];
            k += 1;
        }
    }
    for i in k..nums.len() {
        nums[i] = 0;
    }
}