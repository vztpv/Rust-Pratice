


fn how_many(x:i32) -> &'static str // ' : 작은 따옴표!
{
    match x
    {
        0 => "no",
        10 => "10",
        _ if x % 2 == 0 => "even", 
        _ => "a few"
    }
}

fn pattern_match() {
    for x in 0..12
    {
        println!("{} : {}", x, how_many(x));
    }
}


#[derive(Debug)] // for print("{}", instance of Point<T>);
struct Point<T>
{
    x: T,
    y: T
}

#[derive(Debug)] // for print("{}", instance of Line<T>);
struct Line<T>
{
    start:Point<T>,
    last:Point<T>
}



fn generic() {
    let a : Point<f64> = Point { x : 0.0, y : 0.0 }; // x : 0 -> error!
    let b : Point<f64> = Point { x : 1.2, y : 3.4 };

    let line = Line { start : a, last : b };

    println!( "{:?}", line);
}

fn main()
{
    pattern_match();
    generic();
}
