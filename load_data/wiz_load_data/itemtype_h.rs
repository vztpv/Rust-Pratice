

pub struct ItemType<T>
{
    pub name : String,
    pub data : T
}

pub trait ItemTypeInterface<T>
{
    fn make(name : String, data : T) -> Self;
}


