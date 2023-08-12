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

// TODO: Can lookup time be optimised using binary search over lexicographically sorted entries
struct LinearLookup<'a> {
    entries: Vec<&'a [bool]>,
}

impl LinearLookup<'_> {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

trait LookUpTable<'table> {
    fn insert(&mut self, value: &'table [bool]);
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool];
}

impl LookUpTable<'_> for BinaryTrie {
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
        let mut last_match_depth = 0;
        for (depth, bit) in value.iter().enumerate() {
            let take_left = !*bit;
            if node_to_search.filled {
                last_match_depth = depth;
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
        &value[..last_match_depth]
    }
}

impl<'table> LookUpTable<'table> for LinearLookup<'table> {
    fn insert(&mut self, value: &'table [bool]) {
        if !self.entries.contains(&value) {
            self.entries.push(value);
        }
    }
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool] {
        let mut best_match_index = 0;
        for entry in self.entries.iter() {
            let mut matched_till = 0;
            for (i, bit) in entry.iter().enumerate() {
                match value.get(i) {
                    Some(test_bit) if test_bit == bit => (),
                    _ => break,
                }
                matched_till = i;
            }
            if matched_till > best_match_index {
                best_match_index = matched_till;
            }
        }
        &value[..=best_match_index]
    }
}

fn bool_vec_from_str(value: &str) -> Vec<bool> {
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
    let mut trie = BinaryTrie::new();
    let mut vec = LinearLookup::new();
    let entries = [
        &bool_vec_from_str("01"),
        &bool_vec_from_str("00"),
        &bool_vec_from_str("11"),
        &bool_vec_from_str("10"),
    ];
    let key = bool_vec_from_str("1011");
    for entry in entries {
        trie.insert(entry);
        vec.insert(entry);
    }
    println!("Trie  :\t{:?}", trie.search(&key));
    println!("Linear:\t{:?}", vec.search(&key));
}
