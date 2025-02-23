#[derive(Debug, Clone)]
struct SumSegTreeNode {
    start: i32,
    end: i32,
    s: i32,
    left: Option<Box<SumSegTreeNode>>,
    right: Option<Box<SumSegTreeNode>>,
}

#[derive(Debug, Clone)]
pub struct SumSegTree {
    root: Option<Box<SumSegTreeNode>>,
}


impl SumSegTreeNode {
    fn new(nums: &Vec<i32>, start: i32, end: i32) -> Self {
        if start == end {
             return SumSegTreeNode { start, end, s: nums[start as usize], left: None, right: None };
        }

        let mut curr_node = SumSegTreeNode { start, end, s: 0, left: None, right: None };
        let mid = (start + end) / 2;
        let left = Box::from(SumSegTreeNode::new(nums, start, mid));
        let right = Box::from(SumSegTreeNode::new(nums, mid+1, end));

        curr_node.s = left.s + right.s;
        curr_node.left = Some(left);
        curr_node.right = Some(right);
        curr_node
    }

    fn update_node(&mut self, index: i32, val: i32) {
        if self.start == index && self.end == index {
            self.s = val;
        }

        if let Some(mut left) = self.left.as_mut() {
            if left.start <= index && index <= left.end {
                self.s -= left.s;
                left.update_node(index, val);
                self.s += left.s;
            }
        }

        if let Some(mut right) = self.right.as_mut() {
            if right.start <= index && index <= right.end {
                self.s -= right.s;
                right.update_node(index, val);
                self.s += right.s;
            }
        }
    }


    fn overlap(&self, curr_left: i32, curr_right: i32, left: i32, right: i32) -> bool {
        !((curr_left < left && curr_right < left) || (curr_left > right && curr_right > right))
    }


    fn get_sum(&self, left: i32, right: i32) -> i32 {
        if self.start >= left && self.end <= right {
            return self.s;
        }

        let mut sum = 0;

        if let Some(l) = self.left.as_ref() {
            if self.overlap(l.start, l.end, left, right) {
                sum += l.get_sum(left, right);
            }
        }

        if let Some(r) = self.right.as_ref() {
            if self.overlap(r.start, r.end, left, right) {
                sum += r.get_sum(left, right);
            }
        }

        sum
    }
}


impl SumSegTree {

    pub fn update(&mut self, index: i32, val: i32) {
        if let Some(root) = self.root.as_mut() {
            root.update_node(index, val);
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        if let Some(root) = self.root.as_ref() {
            return root.get_sum(left, right);
        }

        0
    }

    pub fn new(nums: Vec<i32>) -> Self {
        if nums.len() == 0 {
            return SumSegTree { root: None };
        }

        SumSegTree {
            root: Some(Box::new(SumSegTreeNode::new(&nums, 0, (nums.len()-1) as i32))),
        }
    }
}



