

mod wiz_load_data;
use wiz_load_data::itemtype_h::ItemTypeInterface;


fn main()
{
    let mut x = wiz_load_data::itemtype_h::ItemType::<String>::Make("abc".to_string(), "def".to_string());

    x.name = "ggg".to_string();

    println!("{}", x.GetName());
    println!("{}", x.GetData());
}

