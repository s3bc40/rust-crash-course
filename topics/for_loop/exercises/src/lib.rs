pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for i in nums {
        sum += i;
    }
    sum
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    for _idx in 0..n {
        result.push(i);
    }
    result
}
