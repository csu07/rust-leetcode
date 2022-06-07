use std::str::Chars;

mod pow;
mod climb;
mod generate_parentheses;


/*
递归是一种应用非常广泛的算法，也是一种非常实用的编程技巧。递归算法能解决的问题，要求同时满足以下3个条件。
1)一个问题的解可以分解为几个子问题的解。
2)这个问题与分解之后的子问题除了数据规模不同外，求解思路 完全一样。
3)存在递归终止条件。
*/

fn recursion(level:i32, param:i32) {

    // 递归终止条件
    if (level > MAX_LEVEL) {
        process_result();
        return;
    }

    // 处理当前层逻辑
    process(level, param);
    // 下探到下一层
    recursion(level+1, newparam);
    // 清理当前层状态
    clear_state();
}


/*

分治算法是一种处理问题的思想，其核心思想是分而治之，也就是将原问题划分成个规模较小的、结构与原问题相似的子问题，
递归地解决这些子问题，最后合并子问题的结果、得到原问题的解。分治算法能解决的问题一般需要满足以下几个条件。
1)原问题与分解后的子问题具有相同的模式;
2)原问题分解后的子问题可以独立求解，子问题之间没有相关性;
3)具有分解终止条件，当问题足够小时，可以直接求解;
4)可以将子问题合并成原问题，且这个合并操作的复杂度不能太高，否则不能起到降低算法总体复杂度的效果。
分治算法通常采用递归来实现。在递归过程中，每一层的递归都会涉及3个操作。
1)分解:将原问题分解成一系列子问题;
2)求解:递归地求解各个子问题，若子问题足够小则直接求解;
3)合并:将子问题的结果合并成原问题的解。
分治算法的代码模板如下:
 */

fn divide_conquer(problem:i32, param1:i32, param2:i32) {
    // 递归终止条件
    if condition {
        return result;
    }
    // 处理当前层逻辑，将问题拆分成子问题
    data = prepare_data(problem);
    subproblems = split_problem(problem, data);

    // 下探到下一层，求解子问题
    subresult1 = divide_conquer(subproblems[0], p1, d1);
    subresult2 = divide_conquer(subproblems[1], p1, d2);
    subresult3 = divide_conquer(subproblems[2], p1, d3);

    // 将子问题的结果合并成原问题的解
    result = process_result(subresult1, subresult2, subresult3);

    // 清理当前层状态
    clear_state();

}

/*
回溯算法是采用试错的思想，在分步解决问题的过程中，当发现现有的分步不能得到有效的正确解时，取消上一步甚至上几步的计算，
通过其他可能的分步再次尝试寻找问题的答案。回溯算法的处理思想有些类似枚举搜索，通过枚举所有的解，找到满足期望的解。
为了有规律地枚举所有可能的解，避免遗漏和重复，我们可将问题求解的过程分为多个阶段。
每个阶段都会面对一个岔路口，先随意选择一条路走，当发现此路不通时，即得不到符合期望的解的时候，
就回退到上一个岔路口，另选一条路继续走。
通常，回溯算法也采用递归来实现，在递归过程中，利用剪枝操 作避免穷举所有可能的情况，提高回溯的效率。
回溯算法的代码模板如下所示。其中，路径是指已经做出的选择，选择列表是指当前可做的选择，结束条件是指无法再做选择的条件。
回溯算法的核心是维护 走过的路径和当前可以做的选择列表，在递归调用之前做选择，在递归调用之后撤销选择。
当触发结束条件时，将路径记入结果集。上述解释可能比较抽象，读者可以对照代码模板来练习子集、组合和皇后三个题目
*/
fn t(){
    let mut solution: Vec<String> = vec![];
    fn backtrack(root:Chars, choices:Vec<String>) {
        // root 路径；choices:选择列表
        // 递归终止条件
        if condition {
            solution.push(root);
            return;
        }
        for 选择 in choices {
            // 做选择
            root.push(选择);
            // 将该选择从选择列表移除后递归调用
            backtrack(路径, 选择列表);
            // 撤销选择，将该选择重新加入选择列表
            root.remove("选择");
        }
    }
}
