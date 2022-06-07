/*
给出 (代表生成括号的对数)，请写出一个函数，使其能够生成所有可能的并且有效的括号组合。
1)示例:
给出n=3，生成结果为:
[
"((()))",
"(()())",
"(())()",
"()(())",
"()()()"
]
https://leetcode-cn.com/problems/generate-parentheses
思路：
    通过左括号和右括号的个数判断是否可以组成有效的括号组合。
提前判断括号组合是否有效，并在递归生成中间结果时，剪掉无效括号组合的这些分支，以便提高效率。
    定义一个动态数组，用于存放由有效括号组合而成的字符串。一个括号能否加入字符串的判断条件是:
    1)只要左括号的个数小于 ，左括号内就可以加入字符串。
    2)只有左括号个数大于右括号的个数，右括号内才可以加入字符串。
当左括号和右括号的个数都为n时，将字符串添加到动态数组中。
*/

use std::fmt::format;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    // 初始化返回动态数组
    let mut res = vec![];
    recursion(&mut res, 0, 0, n, String::from(""));
    res
}

pub fn recursion(vec: &mut Vec<String>, left: i32, right: i32, n: i32, s: String) {
    // 对应模版：递归终止条件
    // 左括号跟右括号都为n时添加这个结果
    if left == n && right == n {
        vec.push(s.clone());
    }

    // 对应模版：下探到下一层
    // 左括号个数小于n， 可以继续添加左括号
    if left < n {
        recursion(vec, left + 1, right, n, format!("{}{}", &s, "("));
    }

    // 左括号个数大于右括号个数， 可以继续加右括号
    if right < left {
        recursion(vec, left, right + 1, n, format!("{}{}", &s, ")"));
    }
}



