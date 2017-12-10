

pub struct ItemType<T>
{
    pub name : String,
    pub data : T
}

pub trait ItemTypeInterface<T>
{
    fn Make(name : String, data : T) -> Self;
    fn GetName(&self) -> &String;
    fn GetData(&self) -> &T;
}


