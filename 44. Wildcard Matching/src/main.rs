fn main() {
    assert_eq!(Solution::is_match(String::from("bbbababbabbbbabbbbaabaaabbbbabbbababbbbababaabbbab"), String::from("a******b*")), false);
}

struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![-1; p.len() + 1]; s.len() + 1];
        dp[0][0] = 1;
        let mut s: Vec<char> = s.chars().collect();
        s.insert(0, ' ');
        let mut p: Vec<char> = p.chars().collect();
        p.insert(0, ' ');
        for j in 1..p.len() {
            if p[j] != '*' { break }
            else { dp[0][j] = 1 }
        }
        for j in 1..p.len() {
            dp[0][j] = if dp[0][j] <= 0 { 0 } else { 1 }
        }
        for i in 1..s.len() {
            dp[i][0] = 0;
        }
        let mut x = Dp::new(&s, &p, &mut dp);
        
        let ret = if x.cal(s.len()-1, p.len()-1) > 0 { true } else { false };
        println!("x.dp = {:#?}", x.dp);
        ret
    }
}

struct Dp<'a> {
    s: &'a [char],
    p: &'a [char],
    dp: &'a mut Vec<Vec<i32>>
}

impl<'a> Dp<'a> {
    pub fn cal(&mut self, i:usize, j:usize) -> i32 {
        if self.dp[i][j] >= 0 { return self.dp[i][j] }
        self.dp[i][j] = match (self.s[i], self.p[j]) {
            (_, '*') => self.cal(i, j-1) | self.cal(i-1, j) | self.cal(i-1, j-1),
            (_, '?') => self.cal(i-1, j-1),
            (a, b) => {
                if a == b { self.cal(i-1, j-1) }
                else { 0 }
            }
        };
        self.dp[i][j]
    }

    pub fn new(s: &'a [char], p: &'a [char], dp: &'a mut Vec<Vec<i32>>) -> Dp<'a> {
        Dp { s,p,dp }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn basic_t() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("a")), false);
        assert_eq!(Solution::is_match(String::from("cb"), String::from("?a")), false);
        assert_eq!(Solution::is_match(String::from("acdcb"), String::from("a*c?b")), false);
    }

    #[test]
    fn basic_f() {
        assert_eq!(Solution::is_match(String::from("aa"), String::from("*")), true);
        assert_eq!(Solution::is_match(String::from("adceb"), String::from("*a*b")), true);
    }

    #[test]
    fn basic() {
        assert_eq!(Solution::is_match(String::from("abccssbsbsbsdbb"), String::from("a*b?cs*sdb?")), true);
        assert_eq!(Solution::is_match(String::from("adbd"), String::from("*a******bd")), true);
    }

    #[test]
    fn edge() {
        assert_eq!(Solution::is_match(String::from("abccssbsbsbsdbb"), String::from("*")), true);
        assert_eq!(Solution::is_match(String::from(""), String::from("****")), true);
        assert_eq!(Solution::is_match(String::from(""), String::from("a****")), false);
        assert_eq!(Solution::is_match(String::from(""), String::from("")), true);
        assert_eq!(Solution::is_match(String::from("sdf"), String::from("")), false);
        assert_eq!(Solution::is_match(String::from("bbbababbabbbbabbbbaabaaabbbbabbbababbbbababaabbbab"), String::from("a******b*")), false);


    }
}