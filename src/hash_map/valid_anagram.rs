/*
给定两个字符串s和t，编写一个函数来判断t是否是s的字母异位 词。异位词是指字母相同，但排列顺序不同的字符串。
(1)说明
假设字符串只包含小写字母。
(2)示例
输入:s = "anagram", t = "nagaram"
输出:true
输入:s = "rat", t = "car"
输出:false
(3)链接 https://leetcode-cn.com/problems/valid-anagram
*/
use std::collections::HashMap;
pub fn is_anagram (s: String, t: String) -> bool {
    // 判断长度是否一致
    if s.len() != t.len() {
        return false;
    }
    // 统计s中各个字符出现的次数
    let mut map = HashMap::new();
    for i in s.chars(){
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    for i in t.chars(){
        let count = map.entry(i).or_insert(0);
        *count -=1;
        if *count < 0 {
            return false;
        }
    }
    true
}