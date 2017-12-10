

use wiz_load_data::itemtype_h::ItemType;
use wiz_load_data::itemtype_h::ItemTypeInterface;


impl<T> ItemTypeInterface<T> for ItemType<T>
{
    fn make(name : String, data : T) -> Self {
        ItemType{name:name, data:data} // return ItemType{name:name, item:item};
    }
}

