/*
给定一组不含重复元素的整数数组nums，返回该数组所有可能的子集(幂集)。
(1)说明:
解集不能包含重复的子集。
(2)示例:
输入:nums=[1, 2, 3]
输出:
[
[3],
    [1],
    [2],
    [1, 2, 3],
    [1, 3],
    [2, 3],
    [1, 2],
    []
]

此题可用回溯算法求解

https://leetcode-cn.com/problems/subsets
*/

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {

    if nums.len() == 0 {
        return vec![];
    }
    let mut vecs: Vec<Vec<i32>> = Vec::new();
    let mut vec: Vec<i32> = Vec::new();
    backtrace(&mut vecs, &mut vec, &nums,0);
    return vecs;
}

fn backtrace(vecs:&mut Vec<Vec<i32>>, vec:&mut Vec<i32>, nums:&Vec<i32>, start:usize) {
    // 将路径写入结果集
    vecs.push(vec.clone());

    for i in start..nums.len() {
        // 做选择
        vec.push(nums[i]);
        // 对应模板:将该选择从选择列表移除后递归调用
        backtrace(vecs, vec, &nums, i + 1);

        // 对应模板:撤销选择，将该选择重新加入选择列表
        vec.remove(vec.len() - 1);

    }
}