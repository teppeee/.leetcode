/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] ZigZag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;

        if num_rows < 2 {
            return s.into();
        }
    
        let mut strings = vec![String::new(); num_rows];
    
        let mut i = 0;
        let mut down = true;
    
        for c in s.chars() {
            strings[i].push(c);
            i = if down { i + 1 } else { i - 1 };
            down = down == (i > 0 && i < num_rows - 1);
        }
    
        strings.concat()
    }
}
// @lc code=end

