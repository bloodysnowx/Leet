impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().fold((0, ' '), |(sum, prev), x| {
            let val = match x {
                'M' => if prev == 'C' { 800 } else { 1000 },
                'D' => if prev == 'C' { 300 } else { 500 },
                'C' => if prev == 'X' { 80 } else { 100 },
                'L' => if prev == 'X' { 30 } else { 50 },
                'X' => if prev == 'I' { 8 } else { 10 },
                'V' => if prev == 'I' { 3 } else { 5 },
                'I' => 1,
                _ => 0
            };
            (sum + val, x)
        }).0
    }
}