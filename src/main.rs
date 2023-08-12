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

trait LookUpTable {
    fn insert(&mut self, value: &[bool]);
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool];
}

impl LookUpTable for BinaryTrie {
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
                        let mut new_child = Self::new();
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
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool] {
        let mut node_to_search = self;
        let mut best_match_index = 0;
        for (depth, bit) in value.iter().enumerate() {
            let take_left = !*bit;
            if node_to_search.filled {
                best_match_index = depth;
            }
            let target_node = if take_left {
                &node_to_search.left
            } else {
                &node_to_search.right
            };
            match target_node {
                Some(ref x) => node_to_search = x,
                None => break,
            }
        }
        &value[..best_match_index]
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
    root.insert(&bool_slice_from_str("00"));
    root.insert(&bool_slice_from_str("1010"));
    root.insert(&bool_slice_from_str("110"));
    root.insert(&bool_slice_from_str("101000"));
    println!("{root:#?}");
    println!("{:?}", root.search(&bool_slice_from_str("101000")));
}
