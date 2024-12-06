use std::fmt;

use crate::interpreter::{cfunction::CFunction, closure::LClosure};

///Lua primitive types
#[derive(Clone, Debug)]
pub enum LPrimitive {
    NIL,            //0
    BOOL(bool),     //1
    NUMBER(f64),    //3
    STRING(String), //4
}
impl fmt::Display for LPrimitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LPrimitive::NIL => write!(f, "nil"),
            LPrimitive::BOOL(true) => write!(f, "true"),
            LPrimitive::BOOL(false) => write!(f, "false"),
            LPrimitive::NUMBER(n) => write!(f, "{}", n),
            LPrimitive::STRING(s) => write!(f, "{}", s),
        }
    }
}

///Any lua value, including primitives
#[derive(Debug)]
pub enum LValue<'i> {
    //Constants
    LPrimitive(LPrimitive),

    //Functions
    LClosure(LClosure<'i>),
    CClosure(CFunction<'i>),

    //Table

    //Thread

    //UserData is a pointer to user memory I guess for embedded applications

    //Internal types
    EmptyRegister,
}
impl<'i> Clone for LValue<'i> {
    fn clone(&self) -> Self {
        match self {
            LValue::LPrimitive(p) => LValue::LPrimitive(p.clone()),

            LValue::LClosure(_) => panic!("Attempted to clone LCLosure primitive type"),
            LValue::CClosure(_) => panic!("Attempted to clone LCLosure primitive type"),

            // LValue::UserData => panic!("Attempted to clone UserData primitive type"),
            LValue::EmptyRegister => LValue::EmptyRegister,
            // LValue::ByRef(_) => panic!("Atemtpted to clone LValue::ByRef"),
        }
    }
}
impl<'i> fmt::Display for LValue<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::LPrimitive(l) => write!(f, "{}", l),
            _ => write!(f, "[display not implemented for this LValue]"),
        }
    }
}
