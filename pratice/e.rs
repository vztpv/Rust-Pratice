
fn main()
{
    let v = vec![1,2,3];

    //let v2 = v;
    //println!("{:?}", v); // error
    //println!("{:?}", v2); // ok

    let foo = |v:Vec<i32>| ();

    foo(v);

  //  println!("{:?}", v); // error

    {
        let u = 1;
        let u2 = u;

        println!("{}", u); // ok
    }


    {
        let u = Box::new(1);
        let u2 = u;

   //     println!("{}", u); // error
    }
}

