use std::mem;

const MEANING_OF_LIFE:u8 = 42; //doesn't hold memory space
static mut Z:i32 = 123; //if we are gonna use 'mut' on global static variable, multiple threads
//might be trying to access it so we will get error
// work around is use 'unsafe' tag while accessing it in main program.
mod sh;
mod control_flow;
mod datastructures;
mod more_ds;
mod functions_exp;
mod lifetime;
mod crates;


/*
=============================
fundamental datatype exercise
============================= */
fn fundamental_data_types() {
    let a:u8 = 123; // 8bits
    println!("a = {}", a);

    let mut b:i8 = 0; //mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; //32-bit signed integer
    println!("c = {}, size = {} bytes",c,mem::size_of_val(&c));

    c = -1;
    println!("c = {}, after modification",c);
    //i8, u8, i16, u16, i32, u32, i64, u64
    let z:isize = 123; //isize , integral datatype
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes on {}-bit OS", z, size_of_z, size_of_z * 8);

    let d = 'x';
    println!("d = {}, size = {} bytes",d, mem::size_of_val(&d));

    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

fn operators()
{
    //arithmetic

    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1; // -- ++
    a -= 2; //-= += *= /= %=

    println!("remainder of {} / {} = {}", a,3, (a%3));

    let a_cubed = i32::pow(a,3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b,b_cubed,b,b_to_pi);

    //bit wise
    let c = 1 | 2;
    println!("1|2 = {}", c);
    //shift operators
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!(" PI < 4.0 is {}",pi_less_4);
    let x = 5;
    let x_is_5 = x == 5; //true
}
fn scope_and_shadowing()
{
    let a = 123;
    let b = 1234;
    {
        let b = 456;
        println!("inside, b = {}", b);
        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside , a = {}", a);
}

fn main() {
    //scope_and_shadowing();
    println!("==============================");
    println!("==============================");
   // operators();
    println!("==============================");
    /*
    unsafe
        {
            println!("{}", Z);
        }
    */

    /*
    sh::stack_and_heap();
    control_flow::if_statement();
    control_flow::while_and_loop();
    control_flow::for_loop();
    control_flow::match_statement();
    datastructures::structures();
    datastructures::enums();
    datastructures::option();
    datastructures::arrays();
    datastructures::vectors();
    more_ds::slices();
    more_ds::strings();
    more_ds::tuples();
    more_ds::pattern_matching();
    more_ds::generics();

    functions_exp::functions();
    functions_exp::methods();
    functions_exp::closures();
    functions_exp::higher_order_functions();
    functions_exp::traits();


    lifetime::lifetime_eval();
    lifetime::borrowing();
    */

    crates::crates_eval();
}