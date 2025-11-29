const THREE_HOURSE_IN_SECOND:u32 = 60 * 60 * 3;

fn main() {
    // let x = 6;
    // println!("x : {x}");
    // x = 5;
    // println!("x : {x}");

    let mut x = 6;
    println!("x : {x}");
    x = 5;
    println!("x : {x}");

    // let str = "   ";
    // let space = str.len();

    // str = "     ";
    // space = str.len();
    
    let mut str = "   ";
    let mut space = str.len();
    println!("langht of string \"{str}\" is {space}");

    str = "     ";
    space = str.len();
    println!("new langht of string \"{str}\" is {space}");
    println!("const veriable THREE_HOURSE_IN_SECOND value before shadowing is {THREE_HOURSE_IN_SECOND}");
    const THREE_HOURSE_IN_SECOND:u32 = 60 * 60 * 60 * 3;
    println!("after shadowing is {THREE_HOURSE_IN_SECOND}");
}

