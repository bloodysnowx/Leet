impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => vec.push(c),
                ')' => if vec.pop() != Some('(') { return false; },
                ']' => if vec.pop() != Some('[') { return false; },
                '}' => if vec.pop() != Some('{') { return false; },
                _ => (),
            }
        }
        return vec.is_empty();
    }
}