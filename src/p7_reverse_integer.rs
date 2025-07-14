struct Solution;

impl Solution {
    pub fn reverse1(x: i32) -> i32 {
        let mut x = x;
        let mut minus = false;
        let mut answer: i64 = 0;        
        if x < 0 {
            x = -1 * x;
            minus = true;
        }
        while x > 0 {
            let last_digit: i64 = (x % 10).try_into().unwrap();
            x = x / 10;
            answer = answer * 10 + last_digit;
            let check = if minus {-1 * answer } else { answer };
            match i32::try_from(check) {
                Ok(_) => (),
                Err(_) => return 0 as i32,
            }
        }
        answer = if minus {-1 * answer } else { answer };
        answer.try_into().unwrap()
    }

    pub fn reverse(x: i32) -> i32 {
        let xx = if x.is_negative() { -1 * x } else { x };
        let mut string_x: String = format!("{}", xx).chars().rev().collect();
        if x.is_negative() {
            string_x.insert(0, '-');
        }

        match string_x.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => return 0 as i32,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_7_1(){
        assert_eq!(Solution::reverse1(123), 321);
        assert_eq!(Solution::reverse1(-123), -321);
        assert_eq!(Solution::reverse1(0), 0);
        assert_eq!(Solution::reverse1(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse1((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse1((-(base.pow(31)-1)) as i32), 0);
    }

    #[test]
    fn test_7(){
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-123000), -321);
        let base: i64 = 2;
        assert_eq!(Solution::reverse((base.pow(31) - 1) as i32), 0);
        assert_eq!(Solution::reverse((-(base.pow(31)-1)) as i32), 0);
    }
}