use Regex::*;
use anyhow::Result;
use std::{iter::Peekable, rc::Rc};
use thiserror::Error;
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
    fn is_match(self, text: &str) -> bool {
        text.chars()
            // 微分
            .fold(self, |result, c| result.derive(c))
            //残ったものにεは含まれるのか？
            .contains_eps()
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

#[derive(Error, Debug)]
enum ParseError {
    #[error("the regex `{0}` is not available")]
    UnimplmentedRegex(String),
    #[error("bracket is not closed")]
    InvalidBracket,
    #[error("invalid syntax")]
    InvalidSyntax,
}

// 正規表現のEBNF
// <expr> ::= <term> ['|' <term>]*
// <term> ::= <factor> [<factor>]*
// <factor> ::= <atom> ['?'|'*'|'+']?
// <atom> :: = '' | <char> | '(' <expr> ')'
// <char> ::= メタ文字以外の全ての文字 | '.'
struct Parser;

impl Parser {
    fn parse_regex(&self, source: impl Into<String>) -> Result<Regex, ParseError> {
        self.parse_expr(&mut source.into().chars().into_iter().peekable())
    }
    fn parse_expr<I>(&self, iter: &mut Peekable<I>) -> Result<Regex, ParseError>
    where
        I: Iterator<Item = char>,
    {
        let mut val = self.parse_term(iter)?;
        while let Some(&next) = iter.peek() {
            match next {
                '|' => {
                    iter.next();
                    let val2 = self.parse_term(iter)?;
                    val = val.or(val2);
                }
                _ => break,
            }
        }
        Ok(val)
    }
    fn parse_term<I>(&self, iter: &mut Peekable<I>) -> Result<Regex, ParseError>
    where
        I: Iterator<Item = char>,
    {
        let mut val = self.parse_factor(iter)?;
        while let Some(&next) = iter.peek()
            && next != '|'
            && next != ')'
        {
            let val2 = self.parse_factor(iter)?;
            val = val.concat(val2);
        }
        Ok(val)
    }
    fn parse_factor<I>(&self, iter: &mut Peekable<I>) -> Result<Regex, ParseError>
    where
        I: Iterator<Item = char>,
    {
        let mut val = self.parse_atom(iter)?;
        if let Some(&next) = iter.peek() {
            match next {
                '?' => {
                    return Err(ParseError::UnimplmentedRegex("?".into()));
                }
                '*' => {
                    iter.next();
                    val = val.star();
                }
                '+' => {
                    return Err(ParseError::UnimplmentedRegex("+".into()));
                }
                _ => (),
            }
        }
        Ok(val)
    }
    fn parse_atom<I>(&self, iter: &mut Peekable<I>) -> Result<Regex, ParseError>
    where
        I: Iterator<Item = char>,
    {
        if let Some(&next) = iter.peek() {
            match next {
                '(' => {
                    iter.next();
                    let inner = self.parse_expr(iter)?;
                    return match iter.next() {
                        Some(')') => Ok(inner),
                        _ => Err(ParseError::InvalidBracket),
                    };
                }
                _ => return self.parse_char(iter),
            }
        }
        Ok(Eps)
    }
    fn parse_char<I>(&self, iter: &mut Peekable<I>) -> Result<Regex, ParseError>
    where
        I: Iterator<Item = char>,
    {
        match iter.next() {
            Some(c) if !Self::is_meta(c) => Ok(Char(c)),
            Some('.') => Ok(Dot),
            _ => Err(ParseError::InvalidSyntax),
        }
    }
    fn is_meta(c: char) -> bool {
        c == '?' || c == '*' || c == '+' || c == '(' || c == ')' || c == '|' || c == '.'
    }
}
fn main() -> Result<()> {
    let parser = Parser;
    let source = "(abc)";
    let regex = parser.parse_regex(source)?;
    println!("{:?}", regex);
    let result = regex.clone().is_match("abc");
    println!("{source}: {result}");
    Ok(())
}
