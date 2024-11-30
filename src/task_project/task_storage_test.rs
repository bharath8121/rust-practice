#[cfg(test)]
mod test {
    use crate::task_project::task_storage::new_json_task_storage;

    #[test]
    fn test_task_storage_read() {
        let task_storage = new_json_task_storage("/tmp/sample.json".to_string());
        let res = task_storage.get_task_for_id("1".to_string());
        assert!(res.is_ok());
        println!("{:#?}", res.unwrap());
    }

}