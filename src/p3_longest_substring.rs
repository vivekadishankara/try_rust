use std::collections::HashMap;

struct Solution;

impl Solution {

    pub fn length_of_longest_substring1(s: String) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = 0;
        let mut char_map: HashMap<char, i32> = HashMap::new();
        let mut max_len: i32 = 0;

        while r < (s.len() as i32) {
            if let Some(ch) = s.chars().nth(r as usize) {
                if let Some(index) = char_map.get_mut(&ch) {
                    if *index >= l {
                        l = *index + 1;
                    }
                    *index = r;
                    max_len = i32::max(max_len, r - l + 1);
                } else {
                    char_map.insert(ch, r);
                    max_len = i32::max(max_len, r - l + 1);
                }
            }
            r += 1
        }
        max_len
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l: i32 = 0;
        let mut char_map: HashMap<char, i32> = HashMap::new();
        let mut max_len: i32 = 0;

        for (r, ch) in s.chars().enumerate() {
            if let Some(prev_loc) = char_map.get(&ch) {
                if *prev_loc >= l {
                    l = *prev_loc + 1
                }
            }
            char_map.insert(ch, r as i32);
            max_len = i32::max(max_len, (r as i32) - l + 1);
        }
        max_len 
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_13_1(){
        assert_eq!(Solution::length_of_longest_substring1("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring1("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring1("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_13(){
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}