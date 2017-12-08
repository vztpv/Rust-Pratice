

trait Animal
{
    fn make(name:&'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) 
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name : &'static str
}

impl Animal for Human
{   
    fn make(name:&'static str) -> Self {
        Human{name:name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
}


trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32 = 0;
        for x in self {result += *x; }
        return result;
    }
}

fn trait_test() 
{
    let a = Human::make("abc");

    a.talk();

    let x = vec![3, 4, 5];
    println!("sum is {}", x.sum());
}


fn main()
{
    trait_test();
}
