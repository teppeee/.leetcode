/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut vec = vec![];
        'outer: for i in 0..nums.len()-1{
            for j in (i+1)..nums.len() {
                if nums[i] + nums[j] == target {
                    vec.push(i as i32);
                    vec.push(j as i32);
                    break 'outer;
                }
            }
        }
        vec
    }
}
// @lc code=end

