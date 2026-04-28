use std::rc::Rc;
use Regex::*;

enum Regex {
    // 空集合
    Emp,
    // ε
    Eps,
    // 一文字
    Char(char),
    // 連接
    Concat(Rc<Regex>, Rc<Regex>),
    // OR
    Or(Rc<Regex>, Rc<Regex>),
    // クリーネ閉包
    Star(Rc<Regex>),
}

impl Regex {
    fn is_match(mut self, text: &str) -> bool {
        // 微分
        text.chars().for_each(|c| {
            self = self.derive(c);
        });
        // 残ったものにεは含まれるのか?
        self.contains_eps()
    }

    fn contains_eps(&self) -> bool {
        match self {
            Emp => false,
            Eps => true,
            Char(_) => false,
            Concat(r1, r2) => r1.contains_eps() && r2.contains_eps(),
            Or(r1, r2) => r1.contains_eps() || r2.contains_eps(),
            Star(_) => true,
        }
    }

    fn derive(&self, target: char) -> Regex {
        match self {
            Emp => Emp,
            Eps => Emp,
            Char(c) => {
                if c == &target {
                    Eps
                } else {
                    Emp
                }
            }
            Concat(r1, r2) => {
                let left = Concat(r1.derive(target).into(), Rc::clone(r2));
                if r1.contains_eps() {
                    Or(left.into(), r2.derive(target).into())
                } else {
                    left
                }
            }
            Or(r1, r2) => Or(r1.derive(target).into(), r2.derive(target).into()),
            Star(r) => Concat(r.derive(target).into(), Star(Rc::clone(r)).into()),
        }
    }
}
fn main() {
    // a*
    let a_star = Star(Char('a').into());
    // a*b
    let a_star_b = Concat(a_star.into(), Char('b').into());
    let result = a_star_b.is_match("aabb");
    println!("a*b: {result}");
}
