/// An operation.
#[derive(Debug)]
pub enum Op {
    /// Collect input for a function.
    InLet { name: String, def: String },
    /// Collect input for a function.
    InVar { name: String, def: String },
    /// Function call.
    Call {
        name: String,
        var_a: String,
        operator: String,
        var_b: String,
    },
    /// Collect output from a function.
    Out {
        name: String,
        def: String,
        from: String,
    },
    ///
    Function { name: String },
    ///
    Type {},
}

#[derive(PartialEq, Copy, Clone)]
#[allow(unused)]
enum Keyword {
    None,
    Def,
    Let,
    Var,
    Out,
}

// #[cfg(feature = "shader")]
/// Util for creating 
mod shader {
    #![allow(unused)]

    use super::*;

    // Inputs
    const PRIMITIVE_VEC1: u8 = 1; // vector x1 (float)
    const PRIMITIVE_VEC2: u8 = 2; // vector x2
    const PRIMITIVE_VEC3: u8 = 3; // vector x3
    const PRIMITIVE_VEC4: u8 = 4; // vector x4
    const PRIMITIVE_MAT4: u8 = 5; // matrix 4x4
    const PRIMITIVE_INT1: u8 = 6; // integer
    const PRIMITIVE_BMAP: u8 = 7; // bitmap

    /// Outputs
    const PRIMITIVE_OUT3: u8 = 254; // Position
    const PRIMITIVE_OUT4: u8 = 255; // Color

    /// Operators
    const OPERATOR_ADD: u8 = 1;
    const OPERATOR_SUB: u8 = 2;
    const OPERATOR_MUL: u8 = 3;
    const OPERATOR_DIV: u8 = 4;
    const OPERATOR_MOD: u8 = 5;

    pub fn shader(ops: Vec<Op>) -> Vec<u8> {
        use std::collections::HashMap;
        let mut variables = HashMap::new();

        let mut out = Vec::new();
        let mut var_count = 0;

        for op in ops.iter() {
        match op {
            Op::InLet { name, def } | Op::InVar { name, def } => {
                match def.as_str() {
                    "View" => {
                        out.push(PRIMITIVE_MAT4);
                    },
                    "Vec1" => {
                        out.push(PRIMITIVE_VEC1);
                    },
                    "Vec2" => {
                        out.push(PRIMITIVE_VEC2);
                    },
                    "Vec3" => {
                        out.push(PRIMITIVE_VEC3);
                    },
                    "Vec4" => {
                        out.push(PRIMITIVE_VEC4);
                    },
                    "Bmap" => {
                        out.push(PRIMITIVE_BMAP);
                    },
                    "Int1" => {
                        out.push(PRIMITIVE_INT1);
                    },
                    _ => panic!("Unknown type!"),
                }
                variables.insert(name, (var_count, def.clone()));
                var_count += 1;
            }
            Op::Call {
                name,
                var_a,
                operator: _,
                var_b,
            } => {
                let var_a = variables.get(&var_a).unwrap_or_else(|| {
                     eprintln!("{:?}", variables);
                     panic!("No such variable {}", &var_a)
                });

                let var_b = variables.get(&var_b).unwrap_or_else(|| panic!("No such variable {}", &var_b));

                match var_b.1.as_str() {
                    "View" => {
                        out.push(PRIMITIVE_MAT4 + 128);
                    },
                    "Vec1" => {
                        out.push(PRIMITIVE_VEC1 + 128);
                    },
                    "Vec2" => {
                        out.push(PRIMITIVE_VEC2 + 128);
                    },
                    "Vec3" => {
                        out.push(PRIMITIVE_VEC3 + 128);
                    },
                    "Vec4" => {
                        out.push(PRIMITIVE_VEC4 + 128);
                    },
                    "Bmap" => {
                        out.push(PRIMITIVE_BMAP + 128);
                    },
                    "Int1" => {
                        out.push(PRIMITIVE_INT1 + 128);
                    },
                    _ => panic!("Unknown type!"),
                }

                match var_a.1.as_str() {
                    "View" => {
                        out.push(OPERATOR_MUL);
                    },
                    a => panic!("Unsupported operator function `{}`", a),
                }

                out.push(var_a.0);
                out.push(var_b.0);

                let def = var_b.1.clone();
                variables.insert(name, (var_count, def));
                var_count += 1;
            }
            Op::Out {
                name: _,
                def: _,
                from,
            } => {
                let from = variables.get(&from).unwrap_or_else(|| panic!("No such variable {}", &from));
                match from.1.as_str() {
                    "Vec3" => {
                        out.push(PRIMITIVE_OUT3);
                    },
                    "Vec4" => {
                        out.push(PRIMITIVE_OUT4);
                    },
                    _ => panic!("Unsupported output type!  Must be Vec3 or Vec4"),
                }
                out.push(from.0);
            }
            Op::Function { name: _ } => panic!("No functions allowed in shaders!"),
            Op::Type {} => panic!("No types allowed in shaders!"),
        }

        }

        out
    }
}

/// Turn Yeet code into portable shader code (Yote bytecode).
pub fn compile_shader(src_code: &str) -> Vec<u8> {
    let mut code = Code::new();
    code.parse(src_code.as_bytes());
    shader::shader(code.to_ops())
}

/// Convert Yote bytecode to GLSL shaders.
pub fn yote_to_rs(yote_bytecode: Vec<u8>) -> String {
    // Convert a number to text.
    fn num_to_text(l: u8) -> [u8; 2] {
        if l >= 128 {
            panic!("Number too high");
        }

        let a = (l >> 4) + b'a';
        let b = (l << 4) + b'a';

        [a, b]
    }

    let mut output = String::new();

    output.push_str("(\"uniform mat4 rotation;\n\
        attribute vec4 pos;\n\
        attribute vec4 color;\n\
        varying vec4 v_color;\n\
        void main() {\n\
            gl_Position = rotation * pos;\n\
            v_color = color;\n\
        }\\0\",\"precision mediump float;\n\
        varying vec4 v_color;\n\
        void main() {\n\
            gl_FragColor = v_color;\n\
        }\\0\")");

    output
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
                _ => {
                    if !comment {
                        eprintln!("If you want 2 commands on a line you need a semicolon `;`");
                        std::process::exit(1);
                    }
                }
            }

            c += 1;
        }
    }

    fn expression(
        &mut self,
        mut c: usize,
        line: &[u8],
        keyword: Keyword,
        name: String,
    ) -> usize {
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
                            _ => {}
                        }
                        c += 1;
                    }
                    self.ops.push(Op::InLet {
                        name,
                        def: unsafe {
                            std::str::from_utf8_unchecked(&line[start..c])
                        }
                        .to_string(),
                    });
                    self.nothing_else_allowed(c, line)
                }
                Keyword::Var => {
                    let start = c;
                    'd: loop {
                        match line[c] {
                            b'\0' | b' ' | b'\n' | b';' => {
                                break;
                            }
                            _ => {}
                        }
                        c += 1;
                    }
                    self.ops.push(Op::InVar {
                        name,
                        def: unsafe {
                            std::str::from_utf8_unchecked(&line[start..c])
                        }
                        .to_string(),
                    });
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
                    let could_b = 'c: loop {
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
                            _ => {}
                        }
                        c += 1;
                    };
                    if could_b {
                        let var_a = unsafe {
                            std::str::from_utf8_unchecked(&line[start..c - 1])
                        }
                        .to_string();

                        let start = c;

                        // Read operand b.
                        let could_c = 'b: loop {
                            match line[c] {
                                b'\0' | b'\n' | b';' => {
                                    break false;
                                }
                                b'.' => {
                                    panic!(
                                        "Function operators not supported yet!"
                                    );
                                }
                                b' ' => {
                                    c += 1;
                                    break line[c] != b' ';
                                }
                                _ => {}
                            }
                            c += 1;
                        };

                        if could_c {
                            eprintln!("Error: Too many operands!");
                            std::process::exit(1);
                        }

                        let var_b = unsafe {
                            std::str::from_utf8_unchecked(&line[start..c - 1])
                        }
                        .to_string();

                        self.ops.push(Op::Call {
                            name: "".to_string(),
                            var_a,
                            operator: "".to_string(),
                            var_b,
                        });
                        self.ops.push(Op::Out {
                            name,
                            def: "".to_string(),
                            from: "".to_string(),
                        });
                    } else {
                        self.ops.push(Op::Out {
                            name,
                            def: "".to_string(),
                            from: unsafe {
                                std::str::from_utf8_unchecked(
                                    &line[start..c - 1],
                                )
                            }
                            .to_string(),
                        });
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
    fn assignment(
        &mut self,
        mut c: usize,
        line: &[u8],
        keyword: Keyword,
        name: String,
    ) -> usize {
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
                    eprintln!("Semicolon `;` in the middle of assignment");
                    std::process::exit(1);
                }
                b'#' => {
                    comment = !comment;
                }
                b' ' => { /* Ignore spaces */ }
                _ => {
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

        'a: loop {
            match line[c] {
                b'\n' | b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b':' => {
                    self.ops.push(Op::Function {
                        name: unsafe {
                            std::str::from_utf8_unchecked(&line[..c])
                        }
                        .to_string(),
                    });
                    return self.nothing_else_allowed(c + 1, line);
                }
                _ => {}
            }

            c += 1;
        }
    }

    // A function or type.
    fn push_let(&mut self, line: &[u8]) -> usize {
        let mut c = 0;
        let mut last = 0;
        let mut end = false;

        'a: loop {
            match line[c] {
                b'\n' | b'\0' => {
                    eprintln!("Unexpected end of line!");
                    std::process::exit(1);
                }
                b' ' => { /* ignore trailing whitespace */ end = true; last = c; }
                b':' => {
                    if end == false {
                        last = c;
                    }
                    let name =
                        unsafe { std::str::from_utf8_unchecked(&line[..last]) }
                            .to_string();
                    return self.assignment(c + 1, line, self.open, name);
                }
                _ => {
                    if end {
                        eprintln!("Can't have spaces in variable name!");
                        std::process::exit(1);
                    }
                }
            }

            c += 1;
        }
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
                    start = c + 1;
                    indentation = 0;
                }
                // Found a keyword (keywords always end with a space)
                b' ' => {
                    // Maybe it's just indentation that we can ignore.
                    if line[c - 1] == b'\n'
                        || line[c - 1] == b' '
                        || line[c - 1] == b'\t'
                    {
                        start = c + 1;
                        c += 1;
                        indentation += 1;
                        continue;
                    }

                    //                    print!("Keyword");
                    match line[c + 1] {
                        b'+' | b'-' | b'*' | b'/' | b'%' | b'.' | b':'
                        | b'!' | b'^' | b'&' | b'|' | b'(' | b'"' | b'\'' => {}
                        _ => match &line[start..c] {
                            // Define function or type.
                            b"def" => {
                                start = c + 1;
                                start += self.push_def(&line[start..]);
                                c = start - 1;
                                req = indentation + 4;
                                indentation = 0;
                            }
                            // Declare immutable variable.
                            b"let" => {
                                if indentation != req
                                    && self.open == Keyword::None
                                {
                                    eprintln!("Wrong indentation");
                                    std::process::exit(1);
                                }
                                start = c + 1;
                                self.open = Keyword::Let;
                                start += self.push_let(&line[start..]);
                                c = start - 1;
                                req = indentation + 4;
                                indentation = 0;
                            }
                            // Declare mutable variable.
                            b"var" => {}
                            // Set function output & return.
                            b"out" => {
                                if indentation != req
                                    && self.open == Keyword::None
                                {
                                    eprintln!("Wrong indentation");
                                    std::process::exit(1);
                                }
                                start = c + 1;
                                self.open = Keyword::Out;
                                start += self.push_let(&line[start..]);
                                c = start - 1;
                                req = indentation + 4;
                                indentation = 0;
                            }
                            a => {
                                let a =
                                    unsafe { std::str::from_utf8_unchecked(a) };

                                if indentation != req {
                                    println!("{} {}", indentation, req);

                                    eprintln!("Unknown keyword `{}`", a);
                                    std::process::exit(1);
                                }

                                if self.open == Keyword::Let
                                    || self.open == Keyword::Out
                                {
                                    start += self.push_let(&line[start..]);
                                    c = start - 1;
                                    indentation = 0;
                                }
                            }
                        },
                    }
                }
                b'+' | b'-' | b'*' | b'/' | b'%' | b'.' | b':' | b'!'
                | b'^' | b'&' | b'|' | b'(' | b'"' | b'\'' => {}
                b'\0' => {
                    return;
                }
                _ => {}
            }

            c += 1;
        }
    }

    /// Convert to `Op`s.
    pub fn to_ops(self) -> Vec<Op> {
        self.ops
    }
}
