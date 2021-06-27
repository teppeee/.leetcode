/*
 * @lc app=leetcode id=18 lang=rust
 *
 * [18] 4Sum
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return vec![];
        }
        let mut nums = nums;
        nums.sort();
        
        let n = nums.len();
        let mut res = vec![];
        
        for i in 0..(n - 3) {
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
        
            for j in (i + 1)..(n - 2) {
                if j > i + 1 && nums[j] == nums[j-1] {
                    continue;
                }
                
                let mut low = j + 1;
                let mut high = n - 1;
                while low < high {
                    if low > j + 1 && nums[low-1] == nums[low] {
                        low += 1;
                        continue
                    }
                    if nums[low] + nums[high] + nums[i] + nums[j] > target {
                        high -= 1;
                    } else if nums[low] + nums[high] + nums[i] + nums[j] < target {
                        low += 1;
                    } else {
                        res.push(vec![nums[i], nums[j], nums[low], nums[high]]);
                        low += 1;
                        high -= 1;
                    }
                }
            }
        }
        res
    }
}
// @lc code=end

