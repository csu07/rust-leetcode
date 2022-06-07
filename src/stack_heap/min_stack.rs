/*
https://leetcode-cn.com/problems/min-stack
设计一个支持push、pop、top操作，并且能在常数时间内检索到 最小元素的栈。
(1)说明
1)push:将数据压入栈中。
2)pop:删除栈顶元素。
3)top:获取栈顶元素。
4)getMin:检索栈中的最小元素。
push方法:判断push的数据是否小于等于min_stack栈顶元素值 (即最小值)，如果是则让min_stack也同步push该元素，即更 新最小值。
pop方法:判断pop的数据是否等于min_stack栈顶元素值(即最 小值)，如果是则让min_stack也同步pop栈顶元素，以保证 min_stack栈顶元素始终是stack中的最小值。
get_min方法:返回min_stack栈顶元素值。


输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

输出：
[null,null,null,null,-3,null,0,-2]

解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.

*/

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {

    fn new() -> Self {
        MinStack{
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min_stack.is_empty() || x <= *self.min_stack.last().unwrap() {
            self.min_stack.push(x);
        }
    }

    fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}