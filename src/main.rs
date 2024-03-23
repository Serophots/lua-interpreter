use bytecode::decode_bytecode;

pub(crate) mod lprimative;
mod bytecode;

fn main() {
    //Decode bytecode
    decode_bytecode();
    
    println!("DONE");
}
