use core::f64;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
};

use crate::grammar::{
    ts2glexer::TS2GLexer,
    ts2gparser::{
        AddContextAttrs, EqContextAttrs, ExponentContextAttrs, ExprstatementContextAttrs,
        IdContextAttrs, LetContextAttrs, MultiplyContextAttrs, NumberContextAttrs,
        ParenthesisContextAttrs, PrintContextAttrs, SContextAttrs, TS2GParser,
        TS2GParserContextType,
    },
    ts2gvisitor::TS2GVisitorCompat,
};

pub mod ts2glexer;
pub mod ts2glistener;
pub mod ts2gparser;
pub mod ts2gvisitor;

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

    pub fn parse(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        let lexer = TS2GLexer::new(InputStream::new(input));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = TS2GParser::new(token_source);
        let root = parser.s().unwrap();
        self.visit(&*root);

        Ok(())
    }
}

impl ParseTreeVisitorCompat<'_> for TS2G {
    type Node = TS2GParserContextType;
    type Return = ();

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.unit
    }
}

impl TS2GVisitorCompat<'_> for TS2G {
    fn visit_s(&mut self, ctx: &ts2gparser::SContext<'_>) -> Self::Return {
        for statement in ctx.statement_all() {
            self.visit(&*statement);
        }
    }

    fn visit_exprstatement(&mut self, ctx: &ts2gparser::ExprstatementContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap());
        self.stack.pop().unwrap();
    }

    fn visit_let(&mut self, ctx: &ts2gparser::LetContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        let t = ctx.TYPE().unwrap().symbol.text.to_string();
        self.visit(&*ctx.expr().unwrap());
        let res = self.stack.pop().unwrap();
        self.vars.insert(id, res);
    }

    fn visit_print(&mut self, ctx: &ts2gparser::PrintContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap());
        println!("{}", self.stack.pop().unwrap());
    }

    fn visit_add(&mut self, ctx: &ts2gparser::AddContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr(0).unwrap());
        self.visit(&*ctx.expr(1).unwrap());
        let r = self.stack.pop().unwrap();
        let l = self.stack.pop().unwrap();
        if ctx.ADD().is_some() {
            self.stack.push(l + r);
        } else {
            self.stack.push(l - r);
        }
    }

    fn visit_number(&mut self, ctx: &ts2gparser::NumberContext<'_>) -> Self::Return {
        let n = ctx.INT().unwrap().get_text();
        self.stack.push(Value {
            v: Number {
                f64: n.parse::<f64>().unwrap(),
            },
            t: NumType::F64,
        });
    }

    fn visit_e(&mut self, _ctx: &ts2gparser::EContext<'_>) -> Self::Return {
        self.stack.push(Value {
            v: Number {
                f64: f64::consts::E,
            },
            t: NumType::F64,
        });
    }

    fn visit_pi(&mut self, _ctx: &ts2gparser::PiContext<'_>) -> Self::Return {
        self.stack.push(Value {
            v: Number {
                f64: f64::consts::PI,
            },
            t: NumType::F64,
        });
    }

    fn visit_id(&mut self, ctx: &ts2gparser::IdContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        self.stack.push(*self.vars.get(&id).unwrap());
    }

    fn visit_multiply(&mut self, ctx: &ts2gparser::MultiplyContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr(0).unwrap());
        self.visit(&*ctx.expr(1).unwrap());
        let r = self.stack.pop().unwrap();
        let l = self.stack.pop().unwrap();
        if ctx.MUL().is_some() {
            self.stack.push(l * r);
        } else {
            self.stack.push(l / r);
        }
    }

    fn visit_parenthesis(&mut self, ctx: &ts2gparser::ParenthesisContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap());
    }

    fn visit_exponent(&mut self, ctx: &ts2gparser::ExponentContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr(0).unwrap());
        self.visit(&*ctx.expr(1).unwrap());
        let r = self.stack.pop().unwrap();
        let l = self.stack.pop().unwrap();
        self.stack.push(l.powf(r));
    }

    fn visit_eq(&mut self, ctx: &ts2gparser::EqContext<'_>) -> Self::Return {
        let id = ctx.ID().unwrap().get_text();
        self.visit(&*ctx.expr().unwrap());
        let v = self.stack.last().unwrap();
        self.vars.insert(id, *v).unwrap();
    }
}
