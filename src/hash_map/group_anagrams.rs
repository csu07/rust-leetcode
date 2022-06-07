/*
给定一个字符串数组，将字母异位词组合在一起。
(1)说明
1)所有输入均为小写字母。 2)不考虑答案输出的顺序。
(2)示例
输入:["eat", "tea", "tan", "ate", "nat", "bat"],
输出:
[
    ["ate","eat","tea"],
    ["nat","tan"],
    ["bat"]
]
(3)链接 https://leetcode-cn.com/problems/group-anagrams
*/

use std::collections::HashMap;

pub fn group_anagrams(array: Vec<String>) -> Vec<Vec<String>> {

    // 初始化返回的数组
    let mut result :Vec<Vec<String>>= Vec::new();
    let mut map :HashMap<String, Vec<String>> = HashMap::new();
    let length = array.len();

    for i in 0..length {
        // 对字符串进行排序
        let mut chars = vec![];
        for c in array[i].chars(){
            chars.push(c);
        }
        chars.sort();
        let key :String = chars.into_iter().collect();
        let value = map.get(&key);

        if value != None {
            let mut v = value.unwrap().to_vec();
            v.push(array[i].clone());
            map.insert(key, v);
        }else {
            let v = vec![array[i].clone()];
            map.insert(key, v);
        }
    }
    for val in map.values() {
        result.push(val.to_vec());
    }

    result
}