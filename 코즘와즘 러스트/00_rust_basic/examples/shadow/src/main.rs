fn main() {
    let x = 5;

    println!("x's value is {x}");
    let x = x + 1;
    // x is shadowed by two's value

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // this fucking scope is over 
    }

    println!("The value of x is: {x}");
    // inner shadowing is returned to 6! scope is over  

    let spaces = "   ";
    let spaces = spaces.len();
    // diff between mut and shadwing
    // 1. immutable property can be preserved after shadowing
    // 2. and then the type of variable can be changed

    println!("the value of spaces is {spaces}");

}