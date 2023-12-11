use std::collections::HashMap;

//leetcode 第一题，两数之和
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::with_capacity(nums.len());
    for (index, n) in nums.iter().enumerate() {
        let temp = target - *n;
        if map.contains_key(&temp) {
            return vec![index as i32, *map.get(&temp).unwrap()];
        } else {
            map.insert(*n, index as i32);
        }
    }
    vec![0]
}

fn main() {
    let arr = two_sum(vec![1, 2, 3, 4, 5], 9);
    println!("{:?}", arr);
}
