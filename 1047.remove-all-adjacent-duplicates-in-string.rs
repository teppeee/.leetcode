/*
 * @lc app=leetcode id=1047 lang=rust
 *
 * [1047] Remove All Adjacent Duplicates In String
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let chars:Vec<char> = s.chars().collect();
        let mut vec = vec![];
        for c in chars {
            if let Some(target) = vec.pop(){
                if target != c {
                    vec.push(target);
                    vec.push(c);
                }
            }else{
                vec.push(c);
            }
        }
    
        vec.iter().collect::<String>()
    }
}
// @lc code=end

