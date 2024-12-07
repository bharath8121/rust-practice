use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_leaf: bool,
}

pub struct Trie {
    root: TrieNode,
}


const ARRAY_REPEAT_VALUE: Option<Box<TrieNode>> = None;
impl Trie {
    pub fn new() -> Trie {
        Trie{
            root: TrieNode{children: [ARRAY_REPEAT_VALUE; 26], is_leaf: false},
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr_node = &mut self.root;
        for c in word.chars() {
            let ascii = (c as u8) - 97;
            if curr_node.children[ascii as usize].is_none() {
                curr_node.children[ascii as usize] = Some(Box::new(TrieNode{children: [ARRAY_REPEAT_VALUE; 26], is_leaf: false}));
            }
            curr_node = curr_node.children[ascii as usize].as_mut().unwrap();
        }
        curr_node.is_leaf = true;
    }

    pub fn search(&self, word: &str) -> bool {
        let mut curr_node = &self.root;
        for c in word.chars() {
            let ascii = (c as u8) - 97;
            if curr_node.children[ascii as usize].is_none() {
                return false;
            }
            curr_node = curr_node.children[ascii as usize].as_ref().unwrap();
        }
        curr_node.is_leaf
    }

}