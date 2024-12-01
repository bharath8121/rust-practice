#[cfg(test)]
mod test {
    use crate::task_project::task::Task;
    use crate::task_project::task_storage::new_json_task_storage;

    #[test]
    fn test_task_storage_create() {
        let task_storage = new_json_task_storage("/tmp/sample.json".to_string());
        let task = Task::new("1".to_string(), "hello world".to_string());
        let res = task_storage.create_task(&task);
        println!("{:#?}", res);
        assert!(res.is_ok());
    }

    #[test]
    fn test_task_storage_read() {
        let task_storage = new_json_task_storage("/tmp/sample.json".to_string());
        let res = task_storage.get_task_for_id("1".to_string());
        assert!(res.is_ok());
        println!("{:#?}", res.unwrap());
    }

    #[test]
    fn test_task_storage_update() {
        let task_storage = new_json_task_storage("/tmp/sample.json".to_string());
        let res = task_storage.get_task_for_id("1".to_string());
        assert!(res.is_ok());
        let mut task = res.unwrap().clone();
        task.description = "description 1".to_string();
        let res = task_storage.update_task(&task);
        println!("{:#?}", res);
        assert!(res.is_ok());
    }

    // delete has bugs
    #[test]
    fn test_task_storage_delete() {
        let task_storage = new_json_task_storage("/tmp/sample.json".to_string());
        let res = task_storage.delete_task("1".to_string());
        assert!(res.is_ok());
    }


}