use std::backtrace;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut x_copy = x;
        let mut reversed = 0;
        
        while x_copy > 0 {
            reversed = reversed * 10 + x_copy % 10;
            x_copy /= 10;
        }

        return reversed == x
    }

    fn reverse_number(x: i32) -> i32 {
        fn reverse_number_sub(x: i32, y: i32) -> i32 {
            if x == 0 {
                return y;
            } else {
                return reverse_number_sub(x / 10, y * 10 + x % 10);
            }
        }
        return reverse_number_sub(x, 0)
    }

    pub fn is_palindrome_2(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        return x == Solution::reverse_number(x);
    }
}