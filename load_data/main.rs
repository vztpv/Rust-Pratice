

mod wiz_load_data;
use wiz_load_data::itemtype_h::ItemTypeInterface;


fn main()
{
    let x = wiz_load_data::itemtype_h::ItemType::make("abc".to_string(), "def".to_string());

    println!("{}", x.name);
    println!("{}", x.data);
}

