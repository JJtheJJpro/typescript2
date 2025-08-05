pub mod ast;

use crate::{
    ast::{Expr, Statement},
    ts2g::SParser,
};
use core::f64;
use lalrpop_util::lalrpop_mod;
use stopwatch::Stopwatch;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

lalrpop_mod!(ts2g);

#[repr(C)]
#[derive(Clone, Copy)]
union Number {
    u8: u8,
    i8: i8,
    u16: u16,
    i16: i16,
    u32: u32,
    i32: i32,
    u64: u64,
    i64: i64,
    f32: f32,
    f64: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NumType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
}
impl Display for NumType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumType::U8 => write!(f, "u8"),
            NumType::I8 => write!(f, "i8"),
            NumType::U16 => write!(f, "u16"),
            NumType::I16 => write!(f, "i16"),
            NumType::U32 => write!(f, "u32"),
            NumType::I32 => write!(f, "i32"),
            NumType::U64 => write!(f, "u64"),
            NumType::I64 => write!(f, "i64"),
            NumType::F32 => write!(f, "f32"),
            NumType::F64 => write!(f, "f64"),
        }
    }
}

#[derive(Clone, Copy)]
struct Value {
    v: Number,
    t: NumType,
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.t {
            NumType::U8 => write!(f, "{}", unsafe { self.v.u8 }),
            NumType::I8 => write!(f, "{}", unsafe { self.v.i8 }),
            NumType::U16 => write!(f, "{}", unsafe { self.v.u16 }),
            NumType::I16 => write!(f, "{}", unsafe { self.v.i16 }),
            NumType::U32 => write!(f, "{}", unsafe { self.v.u32 }),
            NumType::I32 => write!(f, "{}", unsafe { self.v.i32 }),
            NumType::U64 => write!(f, "{}", unsafe { self.v.u64 }),
            NumType::I64 => write!(f, "{}", unsafe { self.v.i64 }),
            NumType::F32 => write!(f, "{}", unsafe { self.v.f32 }),
            NumType::F64 => write!(f, "{}", unsafe { self.v.f64 }),
        }
    }
}
impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.t != rhs.t {
            panic!("Types {} and {} are not the same.", self.t, rhs.t)
        }

        match self.t {
            NumType::U8 => Self {
                t: self.t,
                v: Number {
                    u8: unsafe { self.v.u8 } + unsafe { rhs.v.u8 },
                },
            },
            NumType::I8 => Self {
                t: self.t,
                v: Number {
                    i8: unsafe { self.v.i8 } + unsafe { rhs.v.i8 },
                },
            },
            NumType::U16 => Self {
                t: self.t,
                v: Number {
                    u16: unsafe { self.v.u16 } + unsafe { rhs.v.u16 },
                },
            },
            NumType::I16 => Self {
                t: self.t,
                v: Number {
                    i16: unsafe { self.v.i16 } + unsafe { rhs.v.i16 },
                },
            },
            NumType::U32 => Self {
                t: self.t,
                v: Number {
                    u32: unsafe { self.v.u32 } + unsafe { rhs.v.u32 },
                },
            },
            NumType::I32 => Self {
                t: self.t,
                v: Number {
                    i32: unsafe { self.v.i32 } + unsafe { rhs.v.i32 },
                },
            },
            NumType::U64 => Self {
                t: self.t,
                v: Number {
                    u64: unsafe { self.v.u64 } + unsafe { rhs.v.u64 },
                },
            },
            NumType::I64 => Self {
                t: self.t,
                v: Number {
                    i64: unsafe { self.v.i64 } + unsafe { rhs.v.i64 },
                },
            },
            NumType::F32 => Self {
                t: self.t,
                v: Number {
                    f32: unsafe { self.v.f32 } + unsafe { rhs.v.f32 },
                },
            },
            NumType::F64 => Self {
                t: self.t,
                v: Number {
                    f64: unsafe { self.v.f64 } + unsafe { rhs.v.f64 },
                },
            },
        }
    }
}
impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.t != rhs.t {
            panic!("Types {} and {} are not the same.", self.t, rhs.t)
        }

        match self.t {
            NumType::U8 => Self {
                t: self.t,
                v: Number {
                    u8: unsafe { self.v.u8 } - unsafe { rhs.v.u8 },
                },
            },
            NumType::I8 => Self {
                t: self.t,
                v: Number {
                    i8: unsafe { self.v.i8 } - unsafe { rhs.v.i8 },
                },
            },
            NumType::U16 => Self {
                t: self.t,
                v: Number {
                    u16: unsafe { self.v.u16 } - unsafe { rhs.v.u16 },
                },
            },
            NumType::I16 => Self {
                t: self.t,
                v: Number {
                    i16: unsafe { self.v.i16 } - unsafe { rhs.v.i16 },
                },
            },
            NumType::U32 => Self {
                t: self.t,
                v: Number {
                    u32: unsafe { self.v.u32 } - unsafe { rhs.v.u32 },
                },
            },
            NumType::I32 => Self {
                t: self.t,
                v: Number {
                    i32: unsafe { self.v.i32 } - unsafe { rhs.v.i32 },
                },
            },
            NumType::U64 => Self {
                t: self.t,
                v: Number {
                    u64: unsafe { self.v.u64 } - unsafe { rhs.v.u64 },
                },
            },
            NumType::I64 => Self {
                t: self.t,
                v: Number {
                    i64: unsafe { self.v.i64 } - unsafe { rhs.v.i64 },
                },
            },
            NumType::F32 => Self {
                t: self.t,
                v: Number {
                    f32: unsafe { self.v.f32 } - unsafe { rhs.v.f32 },
                },
            },
            NumType::F64 => Self {
                t: self.t,
                v: Number {
                    f64: unsafe { self.v.f64 } - unsafe { rhs.v.f64 },
                },
            },
        }
    }
}
impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.t != rhs.t {
            panic!("Types {} and {} are not the same.", self.t, rhs.t)
        }

        match self.t {
            NumType::U8 => Self {
                t: self.t,
                v: Number {
                    u8: unsafe { self.v.u8 } * unsafe { rhs.v.u8 },
                },
            },
            NumType::I8 => Self {
                t: self.t,
                v: Number {
                    i8: unsafe { self.v.i8 } * unsafe { rhs.v.i8 },
                },
            },
            NumType::U16 => Self {
                t: self.t,
                v: Number {
                    u16: unsafe { self.v.u16 } * unsafe { rhs.v.u16 },
                },
            },
            NumType::I16 => Self {
                t: self.t,
                v: Number {
                    i16: unsafe { self.v.i16 } * unsafe { rhs.v.i16 },
                },
            },
            NumType::U32 => Self {
                t: self.t,
                v: Number {
                    u32: unsafe { self.v.u32 } * unsafe { rhs.v.u32 },
                },
            },
            NumType::I32 => Self {
                t: self.t,
                v: Number {
                    i32: unsafe { self.v.i32 } * unsafe { rhs.v.i32 },
                },
            },
            NumType::U64 => Self {
                t: self.t,
                v: Number {
                    u64: unsafe { self.v.u64 } * unsafe { rhs.v.u64 },
                },
            },
            NumType::I64 => Self {
                t: self.t,
                v: Number {
                    i64: unsafe { self.v.i64 } * unsafe { rhs.v.i64 },
                },
            },
            NumType::F32 => Self {
                t: self.t,
                v: Number {
                    f32: unsafe { self.v.f32 } * unsafe { rhs.v.f32 },
                },
            },
            NumType::F64 => Self {
                t: self.t,
                v: Number {
                    f64: unsafe { self.v.f64 } * unsafe { rhs.v.f64 },
                },
            },
        }
    }
}
impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if self.t != rhs.t {
            panic!("Types {} and {} are not the same.", self.t, rhs.t)
        }

        match self.t {
            NumType::U8 => Self {
                t: self.t,
                v: Number {
                    u8: unsafe { self.v.u8 } / unsafe { rhs.v.u8 },
                },
            },
            NumType::I8 => Self {
                t: self.t,
                v: Number {
                    i8: unsafe { self.v.i8 } / unsafe { rhs.v.i8 },
                },
            },
            NumType::U16 => Self {
                t: self.t,
                v: Number {
                    u16: unsafe { self.v.u16 } / unsafe { rhs.v.u16 },
                },
            },
            NumType::I16 => Self {
                t: self.t,
                v: Number {
                    i16: unsafe { self.v.i16 } / unsafe { rhs.v.i16 },
                },
            },
            NumType::U32 => Self {
                t: self.t,
                v: Number {
                    u32: unsafe { self.v.u32 } / unsafe { rhs.v.u32 },
                },
            },
            NumType::I32 => Self {
                t: self.t,
                v: Number {
                    i32: unsafe { self.v.i32 } / unsafe { rhs.v.i32 },
                },
            },
            NumType::U64 => Self {
                t: self.t,
                v: Number {
                    u64: unsafe { self.v.u64 } / unsafe { rhs.v.u64 },
                },
            },
            NumType::I64 => Self {
                t: self.t,
                v: Number {
                    i64: unsafe { self.v.i64 } / unsafe { rhs.v.i64 },
                },
            },
            NumType::F32 => Self {
                t: self.t,
                v: Number {
                    f32: unsafe { self.v.f32 } / unsafe { rhs.v.f32 },
                },
            },
            NumType::F64 => Self {
                t: self.t,
                v: Number {
                    f64: unsafe { self.v.f64 } / unsafe { rhs.v.f64 },
                },
            },
        }
    }
}
impl Value {
    pub fn powf(self, rhs: Self) -> Self {
        if self.t != rhs.t {
            panic!("Types {} and {} are not the same.", self.t, rhs.t)
        }

        match self.t {
            NumType::U8 => Self {
                t: self.t,
                v: Number {
                    u8: unsafe { self.v.u8 as f64 }.powf(unsafe { rhs.v.u8 } as f64) as u8,
                },
            },
            NumType::I8 => Self {
                t: self.t,
                v: Number {
                    i8: unsafe { self.v.i8 as f64 }.powf(unsafe { rhs.v.i8 } as f64) as i8,
                },
            },
            NumType::U16 => Self {
                t: self.t,
                v: Number {
                    u16: unsafe { self.v.u16 as f64 }.powf(unsafe { rhs.v.u16 } as f64) as u16,
                },
            },
            NumType::I16 => Self {
                t: self.t,
                v: Number {
                    i16: unsafe { self.v.i16 as f64 }.powf(unsafe { rhs.v.i16 } as f64) as i16,
                },
            },
            NumType::U32 => Self {
                t: self.t,
                v: Number {
                    u32: unsafe { self.v.u32 as f64 }.powf(unsafe { rhs.v.u32 } as f64) as u32,
                },
            },
            NumType::I32 => Self {
                t: self.t,
                v: Number {
                    i32: unsafe { self.v.i32 as f64 }.powf(unsafe { rhs.v.i32 } as f64) as i32,
                },
            },
            NumType::U64 => Self {
                t: self.t,
                v: Number {
                    u64: unsafe { self.v.u64 as f64 }.powf(unsafe { rhs.v.u64 } as f64) as u64,
                },
            },
            NumType::I64 => Self {
                t: self.t,
                v: Number {
                    i64: unsafe { self.v.i64 as f64 }.powf(unsafe { rhs.v.i64 } as f64) as i64,
                },
            },
            NumType::F32 => Self {
                t: self.t,
                v: Number {
                    f32: unsafe { self.v.f32 as f64 }.powf(unsafe { rhs.v.f32 } as f64) as f32,
                },
            },
            NumType::F64 => Self {
                t: self.t,
                v: Number {
                    f64: unsafe { self.v.f64 }.powf(unsafe { rhs.v.f64 }),
                },
            },
        }
    }
}

pub struct TS2G {
    unit: (),
    _errors: (),
    vars: HashMap<String, Value>,
    stack: Vec<Value>,
}
impl TS2G {
    pub fn init() -> Self {
        Self {
            unit: (),
            _errors: (),
            vars: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn visit_statement(&mut self, statement: Box<Statement>) {
        match *statement {
            Statement::ExprStatement(expr) => {
                self.visit_expr(expr);
                self.stack.pop().unwrap();
            }
            Statement::Let(id, _t, expr) => {
                self.visit_expr(expr);
                let res = self.stack.pop().unwrap();
                self.vars.insert(id, res);
            }
            Statement::Print(expr) => {
                self.visit_expr(expr);
                println!("{}", self.stack.pop().unwrap());
            }
        }
    }
    pub fn visit_expr(&mut self, expr: Box<Expr>) {
        match *expr {
            Expr::Number(n) => {
                self.stack.push(Value {
                    v: Number { f64: n },
                    t: NumType::F64,
                });
            }
            Expr::Id(id) => {
                self.stack.push(*self.vars.get(&id).unwrap());
            }
            Expr::PI => {
                self.stack.push(Value {
                    v: Number {
                        f64: f64::consts::PI,
                    },
                    t: NumType::F64,
                });
            }
            Expr::E => {
                self.stack.push(Value {
                    v: Number {
                        f64: f64::consts::E,
                    },
                    t: NumType::F64,
                });
            }
            Expr::Parenthesis(expr) => self.visit_expr(expr),
            Expr::Exponent(l, r) => {
                self.visit_expr(l);
                self.visit_expr(r);
                let r = self.stack.pop().unwrap();
                let l = self.stack.pop().unwrap();
                self.stack.push(l.powf(r));
            }
            Expr::Multiply(l, r) => {
                self.visit_expr(l);
                self.visit_expr(r);
                let r = self.stack.pop().unwrap();
                let l = self.stack.pop().unwrap();
                self.stack.push(l * r);
            }
            Expr::Divide(l, r) => {
                self.visit_expr(l);
                self.visit_expr(r);
                let r = self.stack.pop().unwrap();
                let l = self.stack.pop().unwrap();
                self.stack.push(l / r);
            }
            Expr::Add(l, r) => {
                self.visit_expr(l);
                self.visit_expr(r);
                let r = self.stack.pop().unwrap();
                let l = self.stack.pop().unwrap();
                self.stack.push(l + r);
            }
            Expr::Sub(l, r) => {
                self.visit_expr(l);
                self.visit_expr(r);
                let r = self.stack.pop().unwrap();
                let l = self.stack.pop().unwrap();
                self.stack.push(l - r);
            }
            Expr::Eq(id, expr) => {
                self.visit_expr(expr);
                let v = self.stack.last().unwrap();
                self.vars.insert(id, *v).unwrap();
            },
        }
    }
}

/*
fn parse_statement<'a>(input: &'a str) -> Result<Box<Statement<'a>>, Box<dyn Error + 'a>> {
    let parse = StatementParser::new();
    match parse.parse(input) {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e.map_token(|e| Box::new(e)))),
    }
}
*/

/*
fn parse_expr<'a>(input: &'a str) -> Result<Box<Expr<'a>>, Box<dyn Error + 'a>> {
    let parse = ExprParser::new();
    match parse.parse(input) {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e.map_token(|e| Box::new(e)))),
    }
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    let parser = SParser::new();
    let mut sw = Stopwatch::start_new();
    let statements = parser.parse("let x:u64=1+1;print(x);x=x+10;print(x);")?;
    sw.stop();
    println!("Parsed code in {}ms", sw.elapsed().as_micros() as f32 / 1000f32);

    let mut ts2builder = TS2G::init();

    sw = Stopwatch::start_new();
    for statement in statements {
        ts2builder.visit_statement(statement);
    }
    sw.stop();
    println!("Checked code in {}ms", sw.elapsed().as_micros() as f32 / 1000f32);

    Ok(())
}
