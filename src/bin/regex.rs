use std::rc::Rc;
use Regex::*;

//Brzozowski derivativeによる正規表現の実装
#[derive(Clone, Debug)]
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
    // Dot
    Dot,
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
            Dot => false,
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
                let left = r1.derive(target).concat(Rc::clone(r2));
                if r1.contains_eps() {
                    left.or(r2.derive(target))
                } else {
                    left
                }
            }
            Or(r1, r2) => r1.derive(target).or(r2.derive(target)),
            Star(r) => r.derive(target).concat(self.clone()),
            Dot => Eps,
        }
    }

    fn concat(self, r2: impl Into<Rc<Regex>>) -> Self {
        Concat(self.into(), r2.into())
    }

    fn or(self, r2: impl Into<Rc<Regex>>) -> Self {
        Or(self.into(), r2.into())
    }

    fn star(self) -> Self {
        Star(self.into())
    }
}
fn main() {
    let a_dot_b = Char('a').concat(Dot).concat(Char('b'));
    let result = a_dot_b.clone().is_match("abb");
    println!("a.b: {result}");
}
