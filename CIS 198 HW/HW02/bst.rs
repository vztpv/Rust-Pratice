

#[derive(Debug)]
pub struct BST
{
    root : Link
}

#[derive(Debug)]
pub enum BST_LINK_TYPE
{
    Empty,
    More(Box<Node>)
}


#[derive(Debug)]
pub struct Link
{
    pub link : BST_LINK_TYPE
}


#[derive(Debug)]
struct Node
{
    pub value : i32,
    pub left : Link,
    pub right : Link
}

impl PartialEq for BST_LINK_TYPE
{
    fn eq(&self, other : &BST_LINK_TYPE) -> bool {
       match (self, other)
       {
           (&BST_LINK_TYPE::Empty, &BST_LINK_TYPE::Empty) => return true,
           (&BST_LINK_TYPE::Empty, _) => panic!("error eq in BST_LINK_TYPE"),
           (&BST_LINK_TYPE::More(ref a), &BST_LINK_TYPE::Empty) => panic!("error eq in BST_LINK_TYPE_2"),
           (&BST_LINK_TYPE::More(ref a), &BST_LINK_TYPE::More(ref b)) => return a.value == b.value 
       }
    }
}


impl BST_LINK_TYPE
{
    fn comp(&self, x : i32) -> i32 {
        match (self, x) 
        {
            (&BST_LINK_TYPE::Empty, _) => return -2,
            (&BST_LINK_TYPE::More(ref a), x) => if a.value < x { return -1; } else { return 1 }
        }
    }
}

impl Node
{
    pub fn new() -> Self {
        Node{value : 0, left : Link{link : BST_LINK_TYPE::Empty}, right : Link{link : BST_LINK_TYPE::Empty}}
    }

    pub fn new_from(x :i32) -> Self {
        Node{value : x, left : Link{link : BST_LINK_TYPE::Empty}, right : Link{link : BST_LINK_TYPE::Empty}}
    }
}

impl Link
{
    pub fn new() -> Self {
        Link{link : BST_LINK_TYPE::Empty}
    }

    pub fn new_from(x : i32) -> Self {
        let mut temp : Link = Link::new();

        temp.link = BST_LINK_TYPE::More(Box::new(Node::new_from(x)));

        temp
    }

    pub fn left_child(&self) -> &Self {
        match &self.link {
            &BST_LINK_TYPE::Empty => panic!("this is nullptr in Link a"),
            &BST_LINK_TYPE::More(ref a) => return &a.left
        }
    }

    pub fn right_child(&self) -> &Self {
        match &self.link {
            &BST_LINK_TYPE::Empty => panic!("this is nullptr in Link b"),
            &BST_LINK_TYPE::More(ref a) => return &a.right
        }
    }

    pub fn insert_left_child(&mut self, x : i32) {
        match &mut self.link {
            &mut BST_LINK_TYPE::Empty => panic!("this is nullptr in Link c"),
            &mut BST_LINK_TYPE::More(ref mut a) => a.left = Link::new_from(x)
        }
    }

    pub fn insert_right_child(&mut self, x : i32) {
        match &mut self.link {
            &mut BST_LINK_TYPE::Empty => panic!("this is nullptr in Link d"),
            &mut BST_LINK_TYPE::More(ref mut a) => a.right = Link::new_from(x)
        }
    }
}

impl BST
{
    pub fn new() -> Self {
        BST{root : Link::new()}
    }
    pub fn search(&mut self, x : i32) -> (&mut Link, Link) {
        let mut p : &mut Link = &mut self.root;
        let mut q : Link = Link::new();

        loop {
            // if p.empty() { break; }
            // else if p->node.value > x { }
            // else if p->node.value < x { }
            // else { return true; }
            if p.link == BST_LINK_TYPE::Empty {
                break;
            }

            let _comp: i32 = p.link.comp(x);
            if _comp < 0 {
                let mut right = p.right_child();
                let mut chk_link : bool = false;
                match &right.link {
                    &BST_LINK_TYPE::Empty => { p = &mut right; },
                    _ => { p = &mut right; q = &mut right; }
                }
            }
            else if _comp > 0 {
                let mut left = p.left_child();

                let mut chk_link : bool = false;
                match left.link {
                    BST_LINK_TYPE::Empty => { p = &mut left; },
                    _ => { p = &mut left; q = &mut left; }
                }
            }   
            else { // _comp == 0
                break;
            }
        }

        (p, q)
    }
    pub fn find(&self, x : i32) -> bool {
        let searched = self.search(x);

        match searched.0.link {
            BST_LINK_TYPE::Empty => return false,
            _ => return true
        }
    }
    pub fn insert(&self, x :i32) -> bool {
        let mut valid : bool = false;
        let mut links : (&mut Link, Link) = self.search(x);

        match links.0.link {
            BST_LINK_TYPE::Empty => valid = true,
            _ => valid = false
        }

        if valid {
            let mut chk : i32 = 0;
            match &mut links.1.link {
                &mut BST_LINK_TYPE::Empty => {
                    chk = 0;
                 },
                &mut BST_LINK_TYPE::More(ref a) => { 
                    if a.value < x {
                        chk = 1; //links.1.insert_right_child(x);
                    }
                    else {
                        chk = 2; //links.1.insert_left_child(x);
                    }
                }
            }
            if chk == 1 { links.1.insert_right_child(x); }
            else if chk == 2 { links.1.insert_left_child(x); }
            else {
                // todo
            }
        }

        valid
    }
}
