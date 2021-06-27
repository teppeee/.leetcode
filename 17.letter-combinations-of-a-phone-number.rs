/*
 * @lc app=leetcode id=17 lang=rust
 *
 * [17] Letter Combinations of a Phone Number
 */

// @lc code=start
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        
    let mut res: Vec<String> = vec![];
        
    if digits.len() == 0 {
        return res;
    }

    let m: std::collections::HashMap<char, Vec<&str>> = [
        ('1', vec![]),
        ('2', vec!["a", "b", "c"]),
        ('3', vec!["d", "e", "f"]),
        ('4', vec!["g", "h", "i"]),
        ('5', vec!["j", "k", "l"]),
        ('6', vec!["m" , "n", "o"]),
        ('7', vec!["p", "q", "r", "s"]),
        ('8', vec!["t", "u", "v"]),
        ('9', vec!["w", "x", "y", "z"]),
    ]
    .iter()
    .cloned()
    .collect();

    for c in digits.chars() {
        let letters = m.get(&c).unwrap();
        let mut tmp: Vec<String> = vec![];

        for cc in letters {
            if res.len() == 0 {
                tmp.push(cc.to_string());
            } else {
                for r in res.iter() {
                    tmp.push(r.to_owned() + &cc.to_string());
                }
            }
        }

        res = tmp;
    }

    res
    }
}
// @lc code=end

