

union IntOrFloat
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe
    {
        match iof {
            IntOrFloat { i : 24 } => { println!("int is {}", iof.i); }
            IntOrFloat { f } => { println!("f32 = {}", f); }
        }
    }
}

fn unions()
{
    let iof = IntOrFloat { f : 3.14 };

    unsafe {
       // iof.i = 42;

        let value = iof.i;

        println!("value is {}", value);
    }
    process_value(iof);   
}

fn options()
{
    let x = 3.0;
    let y = 2.0;

    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };

    println!("{:?}", result); // ok.
    //println!("{}", result); // error!
}

fn main()
{
    options();
    unions();
}
