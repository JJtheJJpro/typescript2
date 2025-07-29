use core::f64;
use std::{collections::HashMap, error::Error};

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

struct Value {
    v: Number,
    t: NumType,
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
        self.visit(&*ctx.expr().unwrap());
        let res = self.stack.pop().unwrap();
        self.vars.insert(id, Value { f64: res });
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
        self.stack.push(n.parse::<f64>().unwrap());
    }

    fn visit_e(&mut self, _ctx: &ts2gparser::EContext<'_>) -> Self::Return {
        self.stack.push(f64::consts::E);
    }

    fn visit_pi(&mut self, _ctx: &ts2gparser::PiContext<'_>) -> Self::Return {
        self.stack.push(f64::consts::PI);
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
