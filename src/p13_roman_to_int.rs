#![allow(dead_code)]
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut answer: i32 = 0;
        let mut last_val: i32 = 0;
        let map = Solution::letters();

        for ch in s.chars().rev() {
            if last_val > *map.get(&ch).unwrap() {
                answer -= map.get(&ch).unwrap();
            } else {
                answer += map.get(&ch).unwrap();
            }
            last_val = *map.get(&ch).unwrap();
        }
        answer
    }

    fn letters() -> HashMap<char, i32> {
        let mut map: HashMap<char, i32> = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);
        map
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_13(){
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}