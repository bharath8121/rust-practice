use std::collections::HashMap;

struct TrieNode {
    children: [Option<TrieNode>; 26],
}

struct Trie {
    root: TrieNode,
}


impl Trie {
    fn new() -> Trie {
        Trie{
            root: TrieNode{children: [None; 26]},
        }
    }

    fn insert(&mut self, word: &str) {
        let mut curr_node = &mut self.root;
        for c in word.chars() {
            let ascii = c as u8;
            if curr_node.children[ascii as usize].is_none() {
                curr_node.children[ascii as usize] = Some(TrieNode{children: [None; 26]});
            }
            curr_node = curr_node.children[ascii as usize].as_mut().unwrap();
        }
    }

    fn search(&self, word: &str) -> bool {
        todo!()
    }

}