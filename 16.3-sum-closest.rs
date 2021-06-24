/*
 * @lc app=leetcode id=16 lang=rust
 *
 * [16] 3Sum Closest
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let mut ret = nums[0] + nums[1] + nums[2];
        nums.sort();
        for i in 0..(nums.len()-2) {
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let total = nums[i] + nums[l] + nums[r];
                
                if total == target { return target; }
                if (total - target).abs() < (ret - target).abs() { ret = total; }
                if total < target { l += 1; }
                else { r -= 1; }
            }
        }
        
        ret
    }
}
// @lc code=end

