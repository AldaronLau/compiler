/// An operation.
#[derive(Debug)]
pub enum Op {
    /// Collect input for a function.
    In{},
    /// Collect output from a function.
    Out{},
    /// 
    Function{name:String},
    /// 
    Type{},
}

/// Compiled code.
pub struct Code {
    ops: Vec<Op>,
}

impl Code {
    /// Create new code.
    pub fn new() -> Code {
        Code {
            ops: vec![],
        }
    }

    fn push_def(&mut self, line: &[u8]) -> usize {
        let mut c = 0;
        let mut function_name = String::new();

        'a: loop {
            if c >= line.len() {
                break 'a;
            }

            match line[c] {
                b'\n' | b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b':' => {
                    self.ops.push(Op::Function { name: unsafe { std::str::from_utf8_unchecked(&line[..c]) }.to_string() });
                    return c + 1;
                }
                _ => {}
            }

            c += 1;
        }

        unreachable!();
    }

    /// Parse a line of code.
    pub fn parse(&mut self, line: &[u8]) {
        let mut start = 0;
        let mut c = 0;
        'a: loop {
            if c >= line.len() {
                break 'a;
            }

            match line[c] {
                b'\n' => {
                    println!("Found newline!");
                    start = c + 1;
                }
                // Found a keyword (keywords always end with a space)
                b' ' => {
//                    print!("Keyword");
                    match line[c + 1] {
                        b'+'|b'-'|b'*'|b'/'|b'%'|b'.'|b':'|b'!'|b'^'|b'&'|b'|'|b'('|b'"'|b'\''
                        => {
                            
                        }
                        _ => match &line[start..c] {
                            // Define function or type.
                            b"def" => {
                                println!("def");
                                start = c + 1;
                                start += self.push_def(&line[start..]);
                            }
                            // Declare immutable variable.
                            b"let" => {
                            }
                            // Declare mutable variable.
                            b"var" => {
                            }
                            // Set function output & return.
                            b"out" => {
                            }
                            a => {
                                let a = unsafe { std::str::from_utf8_unchecked(a) };
                                eprintln!("Unknown keyword `{}`", a);
//                                std::process::exit(1);
                            }
                        }
                    }
                }
                b'+'|b'-'|b'*'|b'/'|b'%'|b'.'|b':'|b'!'|b'^'|b'&'|b'|'|b'('|b'"'|b'\'' => {
                }
                b'\0' => {
                    println!("END");
                    return;
                }
                c => {
                    
                }
            }

            c += 1;
        }
    }

    /// Convert to `Op`s.
    pub fn to_ops(self) -> Vec<Op> {
        self.ops
    }
}
