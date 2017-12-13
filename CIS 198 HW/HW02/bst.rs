
#[derive(Debug)]
pub struct BST
{
    root : Link
}

#[derive(Debug, Clone)]
pub enum BST_LINK_TYPE
{
    Empty,
    More(Box<Node>)
}


#[derive(Debug, Clone)]
pub struct Link
{
    pub link : BST_LINK_TYPE
}


#[derive(Debug, Clone)]
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
           (&BST_LINK_TYPE::Empty, _) => return false,
           (&BST_LINK_TYPE::More(ref a), &BST_LINK_TYPE::Empty) => return false,
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
        Link{link : BST_LINK_TYPE::More(Box::new(Node::new_from(x)))}
    }
}



impl BST
{
    pub fn new() -> Self {
        BST{root : Link::new()}
    }

    pub fn search(&self, x : i32) -> bool {
        let link = &self.root;
        
        BST::_search(link, x)
    }
    pub fn _search(link : &Link, x :i32) -> bool {
        match &link.link {
            &BST_LINK_TYPE::Empty => return false,
            &BST_LINK_TYPE::More(ref a) => {
                if x == a.value {
                    return true;
                }
                else {
                    return BST::_search(&a.left, x) || BST::_search(&a.right, x);
                }
            }
        }
        return false;
    }
    pub fn find(&self, x : i32) -> bool {
        self.search(x)
    }

    pub fn insert(&mut self, x : i32) -> bool {
        if self.find(x) {
            false
        }
        else {
            let mut chk : bool = false;
            {
                let link : &mut Link = &mut self.root;

                match &mut link.link {
                    &mut BST_LINK_TYPE::Empty => { 
                        chk = true;
                    },
                    _ => {
                        BST::_insert(link, x);
                    }
                }
            }
            if chk {
                println!("added {}", x);
                self.root = Link::new_from(x);
            }

            true
        }
    } 

    fn _insert(node : &mut Link, x : i32) {
        let mut chk : bool = false;
        {
            match &mut node.link {
                &mut BST_LINK_TYPE::Empty => { chk = true; },
                &mut BST_LINK_TYPE::More(ref mut a) => {
                    if x < a.value {
                        match &mut a.left.link {
                            &mut BST_LINK_TYPE::Empty => { }, // error
                            &mut BST_LINK_TYPE::More(ref mut b) => {
                                return BST::_insert(&mut b.left, x);
                            }
                        }
                    }
                    else if x > a.value {
                        match &mut a.right.link {
                            &mut BST_LINK_TYPE::Empty => { }, // error
                            &mut BST_LINK_TYPE::More(ref mut b) => {
                                return BST::_insert(&mut b.right, x);
                            }
                        }
                    }

                    if x < a.value {
                        println!("added {} ", x);
                        a.left = Link::new_from(x);
                    }
                    else if x > a.value {
                        println!("added {}", x);
                        a.right = Link::new_from(x);
                    }

                    return;
                }
            }
        }
        if chk {
            println!("added {}", x);
            *node = Link::new_from(x);
        }
    }
}

