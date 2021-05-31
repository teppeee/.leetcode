/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut i = 0;
        let mut j = len - 1;
        let mut max = 0;
        
        loop {
            if i >= j { break; }
            let h_i = height[i];
            let h_j = height[j];
            let temp = std::cmp::min(h_i, h_j) * (j - i) as i32;
            if max < temp {
                max = temp;
            }
            
            if h_i < h_j { i += 1; }
            else { j -= 1; }
        }
        
        max
    }
}
// @lc code=end

