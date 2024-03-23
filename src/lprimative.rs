///Lua language primative types
#[derive(Debug)]
pub enum LPrimitive {
    NIL,            //0
    BOOL(bool),     //1
    NUMBER(f64),    //3
    STRING(String), //4
}
