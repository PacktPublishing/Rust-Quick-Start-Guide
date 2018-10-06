use std::fmt::Display;

struct TreeNode<K, V> where K: PartialOrd + PartialEq {
    key: K,
    value: V,
    lesser: Option<Box<TreeNode<K, V>>>,
    greater: Option<Box<TreeNode<K, V>>>,
}

impl<K, V> TreeNode<K, V> where K: PartialOrd + PartialEq {
    fn set(&mut self, key: K, value: V) {
        if key == self.key {
            self.value = value;
        }
        else if key < self.key {
            match self.lesser {
                None => {
                    self.lesser = Some(Box::new(TreeNode {key, value, lesser: None, greater: None }));
                },
                Some(ref mut lesser) => {
                    lesser.set(key, value);
                }
            }
        }
        else {
            match self.greater {
                None => {
                    self.greater = Some(Box::new(TreeNode {key, value, lesser: None, greater: None }));
                }
                Some(ref mut greater) => {
                    greater.set(key, value);
                }
            }
        }
    }

    fn get_ref(&self, key: K) -> Result<&V, String> {
        if key == self.key {
            return Ok(&self.value);
        }
        else if key < self.key {
            match self.lesser {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref lesser) => {
                    return lesser.get_ref(key);
                }
            }
        }
        else {
            match self.greater {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref greater) => {
                    return greater.get_ref(key);
                }
            }
        }
    }

    fn get_mut(&mut self, key: K) -> Result<&mut V, String> {
        if key == self.key {
            return Ok(&mut self.value);
        }
        else if key < self.key {
            match self.lesser {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref mut lesser) => {
                    return lesser.get_mut(key);
                }
            }
        }
        else {
            match self.greater {
                None => {
                    return Err("No such key".to_string());
                }
                Some(ref mut greater) => {
                    return greater.get_mut(key);
                }
            }
        }
    }
}

pub struct Tree<K, V> where K: PartialOrd + PartialEq {
    root: Option<Box<TreeNode<K, V>>>,
}

impl<K, V> Tree<K, V> where K: PartialOrd + PartialEq {
    pub fn new() -> Tree<K, V> {
        Tree { root: None }
    }

    pub fn set(&mut self, key: K, value: V) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode { key, value, lesser: None, greater: None }));
            }
            Some(ref mut root) => {
                root.set(key, value);
            }
        }
    }

    pub fn get_ref(&self, key: K) -> Result<&V, String> {
        match self.root {
            None => {
                return Err("No such key".to_string());
            }
            Some(ref root) => {
                return root.get_ref(key);
            }
        }
    }

    pub fn get_mut(&mut self, key: K) -> Result<&mut V, String> {
        match self.root {
            None => {
                return Err("No such key".to_string());
            }
            Some(ref mut root) => {
                return root.get_mut(key);
            }
        }
    }
}

pub enum NotOrdered {
    A,
    B,
    C,
}

fn print_generic<T>(value: T) where T: Display {
    println!("{}", value);
}

fn print_trait(value: &dyn Display) {
    println!("{}", value);
}

fn requires_trait(value: impl Display) -> impl Display {
    value
}

// fn faulty_return(sel: bool) -> impl Display {
//     if sel {
//         return 52;
//     } else {
//         return "Oh no";
//     }
// }

fn higher_order(f: impl FnOnce(u32) -> u32) {
    f(5);
}

// fn higher_order2<F>(f: F) where F: FnOnce(u32) -> u32 {
//     f(5);
// }

// fn higher_order3<F: FnOnce(u32) -> u32>(f: F) {
//     f(5);
// }

// fn higher_order4<F>(f: F) -> u32 where F: FnOnce(u32) -> u32 {
//     f(5)
// }

fn main() {
    let mut tree: Tree<&'static str, f32> = Tree::new();

    tree.set("first key", 12.65);
    tree.set("second key", 99.999);
    tree.set("third key", -128.5);
    tree.set("fourth key", 67.21);

    let mut y = "y".to_string();
    higher_order(|x: u32| {
        y.push('X');
        println!("In the closure, y is now {}", y);
        x
    });
    println!("After higher_order, y is {}", y);

    println!("tree.get_ref(\"third key\") is {}", match tree.get_ref("third key") {
        Err(_) => {println!("Invalid!"); &0.0},
        Ok(x) => x,
    });
}
