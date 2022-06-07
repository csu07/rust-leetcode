/*
https://leetcode-cn.com/problems/two-sum
给定一个整数数组nums和一个目标值target，请在该数组中找出 和为目标值的两个整数，并返回它们的数组索引。
(1)说明
  假设每种输入只对应一个答案，不能重复利用这个数组中相同的元素。
 给定nums = [2, 7, 11, 15]，target = 9
nums[0] + nums[1] = 2 + 7 = 9，所以返回[0, 1]
 */

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // 初始化一个map，保存数组中的值-该值的索引
    let mut map:HashMap<i32, usize> = HashMap::new();

    // 遍历数组 找到给定目标与当前元素差值在该map中的索引，返回当前索引和差值对应的索引
    // 否则将该元素跟索引插入map
    for i in 0..nums.len() {
        let temp = target - nums[i];
        if map.contains_key(&temp) {
            return vec![i as i32, map[&temp] as i32];
        }
        map.insert(nums[i], i);
    }
    vec![]
}