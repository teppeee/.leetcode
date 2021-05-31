/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut max = 0;
    
        while i < j {
            let h = height[i].min(height[j]);
            max = max.max(h * (i - j) as i32);
    
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
    
        max
    }
}
// @lc code=end

