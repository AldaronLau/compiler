/// An operation.
#[derive(Debug)]
pub enum Op {
    /// Collect input for a function.
    InLet{name:String, def:String},
    /// Collect input for a function.
    InVar{name:String, def:String},
    /// Function call.
    Call{name:String, var_a:String, operator:String, var_b: String},
    /// Collect output from a function.
    Out{name:String, def:String, from:String},
    /// 
    Function{name:String},
    /// 
    Type{},
}

#[derive(PartialEq, Copy, Clone)]
enum Keyword {
    None,
    Def,
    Let,
    Var,
    Out,
}

/// Compiled code.
pub struct Code {
    ops: Vec<Op>,
    open: Keyword,
}

impl Code {
    /// Create new code.
    pub fn new() -> Code {
        Code {
            ops: vec![],
            open: Keyword::None,
        }
    }

    // Read through the rest of line making sure only spaces and comments.
    fn nothing_else_allowed(&mut self, mut c: usize, line: &[u8]) -> usize {
        let mut comment = false;

        'a: loop {
            match line[c] {
                b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b'\n' => {
                    return c + 1;
                }
                b';' => {
                    if comment == false {
                        return c + 1;
                    }
                }
                b'#' => {
                    comment = !comment;
                }
                b' ' => { /* Ignore spaces */ }
                a => {
                    if !comment {
                        eprintln!("If you want 2 commands on a line you need a semicolon `;`");
                        std::process::exit(1);
                    }
                }
            }

            c += 1;
        }
    }

    fn expression(&mut self, mut c: usize, line: &[u8], keyword: Keyword, name: String) -> usize {
        if line[c] == b'$' {
            c += 1;
            match keyword {
                Keyword::Let => {
                    let start = c;
                    'a: loop {
                        match line[c] {
                            b'\0' | b' ' | b'\n' | b';' => {
                                break;
                            }
                            a => {}
                        }
                        c += 1;
                    }
                    self.ops.push(Op::InLet { name, def: unsafe { std::str::from_utf8_unchecked(&line[start..c]) }.to_string() });
                    self.nothing_else_allowed(c, line)          
                }
                Keyword::Var => {
                    let start = c;
                    'a: loop {
                        match line[c] {
                            b'\0' | b' ' | b'\n' | b';' => {
                                break;
                            }
                            a => {}
                        }
                        c += 1;
                    }
                    self.ops.push(Op::InVar { name, def: unsafe { std::str::from_utf8_unchecked(&line[start..c]) }.to_string() });
                    self.nothing_else_allowed(c, line)
                }
                Keyword::None => {
                    eprintln!("TODO: scanf functionality");
                    unimplemented!();
                }
                _ => {
                    eprintln!("The `out` & `def` keywords can't use `$`");
                    std::process::exit(1);
                }
            }            
        } else {
            match keyword {
                Keyword::Out => {
                    let start = c;
                    // Read operand a.
                    let could_b = 'a: loop {
                        match line[c] {
                            b'\0' | b'\n' | b';' => {
                                break false;
                            }
                            b'.' => {
                                panic!("Function operators not supported yet!");
                            }
                            b' ' => {
                                c += 1;
                                break line[c] != b' ';
                            }
                            a => {}
                        }
                        c += 1;
                    };
                    if could_b {
                        let var_a = unsafe { std::str::from_utf8_unchecked(&line[start..c-1]) }.to_string();

                        let start = c;

                        // Read operand b.
                        let could_c = 'a: loop {
                            match line[c] {
                                b'\0' | b'\n' | b';' => {
                                    break false;
                                }
                                b'.' => {
                                    panic!("Function operators not supported yet!");
                                }
                                b' ' => {
                                    c += 1;
                                    break line[c] != b' ';
                                }
                                a => {}
                            }
                            c += 1;
                        };

                        if could_c {
                            eprintln!("Error: Too many operands!");
                            std::process::exit(1);
                        }

                        let var_b = unsafe { std::str::from_utf8_unchecked(&line[start..c-1]) }.to_string();

                        self.ops.push(Op::Call { name: "".to_string(), var_a, operator: "".to_string(), var_b });
                        self.ops.push(Op::Out { name, def: "".to_string(), from: "".to_string() });
                    } else {
                        self.ops.push(Op::Out { name, def: "".to_string(), from: unsafe { std::str::from_utf8_unchecked(&line[start..c-1]) }.to_string() });
                    }
                    self.nothing_else_allowed(c, line)          
                }
                _ => {
                    eprintln!("No `$` is not implemented for this keyword yet");
                    std::process::exit(1);
                }
            }
        }
    }

    // Read through the rest of line making sure only spaces and comments.
    fn assignment(&mut self, mut c: usize, line: &[u8], keyword: Keyword, name: String) -> usize {
        let mut comment = false;
        let mut found = false;

        'a: loop {
            match line[c] {
                b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b'\n' => {
                    return c + 1;
                }
                b';' => {
                    eprintln!("Semicolon `;` in the middle of assignment");
                    std::process::exit(1);
                }
                b'#' => {
                    comment = !comment;
                }
                b' ' => { /* Ignore spaces */ }
//                b':' => { found = true }
                a => {
/*                    if !comment && !found {
                        eprintln!("Extra token {}!", a as char);
                        std::process::exit(1);
                    }*/
                    if !comment {
                        return self.expression(c, line, keyword, name);
                    }
                }
            }

            c += 1;
        }
    }

    // A function or type.
    fn push_def(&mut self, line: &[u8]) -> usize {
        let mut c = 0;
        let mut function_name = String::new();

        'a: loop {
            match line[c] {
                b'\n' | b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b':' => {
                    self.ops.push(Op::Function { name: unsafe { std::str::from_utf8_unchecked(&line[..c]) }.to_string() });
                    return self.nothing_else_allowed(c + 1, line);
                }
                _ => {}
            }

            c += 1;
        }

        unreachable!();
    }

    // A function or type.
    fn push_let(&mut self, line: &[u8]) -> usize {
        let mut c = 0;
        let mut function_name = String::new();

        'a: loop {
            match line[c] {
                b'\n' | b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b':' => {
                    let name = unsafe { std::str::from_utf8_unchecked(&line[..c]) }.to_string();
                    return self.assignment(c + 1, line, self.open, name);
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
        let mut indentation = 0;
        let mut req = 0;

        'a: loop {
            if c >= line.len() {
                break 'a;
            }

            match line[c] {
                b'\n' => {
                    println!("Found newline!");
                    start = c + 1;
                    indentation = 0;
                }
                // Found a keyword (keywords always end with a space)
                b' ' => {
                    // Maybe it's just indentation that we can ignore.
                    if line[c - 1] == b'\n' || line[c - 1] == b' ' || line[c - 1] == b'\t' {
                        start = c + 1;
                        c += 1;
                        indentation += 1;
                        continue;
                    }

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
                                c = start - 1;
                                req = indentation + 4;
                                indentation = 0;
                            }
                            // Declare immutable variable.
                            b"let" => {
                                if indentation != req && self.open == Keyword::None  {
                                    eprintln!("Wrong indentation");
                                    std::process::exit(1);
                                }
                                println!("let");
                                start = c + 1;
                                self.open = Keyword::Let;
                                start += self.push_let(&line[start..]);
                                c = start - 1;
                                req = indentation + 4;
                                indentation = 0;
                            }
                            // Declare mutable variable.
                            b"var" => {
                                
                            }
                            // Set function output & return.
                            b"out" => {
                                if indentation != req && self.open == Keyword::None {
                                    eprintln!("Wrong indentation");
//                                    std::process::exit(1);
                                }
                                println!("out");
                                start = c + 1;
                                self.open = Keyword::Out;
                                start += self.push_let(&line[start..]);
                                c = start - 1;
                                req = indentation + 4;
                                indentation = 0;
                            }
                            a => {
                                let a = unsafe { std::str::from_utf8_unchecked(a) };

                                if indentation != req {
                                    println!("{} {}", indentation, req);

                                    eprintln!("Unknown keyword `{}`", a);
                                    std::process::exit(1);
                                }

                                if self.open == Keyword::Let || self.open == Keyword::Out {
                                    start += self.push_let(&line[start..]);
                                    c = start - 1;
                                    indentation = 0;
                                }
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
