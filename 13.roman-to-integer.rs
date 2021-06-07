/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sum = 0;
        let mut last = 0;
        for c in s.chars() {
            if last >= get_value(c) || last == 0 {
                sum = sum + last;
                last = get_value(c);
            } else {
                sum = sum + (get_value(c) - last );
                last = 0;
            }
        }
    return sum + last;
    }
}
    pub fn get_value(ch: char) -> i32 {
        match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
// @lc code=end

