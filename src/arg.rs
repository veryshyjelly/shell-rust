use std::{ops::Index, slice::SliceIndex};

pub struct Argument {
    args: Vec<String>,
}

impl Argument {
    pub fn try_from(text: &str) -> Option<Self> {
        let mut args = vec![];
        let mut quoted = None;
        let mut escaping = false;
        let mut buffer = String::new();

        for c in text.chars() {
            match quoted {
                Some('\'') => {
                    if c == '\'' {
                        quoted = None;
                    } else {
                        buffer.push(c);
                    }
                }
                Some('\"') => {
                    if escaping {
                        match c {
                            '\\' | '$' | '\"' | '\r' | '\n' => buffer.push(c),
                            _ => {
                                buffer.push('\\');
                                buffer.push(c);
                            }
                        }
                        escaping = false;
                        continue;
                    }
                    match c {
                        '\\' => escaping = true,
                        '\"' => {
                            quoted = None;
                        }
                        _ => buffer.push(c),
                    }
                }
                Some(_) => panic!("Quoted shouldn't be char other than ' or \""),
                None => {
                    if escaping {
                        buffer.push(c);
                        escaping = false;
                        continue;
                    }
                    // when not escaping
                    match c {
                        '\'' => quoted = Some('\''),
                        '\"' => quoted = Some('\"'),
                        '\\' => {
                            escaping = true;
                        }
                        ' ' => {
                            if buffer.is_empty() {
                                continue;
                            };
                            args.push(buffer);
                            buffer = String::new();
                        }
                        _ => buffer.push(c),
                    }
                }
            }
        }
        if !buffer.is_empty() {
            args.push(buffer);
        }
        Some(Argument { args })
    }
}

impl<I: SliceIndex<[String]>> Index<I> for Argument {
    type Output = I::Output;
    fn index(&self, index: I) -> &Self::Output {
        &self.args[index]
    }
}
