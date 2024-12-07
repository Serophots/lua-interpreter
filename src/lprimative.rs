use std::fmt;

use crate::interpreter::{cfunction::CFunction, lclosure::LClosure};

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
}
impl<'i> Default for LValue<'i> {
    fn default() -> LValue<'i> {
        LValue::LPrimitive(LPrimitive::NIL)
    }
}
impl<'i> Clone for LValue<'i> {
    fn clone(&self) -> Self {
        match self {
            LValue::LPrimitive(p) => LValue::LPrimitive(p.clone()),

            LValue::LClosure(_) => panic!("cannot clone LCLosure primitive type"),
            LValue::CClosure(_) => panic!("cannot clone LCLosure primitive type"),
        }
    }
}
impl<'i> fmt::Display for LValue<'i> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LValue::LPrimitive(l) => write!(f, "{}", l),
            LValue::CClosure(c) => write!(f, "CClosure: {:p}", c), // TODO: Addresses in rust aren't a great unique identifier since things move a tonne
            LValue::LClosure(l) => write!(f, "LClosure: {:p}", l), // Maybe attach a unqiue ID to each proto for the LClosure's to be ID'ed and then make CClosures a full type and do something similar there
        }
    }
}
