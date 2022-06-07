/*
假设需要爬 个台阶到达楼顶，每次可以爬1或2个台阶，问有多少 种不同的方法可以爬到楼顶。
给定 是一个正整数。
(2)示例
输入:2
输出:2
输入:4
输出:5
(3)链接 https://leetcode-cn.com/problems/climbing-stairs
每一步可以爬1或2个台阶，要上到第n 阶有以下两种方法:
1)在第 n-1阶向上爬1个台阶;
2)在第 n-2阶向上爬2个台阶。
因此，到达第 n阶的方法总数就是到第 n-1阶和第 n-2阶的方法数之和。
由于直接进行递归会出现大量的冗余计算，可以在递归过程中把每一步结果存储在数组中，
当再次计算第n阶的方法数时直接从数组中返回结果。
*/


pub fn climb_stairs(n: i32) -> i32 {
    let mut res = vec![0; n as usize];
    recursion(n as usize, &mut res)
}

fn recursion(n: usize, res: &mut Vec<i32>) -> i32 {
    if n <= 2 {
        return n as i32;
    }
    // 对应模板:下探到下一层
    // 到达第n-1阶的方法数
    if res[n - 1] == 0 {
        res[n - 1] = recursion(n - 1, res);
    }
    // 对应模板:下探到下一层
    // 到达第n-2阶的方法数
    if res[n - 2] == 0 {
        res[n - 2] = recursion(n - 2, res);
    }
    // 到达第n阶的方法数是到达第n-1阶和第n-2阶的方法数之和
    return res[n - 1] + res[n - 2];
}