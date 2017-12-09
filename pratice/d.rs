


struct Circle { radius: f64}
struct Square { side: f64 }

trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("int : {}", *self)
    }
}

impl Printable for Circle
{
    fn format(&self) -> String
    {
        format!("Circle : {}", self.radius)
    }
}

impl Printable for Square
{
    fn format(&self) -> String
    {
        format!("Square : {}", self.side)
    }
}


fn print_it<T : Printable>(z:T) // static?
{
    println!("{}", z.format());
}

fn print_it_too(z: &Printable) // dynamic?
{
    println!("{}", z.format());
}

fn main()
{
    let a = 123;

    println!("{}", a.format());
    print_it(a);
    print_it_too(&a);

    let shapes:[&Printable; 4] = [
        &Square{side: 4.0},
        &Circle{radius:1.0},
        &Circle{radius:2.0},
        &Square{side:3.0}
    ];

    for (i, shape) in shapes.iter().enumerate()
    {
        println!("{} : {}", i , shape.format());
    }
}
