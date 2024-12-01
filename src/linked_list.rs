use std::cell::RefCell;
use std::io::Error;

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

        while l.is_some() {
            temp.as_mut().unwrap().next = Some(Box::new(LinkedListNode{val: l.as_ref().unwrap().val, next: None }));
            l = &l.as_ref().unwrap().next;
            temp = &mut temp.as_mut().unwrap().next;
        }

        while r.is_some() {
            temp.as_mut().unwrap().next = Some(Box::new(LinkedListNode{val: r.as_ref().unwrap().val, next: None }));
            r = &r.as_ref().unwrap().next;
            temp = &mut temp.as_mut().unwrap().next;
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

        println!("*** length: {}", l);
        let h = Self::merge_sort(&head, 0, l-1);
        let mut temp = &h;
        while temp.is_some() {
            println!("{}", temp.as_ref().unwrap().val);
            temp = &temp.as_ref().unwrap().next;
        }
        h
    }

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

