use std::cell::RefCell;
use std::rc::{Rc, Weak};

// I am gussing this is just for the assingining the type here just like the varible it will store the type here
type NodeRef = Rc<RefCell<Node>>;

// have to implement the clone because need to give the root and the clone same node here
#[derive(Debug, Clone)]
pub enum NodeKind {
    File,
    Directory(Vec<NodeRef>),
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    kind: NodeKind,
    parent: Option<Weak<RefCell<Node>>>,
}

// here I belive this are like the pointer here like the root pointer and current pointer
//

pub struct FileSystem {
    root: NodeRef,
    current: NodeRef,
}

impl FileSystem {
    pub fn new() -> Self {
        let node = Node {
            name: format!("root"),
            parent: None,

            kind: { NodeKind::Directory(vec![]) },
        };

        let root = Rc::new(RefCell::new(node));

        let current = Rc::clone(&root);

        // I think there is some syntax error in this hre
        // FileSystem {
        //     current: Rc::new(RefCell::new(current_node.clone())),
        //     root: Rc::clone(&self),
        // }

        FileSystem { root, current }
    }

    pub fn mkdir(&self, name: String) {
        let child_node = Node {
            kind: NodeKind::Directory(vec![]),
            name,
            parent: Some(Rc::downgrade(&self.current)),
        };

        let mut current = self.current.borrow_mut();

        if let NodeKind::Directory(children) = &mut current.kind {
            children.push(Rc::new(RefCell::new(child_node)));
        }
    }

    pub fn touch(&self, name: String) {
        let child_node = Node {
            kind: NodeKind::File,
            name,
            parent: Some(Rc::downgrade(&self.current)),
        };

        let mut current = self.current.borrow_mut();

        if let NodeKind::Directory(children) = &mut current.kind {
            children.push(Rc::new(RefCell::new(child_node)));
        }
    }

    pub fn ls(&self) {
        let current = self.current.borrow();

        if let NodeKind::Directory(children) = &current.kind {
            let mut file_and_dir = vec![];

            for child in children {
                if let NodeKind::Directory(_) = &child.borrow().kind {
                    let dir_name = format!("{}/", child.borrow().name);

                    file_and_dir.push(dir_name);
                } else {
                    // child.borrow().name I could not use this in the push here was giving the move error
                    let file_name = format!("{}", &child.borrow().name);
                    file_and_dir.push(file_name);
                }
            }

            println!("{}", file_and_dir.join("  "));
        }
    }

    pub fn cd(&mut self, name: String) {
        let mut dest = None;
        if name == ".." {
            let current = self.current.borrow();

            match &current.parent {
                Some(weak_ref) => {
                    let strong_ref = weak_ref.upgrade();
                    dest = strong_ref;
                }
                None => {
                    println!("It is the root we can't go further up");
                    return;
                }
            }
        }
        if let Some(dest_ptr) = dest {
            self.current = dest_ptr;
            println!("Moved to parent dir");

            return;
        };

        let target = {
            let current = self.current.borrow();
            let mut found = None;

            if let NodeKind::Directory(dir) = &current.kind {
                for dest in dir {
                    let d = dest.borrow();

                    if let NodeKind::Directory(_) = &d.kind {
                        if d.name == name {
                            found = Some(Rc::clone(dest));
                        }
                    }
                }
            }

            found
        };

        match target {
            Some(dest_ptr) => self.current = dest_ptr,
            None => println!("cd: no such directory: {name}"),
        }
    }

    // pwd we have to print the location of the current folder here like
    //
    // I am struck here again
    //     pub fn pwd(&self) {
    //         let mut full_path: Vec<String> = vec![];
    //         let mut current_node = self.current.borrow();
    //
    //         loop {
    //             full_path.push(current_node.name.clone());
    //
    //             if current_node.parent.is_none() {
    //                 break;
    //             }
    //
    //             current_node = match current_node.parent {
    //                 Some(weak_node) => {
    //                     let strong_node = weak_node.upgrade().unwrap();
    //
    //                     strong_node
    //                 }
    //
    //                 None => {
    //                     println!("it does't have the parents here")
    //                 }
    //             }
    //         }
    //     }
    //
    // ok here we are trying to perform the read and write operation
    //
    // like reading the name of the current node
    // and then moving the current node to parent node here
    // how do we move up the parent here
    //
    //
    pub fn pwd(&self) -> String {
        // this is temperory cursor that I have used here instead of changing the self.current
        let mut cursor = Rc::clone(&self.current);

        let mut abs_addr: Vec<String> = vec![];

        loop {
            let next_ptr = {
                abs_addr.push(cursor.borrow().name.clone());

                if cursor.borrow().parent.is_none() {
                    break;
                }

                cursor.borrow().parent.clone()
            };

            if let Some(weak_ptr) = next_ptr {
                let strong_ptr: Rc<RefCell<Node>> = if let Some(str_node) = weak_ptr.upgrade() {
                    let str_node: Rc<RefCell<Node>> = str_node;
                    str_node
                } else {
                    break;
                };

                cursor = strong_ptr;
            }
        }

        abs_addr.reverse();

        format!("/{}", abs_addr.join("/"))
    }

    pub fn tree(&self) {
        Self::print_node(&self.root, 0);
    }

    fn print_node(node: &NodeRef, depth: usize) {
        let n = node.borrow();
        let indent = " ".repeat(depth);

        match &n.kind {
            NodeKind::Directory(children) => {
                println!("{indent}{}/", n.name);

                for child in children {
                    Self::print_node(child, depth + 1);
                }
            }
            NodeKind::File => {
                println!("{indent}{}", n.name);
            }
        }
    }

    pub fn rm(&self, name: &str) {
        let mut current_mut = self.current.borrow_mut();

        // here I got confused againg like why do we need the mut when current_mut is already there
        if let NodeKind::Directory(children) = &mut current_mut.kind {
            let before_len = children.len();
            children.retain(|c| c.borrow().name != name);
            if children.len() == before_len {
                println!("rm: no such file or directory: {name}");
            }
        }
    }

    pub fn _debug_counts(&self) {
        println!("strong={}", Rc::strong_count(&self.root));
        println!("weak={}", Rc::weak_count(&self.root));
    }
}
