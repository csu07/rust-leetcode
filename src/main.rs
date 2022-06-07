mod array;
mod stack_heap;
mod hash_map;
mod link;
mod tree;
mod recursion;

fn main() {
    let  mut nums = vec![0, 1, 0, 3, 12];

    array::move_zero(&mut nums);

    println!("{:?}", nums);
    // //2.
    // let res = array::plus_one(nums);
    // println!("{:?}", res);
    // //3.
    // let res = array::remove_dup(&mut nums);
    // print!("{}, {:?}", res, nums);

    // let s = String::from("(]");
    // let res = stack_heap::is_valid(s);
    // println!("{}", res);
    // let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    // let k = 1;
    //
    // let res = stack_heap::max_sliding_window(nums, k);
    // println!("{:?}", res);

}
