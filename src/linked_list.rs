use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Error;
use std::rc::Rc;

pub struct LinkedList {
    pub head: Option<Box<LinkedListNode>>,
}

#[derive(Debug, Clone)]
pub struct LinkedListNode {
    val: i32,
    next: Option<Box<LinkedListNode>>,
}

impl LinkedList {

    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub fn print(&self) {
        if self.head.is_none() {
            print!("no items to print");
        }

        let mut curr_node = &self.head;
        while curr_node.is_some() {
            println!("{}", curr_node.as_ref().unwrap().val);
            curr_node = &curr_node.as_ref().unwrap().next;
        }
    }

    pub fn delete_at_head(&mut self) -> Result<bool, String> {
        if let Some(mut node) = self.head.take() {
            self.head = node.next.take();
            return Ok(true)
        }
        Err(String::from("no head found"))
    }

    // pub fn delete_at(&mut self, index: i16) -> Result<bool, String> {
    //     let mut curr_index = 0;
    //     let mut curr_node = &self.head;
    //     while curr_index < index-1 {
    //         if let Some(mut node) = curr_node {
    //             curr_node = &mut node.next;
    //         } else {
    //             return Err(String::from("out of bounds"));
    //         }
    //         curr_index += 1;
    //     }
    //
    //     if let Some(mut node) = curr_node {
    //         curr_node = &node.next.take();
    //     }
    //
    //     Ok(true)
    // }

    pub fn insert_at(&mut self, index: i16, val: i32) -> Result<bool, String> {
        if self.head.is_none() {
           return Err(String::from("empty linked list. please insert at head first"));
        }

        let mut curr_node = &mut self.head;
        let mut curr_index = 0;
        while curr_index < index {
            curr_index += 1;
            if curr_node.is_none() {
                return Err(String::from(format!("index out of bound. length of linked list: {}", curr_index-1)));
            }
            curr_node = &mut curr_node.as_mut().unwrap().next;
        }

        let new_node = Box::new(LinkedListNode {val, next: curr_node.as_mut().unwrap().next.take() });
        curr_node.as_mut().unwrap().next = Some(new_node);

        Ok(true)
    }

    pub fn insert_at_head(&mut self, val: i32) {
        let curr_node = LinkedListNode{val, next: self.head.take()};
        self.head = Some(Box::new(curr_node));
    }

}

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode{
            val,
            left: None,
            right: None,
        }
    }
}

pub struct LeetCode {
}


impl LeetCode {


    pub fn merge(left: Option<Box<LinkedListNode>>, right: Option<Box<LinkedListNode>>) -> Option<Box<LinkedListNode>> {
        let mut l = &left;
        let mut r = &right;
        let mut result = Some(Box::new(LinkedListNode{val: 0, next: None}));
        let mut temp = &mut result;
        while l.is_some() && r.is_some() {
            if l.as_ref().unwrap().val <= r.as_ref().unwrap().val {
                temp.as_mut().unwrap().next = Some(Box::new(LinkedListNode{val: l.as_ref().unwrap().val, next: None }));
                l = &l.as_ref().unwrap().next;
            } else {
                temp.as_mut().unwrap().next = Some(Box::new(LinkedListNode{val: r.as_ref().unwrap().val, next: None }));
                r = &r.as_ref().unwrap().next;
            }
            temp = &mut temp.as_mut().unwrap().next;
        }

        if l.is_some() {
            // temp.as_mut().unwrap().next = Some(Box::new(LinkedListNode { val: l.as_ref().unwrap().val, next: None }));
            // l = &l.as_ref().unwrap().next;
            // temp = &mut temp.as_mut().unwrap().next;
            temp.as_mut().unwrap().next = l.clone();
        }

        if r.is_some() {
            // temp.as_mut().unwrap().next = Some(Box::new(LinkedListNode{val: r.as_ref().unwrap().val, next: None }));
            // r = &r.as_ref().unwrap().next;
            // temp = &mut temp.as_mut().unwrap().next;
            temp.as_mut().unwrap().next = r.clone();
        }

        result.unwrap().next
    }

    fn get_at(head: &Option<Box<LinkedListNode>>, index: i32) -> Option<Box<LinkedListNode>> {
        let mut i = 0;
        let mut temp  = head;
        while i < index {
            if temp.is_some() {
                temp = &temp.as_ref().unwrap().next;
            } else {
                return None
            }
            i += 1;
        }
        Some(Box::new(LinkedListNode{val: temp.as_ref().unwrap().val, next: None}))
    }

    pub fn merge_sort(head: &Option<Box<LinkedListNode>>, start: i32, end: i32) -> Option<Box<LinkedListNode>> {
        if start == end {
            return Self::get_at(head, start);
        }

        let mid = (start + end) / 2;
        let left = Self::merge_sort(head, start, mid);
        let right = Self::merge_sort(head, mid+1, end);
        Self::merge(left, right)
    }

    pub fn sort_list(head: Option<Box<LinkedListNode>>) -> Option<Box<LinkedListNode>> {
        let mut temp = &head;
        let mut l = 0;
        while temp.is_some() {
            l += 1;
            temp = &temp.as_ref().unwrap().next;
        }

        if l == 0 {
            return head;
        }

        let h = Self::merge_sort(&head, 0, l-1);
        let mut temp = &h;
        while temp.is_some() {
            println!("{}", temp.as_ref().unwrap().val);
            temp = &temp.as_ref().unwrap().next;
        }
        h
    }

    fn is_palindrome(arr: &Vec<i32>) -> bool {
        let mut i = 0;
        let mut j = arr.len() - 1;
        if arr.len() % 2 != 0 {
            return false;
        }

        while i < j {
            if arr[i] != arr[j] {
                return false;
            }
            i+=1;
            j-=1;
        }

        true
    }

    pub fn is_symmetric(head: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        queue.push(head.clone());
        while !queue.is_empty() {
            let mut lot: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            let mut r: Vec<i32> = Vec::new();

            while !queue.is_empty() {
                let node = queue.pop().unwrap();
                if node.is_some() {
                    let mut bn = node.as_ref().unwrap().borrow_mut();
                    r.push(bn.val);
                    if let Some(left) = bn.left.take() {
                        lot.push(Some(left));
                    } else {
                        lot.push(None);
                    }

                    if let Some(right) = bn.right.take() {
                        lot.push(Some(right));
                    } else {
                        lot.push(None);
                    }
                } else {
                    r.push(-1000);
                }
            }

            if !Self::is_palindrome(&r) {
                return false;
            }

            // println!("{:?}", lot);
            queue.clone_from(&lot);
        }
        true
    }


    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        let mut iter = arr.iter();
        
        loop {
            if count == 3 {
                return true;
            }
            match iter.next() {
                Some(x) => {
                    if x%2 == 1 {
                        count+=1;
                    } else {
                        count = 0;
                    }
                },
                None => {
                    break;
                }
            }
            
        }
        false
    }


    // TODO: non-trie implementation
    // TODO: change to rust way using map
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let s = sentence.split(" ");
        let mut st = HashMap::new();
        let mut res: Vec<String> = Vec::new();
        for i in dictionary.iter() {
            st.insert(i.to_string(), true);
        }

        for item in s.into_iter() {
            let mut i = 1;
            let mut pushed = false;
            while i <= item.len() {
                let subitem = &item[0..i];
                if st.contains_key(subitem) {
                    res.push(subitem.to_string());
                    pushed = true;
                    break;
                }
                i += 1;
            }
            if !pushed {
                res.push(item.to_string());
            }
        }

        res.join(" ")
    }

    pub fn lies_between(left: usize, right: usize, target: i32, nums: &Vec<i32>) -> bool {
        if nums[left] <= nums[right] && (nums[left] <= target && target <= nums[right]) {
            println!("1. target: {} lies between {} and {}", target, nums[left], nums[right]);
            return true;
        }

        if nums[left] >= nums[right] && (nums[left] >= target || target <= nums[right]) {
            println!("2. target: {} lies between {} and {}", target, nums[left], nums[right]);
            return true;
        }

        println!("3. target: {} doesn't lie between {} and {}", target, nums[left], nums[right]);
        false
    }

    pub fn static_search(nums: &Vec<i32>, target: i32) -> i32 {
        println!("in static");
        if nums.len() == 2 {
            if nums[0] == target {
                return 0;
            }

            if nums[1] == target {
                return 1;
            }
        }

        if nums.len() == 3 {
            if nums[0] == target {
                return 0;
            }

            if nums[1] == target {
                return 1;
            }

            if nums[2] == target {
                return 2;
            }

        }

        -1
    }

    // TODO: not working for few inputs. needs fix in logic
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0i32;
        let mut right = (nums.len() - 1) as i32;
        let mut mid = (left + right) / 2;

        if right - left == 2 || right - left == 1 {
            return Self::static_search(&nums, target);
        }

        while left != mid {
            if nums[mid as usize] == target {
                return mid;
            }

            if right - left == 2 || right - left == 1 {
                return Self::static_search(&nums, target);
            }

            if Self::lies_between(left as usize, mid as usize, target.clone(), &nums) {
                println!("lies between {} and {}", nums[left as usize], nums[mid as usize]);
                right = mid;
                mid = (left + mid) / 2;
            } else {
                println!("lies between {} and {}", nums[(mid+1) as usize], nums[right as usize]);
                left = mid + 1;
                mid = (mid+1 + right) / 2;
            }
            println!("new left: {} mid: {}, right: {}", nums[left as usize], nums[mid as usize], nums[right as usize]);
        }

        if nums[left as usize] == target {
            return left;
        }

        -1
    }

    pub fn remove_duplicate_letters(s: String) -> String {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        todo!()
    }

    // pub fn license_key_formatting(s: String, k: i32) -> String {
    //     let mut ptr: i32 = s.len() as i32 - 1i32;
    //     let mut res = String::new();
    //     while ptr >= k {
    //         let mut i = 0;
    //         while i < k {
    //             let b = s.chars().nth(i as usize);
    //             res.push_str(s.chars().nth(i));
    //             i += 1;
    //         }
    //         res.push_str("-");
    //         ptr = ptr - k;
    //     }
    //     if ptr >= 0 {
    //
    //     }
    //     res.chars().rev().collect::<String>().to_uppercase()
    // }


    pub fn create_new_list() {
        let mut ll = LinkedList::new();
        ll.insert_at_head(50);
        ll.insert_at_head(109);
        ll.insert_at_head(20);
        ll.insert_at_head(10);
        ll.insert_at_head(5);
        ll.insert_at_head(50);

        Self::sort_list(ll.head);
    }
}

