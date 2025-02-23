use std::fmt::Debug;
// mod linked_list;
mod task_project;
mod linked_list;
mod range_frequency;
mod trie;
mod configuration;
mod algorithms;
mod schema;

mod data_structures;

mod entities;

use task_project::task_manager::TaskManager;
use task_project::task_storage::TaskStorage;
use crate::data_structures::segment_tree::SumSegTree;
use crate::algorithms::sort_strings;
use crate::linked_list::LeetCode;
// fn insert_at_linked_list_head(mut ll: linked_list::LinkedList) -> linked_list::LinkedList {
//
//     println!("enter the integer to be inserted into the linked list");
//
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("failed to read line");
//     let input: i16 = input.trim().parse().expect("failed to convert value to integer");
//
//     ll.insert_at_head(input);
//     ll
// }
//
// fn insert_at_index(mut ll: linked_list::LinkedList) -> linked_list::LinkedList {
//
//     println!("enter the integer to insert into the linked list");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("failed to read line");
//     let val: i16 = input.trim().parse().expect("failed to convert value to integer");
//
//     input = String::new();
//     println!("enter index to insert the item:");
//     io::stdin().read_line(&mut input).expect("failed to read line");
//     let index: i16 = input.trim().parse().expect("failed to convert value to integer");
//
//     let res = ll.insert_at(index, val);
//     if res.is_err() {
//         println!("failed to insert into linked list: {}", res.err().unwrap());
//     }
//
//     ll
// }
//
// fn start_linked_list_program() {
//
//     let mut ll = linked_list::LinkedList::new();
//
//     loop {
//         println!("Select one of the following options: ");
//         println!("1. insert at head");
//         println!("2. insert anywhere");
//         println!("enter X to exit");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("failed to read line");
//         if input.trim_end_matches("\n").trim() == "X" {
//             break;
//         }
//
//         let input: i16 = input.trim().parse().expect("failed to convert value to integer");
//
//         match input {
//             1 => ll = insert_at_linked_list_head(ll),
//             2 => ll = insert_at_index(ll),
//
//             _ => {}
//         }
//     }
//
//     println!("\n\n printing items in the linked list:");
//     ll.print();
// }

fn main() {
    // let task_manager = new_task_manager();
    // let res = task_manager.add_task(String::from("test task"), chrono::Utc::now());
    // match res {
    //     Ok(value) => println!("\ntask added successfully. {:?}", value.id),
    //     Err(error) => println!("error occurred with task: {:?}", error)
    // }
    // let mut ll = LinkedList::new();
    // ll.insert_at_head(50);
    // ll.insert_at_head(109);
    // ll.insert_at_head(20);
    // ll.insert_at_head(10);
    // ll.insert_at_head(5);
    // ll.insert_at_head(50);
    //
    // LeetCode::sort_list(ll.head);

    // let mut t = Trie::new();
    // t.insert(&"bharath".to_string());
    // if t.search(&"bharath".to_string()) {
    //     println!("bharath is a found");
    // } else {
    //     println!("bharath is not found");
    // }

    // println!("{}", LeetCode::search(vec![1,3], 3))

    // println!("{:?}", sort_strings(vec!["bharath".to_string(), "aneesa".to_string()]))


    let mut tree = SumSegTree::new(vec![1, 3, 5]);
    println!("{:#?}", tree.sum_range(0, 2));
    tree.update(1, 2);
    println!("{:#?}", tree.sum_range(0, 2));

}