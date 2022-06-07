/*
https://leetcode-cn.com/problems/plus-one
给定一个由整数组成的非空数组所表示的非负整数，在该数的基础上加一。最高位数字放在数组的首位，数组中每个元素只存单个数字。
假设除了整数0之外，这个整数不会以零开头。
示例：
    输入:[1,2,3]
    输出:[1,2,4]
    输入:[9,9,9]
    输出:[1,0,0,0]
*/

pub fn plus_one(mut nums:  Vec<i32>) -> Vec<i32>{
    // 反向下标
    let mut i = nums.len() - 1;
    loop {
        // 末尾非数字9，加一返回
        if nums[i] < 9 {
            nums[i] += 1;
            return nums;
        }
        // 是数字9，将其设为0
        nums[i] = 0;
        if i > 0 {
            i -= 1;
        } else if i == 0{
            break;
        }
    }
    // 处理全部是9的情况, 长度加一
    nums = vec![0; nums.len() + 1];
    nums[0] = 1;
    nums

}