#[derive(Debug, Clone)]
struct BinaryTrie {
    filled: bool,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl BinaryTrie {
    fn new() -> Self {
        Self {
            filled: false,
            left: None,
            right: None,
        }
    }
}

impl BinaryTrie {
    fn insert(&mut self, value: &[bool]) {
        match value.split_first() {
            Some((first, rest)) => {
                let take_left = !*first;
                let target_node = if take_left {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    &mut Some(ref mut child) => child.insert(rest),
                    &mut None => {
                        let mut new_child = Self {
                            filled: false,
                            left: None,
                            right: None,
                        };
                        if take_left {
                            new_child.left = Some(Box::new(new_child.clone()));
                        } else {
                            new_child.right = Some(Box::new(new_child.clone()));
                        }
                        new_child.insert(rest);
                        *target_node = Some(Box::new(new_child));
                    }
                };
            }
            None => {
                self.filled = true;
            }
        }
    }
}

fn bool_slice_from_str(value: &str) -> Vec<bool> {
    let mut result = Vec::with_capacity(value.len());
    value.chars().for_each(|char| {
        result.push(match char {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid symbol"),
        })
    });
    result
}

fn main() {
    let mut root = BinaryTrie::new();
    root.insert(&bool_slice_from_str("10010"));
    println!("{root:#?}");
}
