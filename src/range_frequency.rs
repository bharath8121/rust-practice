use std::collections::HashMap;

struct SegmentTreeNode {
    left_index: i32,
    right_index: i32,
    left: Option<Box<SegmentTreeNode>>,
    right: Option<Box<SegmentTreeNode>>,
    counts: HashMap<i32, i32>,
}

struct SegmentTree {
    root: Option<Box<SegmentTreeNode>>,
}


impl SegmentTreeNode {
    fn new(left_index: i32, right_index: i32, arr: &Vec<i32>) -> Self {
        SegmentTreeNode {
            left_index,
            right_index,
            left: None,
            right: None,
            counts: Self::get_counts(&arr),
        }
    }

    fn get_counts(arr: &Vec<i32>) -> HashMap<i32, i32> {
        let mut counts = HashMap::new();
        for a in arr {
            *counts.entry(*a).or_insert(0) += 1;
        }
        counts
    }
}


impl SegmentTree {
    fn new(arr: Vec<i32>) -> Self {
        SegmentTree {
            root: Self::build_tree(0, (arr.len()-1) as i32, &arr),
        }
    }

    fn build_tree(left_index: i32, right_index: i32, arr: &Vec<i32>) -> Option<Box<SegmentTreeNode>> {
        let mut root = Some(Box::new(SegmentTreeNode::new(left_index, right_index, arr[left_index..right_index+1])));
        if left_index == right_index {
            return root;
        }

        let mid = (left_index + right_index) / 2;
        root.as_mut().unwrap().left = Self::build_tree(left_index, mid, arr);
        root.as_mut().unwrap().right = Self::build_tree(mid+1, right_index, arr);
        root
    }

    fn traverse(&self) {
        let mut q = Vec::new();
        q.push(&self.root);

        while q.len() != 0 {
            let node = q.pop();
            if node.is_some() {
                println!("{:?}", node);
                let left_node = node.unwrap().as_ref().unwrap().left;
                if left_node.is_some() {
                    q.push(&left_node);
                }
                let right_node = node.unwrap().as_ref().unwrap().right;
                if right_node.is_some() {
                    q.push(&right_node);
                }
            }
        }
    }

}

struct RangeFreqQuery {
    arr: Vec<i32>
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        RangeFreqQuery {
            arr,
        }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let mut c = 0;
        let mut i = left;

        while i <= right {
            if self.arr[i as usize] == value {
                c += 1;
            }
            i += 1;
        }

        c
    }
}