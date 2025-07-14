struct Solution;

impl Solution {

    pub fn longest_palindrome1(s: String) -> String {
        let s_char: Vec<char> = s.chars().collect();
        let mut answer: String = String::new();
        let mut len_ans = 0;

        for i in 0..s.len() {
            let mut l = i as isize;
            let mut r = i as isize;

            while l >= 0 && (r as usize) < s.len() && s_char[l as usize] == s_char[r as usize] {
                if (r - l + 1) > len_ans {
                    answer = s_char[(l as usize)..((r+1) as usize)].iter().collect();
                    len_ans = r - l + 1;
                }
                l -= 1;
                r += 1;
            }

            let mut l = i as isize;
            let mut r = (i + 1) as isize;

            while l >= 0 && (r as usize) < s.len() && s_char[l as usize] == s_char[r as usize] {
                if (r - l + 1) > len_ans {
                    answer = s_char[(l as usize)..((r+1) as usize)].iter().collect();
                    len_ans = r - l + 1;
                }
                l -= 1;
                r += 1;
            }
        }
        answer
    }

    pub fn longest_palindrome(s: String) -> String {
        let s_char: Vec<char> = s.chars().collect();
        let mut answer: String = String::new();
        let mut len_ans = 0;

        for i in 0..s.len() {
            let mut l = i;
            let mut r = i;

            while r < s.len() && s_char[l] == s_char[r] {
                if (r - l + 1) > len_ans {
                    answer = s_char[l..(r+1)].iter().collect();
                    len_ans = r - l + 1;
                }
                if l > 0 {
                    l -= 1;
                } else {
                    break;
                }
                r += 1;
            }

            let mut l = i;
            let mut r = i + 1;

            while r < s.len() && s_char[l] == s_char[r] {
                if (r - l + 1) > len_ans {
                    answer = s_char[l..(r+1)].iter().collect();
                    len_ans = r - l + 1;
                }
                if l > 0 {
                    l -= 1;
                } else {
                    break;
                }
                r += 1;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_1() {
        assert_eq!(Solution::longest_palindrome1("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome1("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome1("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome1("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome1("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome1("".to_owned()), "");
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
    }
}