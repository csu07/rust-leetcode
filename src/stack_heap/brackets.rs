/*
https://leetcode-cn.com/problems/valid-parentheses
给定一个只包括“(”“)”“{”“}”“[”“]”的字符串，判 断字符串是否有效。
有效的字符串应该满足左括号必须用相同类型的右括号闭合，且左括号必须以正确的顺序闭合。
*/



pub fn is_valid(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();

    // 空字符串返回true
    if chars.len() == 0 {
        return true;
    }
    // 初始化一个栈
    let mut stack = vec![];
    for i in 0..chars.len() {
        if chars[i] == '(' {
            stack.push(')');
            println!("{}", chars[i])
        }else if chars[i] == '[' {
            stack.push(']');
        }else if chars[i] == '{' {
            stack.push('}')
        }else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
            println!("{}",chars[i]);
            return false;
        }
    }

    return stack.is_empty();
}