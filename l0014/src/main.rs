impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].clone();
        for s in strs.iter().skip(1) {
            prefix = prefix.chars().zip(s.chars()).take_while(|(a, b)| a == b).map(|(a, _)| a).collect::<String>();
        }
        return prefix;
    }
}