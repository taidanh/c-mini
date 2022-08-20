extern crate lazy_static;

use regex::Regex;

pub type Token = (String, String);

pub struct Scanner {
    pub lineno: i32,
    off: usize,
    // tokens: Vec<(&'static str, Regex, &'static fn(Token) -> Box<Token>)>,
    // tokens: &'static [(&'static str, Regex, &'staticfn(Token) -> Box<Token>); 16],
    istring: String,
}

impl Scanner {
    pub fn new(istring: String) -> Scanner {
        Scanner {
            lineno: 1,
            off: 0,
            // tokens: Vec::from(&TOKENS),
            istring
        }
    }

    pub fn token(&mut self) -> Option<Token> {
        loop {
            if self.off >= (self.istring.len() - 1) {
                return None;
            }

            let mut matches = Vec::new();

            // loop through substrings
            let mut i = self.istring.len() - 1;
            // println!("i starting at {i}");
            while i > self.off {
                matches = Vec::new();

                for t in TOKENS.iter() {
                    let m = t.1.captures(&self.istring[self.off..i])
                            .map_or("", |m| {
                                m.get(0).map_or("", |x| x.as_str())
                            });
                    if m.len() > 0 {
                        matches.push((t.0, m, t.2));
                    }
                }

                if matches.len() > 0 {
                    break;
                }

                i -= 1;
            }

            if matches.len() == 0 {
                // TODO raise error
                println!("Error no matches found");
                std::process::exit(1);
            }
            matches.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
            // println!("matches: {:?}", matches);
            let longest = matches[0];
            let lexeme = longest.2((longest.0.to_string(), longest.1.to_string()));
            // println!("lex: {:?}", lexeme);

            self.off += lexeme.1.len();
            // println!("off: {:?}", self.off);

            // println!("remaining string: {:?}", &self.istring[self.off..]);

            if lexeme.0 != "IGNORE" {
                return Some(*lexeme);
            } else {
                if *lexeme.1 == *"\n" {
                    self.lineno += 1;
                }
            }
        }
    }
}

lazy_static! {
    static ref TOKENS: [(&'static str, Regex, &'static fn(Token) -> Box<Token>); 17] =
    [  ("MUL",    Regex::new(r"^\*$").unwrap(),                             &IDY)
    ,  ("PLUS",   Regex::new(r"^\+$").unwrap(),                             &IDY)
    ,  ("MINUS",  Regex::new(r"^-$").unwrap(),                              &IDY)
    ,  ("DIV",    Regex::new(r"^/$").unwrap(),                              &IDY)
    ,  ("EQ",     Regex::new(r"^==$").unwrap(),                             &IDY)
    ,  ("LT",     Regex::new(r"^<$").unwrap(),                              &IDY)
    ,  ("LBRACE", Regex::new(r"^\{$").unwrap(),                             &IDY)
    ,  ("RBRACE", Regex::new(r"^\}$").unwrap(),                             &IDY)
    ,  ("LPAR",   Regex::new(r"^\($").unwrap(),                             &IDY)
    ,  ("RPAR",   Regex::new(r"^\)$").unwrap(),                             &IDY)
    ,  ("SEMI",   Regex::new(r"^;$").unwrap(),                              &IDY)
    ,  ("ASSIGN", Regex::new(r"^=$").unwrap(),                              &IDY)
    ,  ("AMP",    Regex::new(r"^&$").unwrap(),                              &IDY)
    ,  ("COMMA",  Regex::new(r"^,$").unwrap(),                              &IDY)
    ,  ("NUM",    Regex::new(r"^([0-9]+(\.[0-9]+)?)|(\.[0-9]+)$").unwrap(), &IDY)
    ,  ("ID",     Regex::new(r"^[a-zA-Z]+[a-zA-Z0-9]*$").unwrap(),          &FIND_KEYWORDS)
    ,  ("IGNORE", Regex::new(r"^[ \n\t]$").unwrap(),                        &IDY)
    ];
}

static KEYWORDS: [(&str, &str); 6] = [
    ("IF", "if"),
    ("ELSE", "else"),
    ("FOR", "for"),
    ("INT", "int"),
    ("FLOAT", "float"),
    ("VOID", "void"),
];

fn idy(t: Token) -> Box<Token> {
    Box::new(t)
}

const IDY: fn(Token) -> Box<Token> = idy;

// TODO change token type to str, str
fn find_keywords(t: Token) -> Box<Token> {
    // println!("finding: {:?}", t);
    Box::new(
        KEYWORDS
            .iter()
            .find(|e| e.1 == t.1)
            .and_then(|e| {
                Some((e.0.to_string(), e.1.to_string()))
            })
            .unwrap_or(t)
    )
}

const FIND_KEYWORDS: fn(Token) -> Box<Token> = find_keywords;
