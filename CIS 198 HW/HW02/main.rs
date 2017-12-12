

mod bst;

fn main()
{
    // test
    let mut test : bst::BST = bst::BST::new();

    if test.find(3) {
        println!("find 3");
    }

    test.insert(4);

    if test.find(4) {
        println!("finded 4-1");
    }


    if !test.insert(4) {
        println!("no 4");
    }

    if test.find(4) {
        println!("finded 4-2");
    }

}

