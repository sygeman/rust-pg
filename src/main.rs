struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![nums[0]];
        
        for (i, v) in nums.iter().enumerate().skip(1) {
            res.push(v + res[i - 1]);
        }
        
        return res;
    }
}

fn main() {
    let nums: Vec<i32> = vec![1,2,3,4];
    let s = Solution::running_sum(nums);
    
    println!("array is {:?}", s);
}
