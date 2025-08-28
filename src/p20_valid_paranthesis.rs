use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let dic: HashMap<_,_> = HashMap::from([
            ('}', '{'),
            (')', '('),
            (']', '['),
        ]);

        let mut brackets: Vec<char> = Vec::new();

        for c in s.chars() {
            if dic.contains_key(&c) {
                if let Some(value) = brackets.last() {
                    if value == dic.get(&c).unwrap() {
                        brackets.pop();
                    } else {
                        return false;
                    }
                } else {
                    return false
                }
            } else {
                brackets.push(c);
            }
        }
        brackets.len() == 0
    } 
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_13(){
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("([])")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
        assert_eq!(Solution::is_valid(String::from("]")), false);
    }
}