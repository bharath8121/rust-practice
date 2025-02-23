pub fn sort_strings(mut arr: Vec<String>) -> Vec<String> {
    arr.sort();
    arr
}


pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = (nums.len() - 1) as i32;

    while start < end {
        let mid = start + (end - start) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        if nums[mid as usize] < target {
            start = mid + 1;
        } else if nums[mid as usize] > target {
            end = mid - 1;
        }
    }

    -1
}


pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    todo!()
}


pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
    todo!()
}


pub fn min_length(s: String) -> i32 {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if stack.is_empty() {
            stack.push(c);
            continue;
        }

        if stack[stack.len() - 1] == 'A' && c == 'B' {
            stack.pop();
        } else if stack[stack.len() - 1] == 'C' && c == 'D' {
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    stack.len() as i32
}