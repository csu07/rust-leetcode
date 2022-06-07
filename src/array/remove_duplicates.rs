/*
https://leetcode-cn.com/problems/remove-duplicates-from-sorted-array

给定一个排序数组，需要在原地删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度。
说明:
    1.不要使用额外的数组空间，必须在原数组修改完成。
    2.不需要考虑数组中超出新长度后面的元素。
示例：
给定数组nums = [1, 1, 2]，函数应返回新的长度2，并且原数组 nums的前两个元素被修改为1、2。
给定nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]，函数应返回新的 长度5，并且原数组nums的前五个元素被修改为0、1、2、3、4。
*/


pub fn remove_dup(nums: &mut Vec<i32>) -> i32{
    if nums.len() <= 0{
        return 0;
    }
    let mut i = 0;
    for j in 1..nums.len() {
        // 如果nums[j] == nums[i], 此时是重复项，需要接着找直到不重复

        // if nums[j] == nums[i] {
        //    continue
        // }else {
        //     if j - i > 1 {
        //         nums[i + 1] = nums[j];
        //
        //     }
        //     i += 1;
        // }

        if nums[j] != nums[i] {
            if j -i > 1 {
                nums[i + 1] = nums[j];
            }
            i += 1;
        }

    }
    (i + 1) as i32  // 下标从零开始的
}