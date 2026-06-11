use std::collections::HashSet;

fn last_n_segments(path: &str, n: usize) -> Vec<String> {
    if path.is_empty() {
        return Vec::new();
    }
    let path_list: Vec<&str> = path.split("|").collect();
    let len = path_list.len();

    let start = if n >= len {
        0
    } else {
       len - n 
    };

    path_list[start..]
        .iter()
        .map(|seg| seg.to_string())
        .collect()

}

fn first_n_segments(path: &str, n: usize) -> Vec<String> {
    if path.is_empty() {
        return Vec::new();
    }

    let path_list: Vec<&str> = path.split("|").collect();
    let len = path_list.len();

    let end = if n >= len {
        len - 1
    } else {
        n 
    };

    path_list[..end]
        .iter()
        .map(|seg| seg.to_string())
        .collect()
}

fn remove_duplicate_keep_order(path: &str) -> String {
    if path.is_empty() {
        return String::new();
    }

    let mut seen = HashSet::new();
    let mut result: Vec<&str> = Vec::new();

    for seg in path.split("|") {
        if !seen.contains(seg) {
            seen.insert(seg);
            result.push(seg);
        }
    }

    result.join("|")
}

fn filter_by_suffix(path: &str, suffix: &str) -> Vec<String> {
    // 你来实现
}

fn main() {
    let path = "G003061001000110|G003061001000210|G003061001000310";
    let n = 2;

    println!("end {}: {:?}", n, last_n_segments(path, n));
    println!("firt {}: {:?}", n, first_n_segments(path, n));

    let path_de = "G003061001000110|G003061001000210|G003061001000210|G003061001000110|G003061001000310";

    println!("de : {}", remove_duplicate_keep_order(path_de));
}