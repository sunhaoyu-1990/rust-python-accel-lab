fn first_segment(path: &str) -> Option<&str> {
    path.split('|').next()
}

fn last_segment(path: &str) -> Option<&str> {
    path.split('|').last()
}

fn segment_count(path: &str) -> usize {
    if path.is_empty() {
        0
    } else {
        path.split('|').count()
    }
}

fn reverse_path(path: &str) -> String {
    let mut parts: Vec<&str> = path.split("|").collect();
    parts.reverse();
    parts.join("|")
}

fn contains_segment(path: &str, target: &str) -> bool {
    path.split('|').any(|seg| seg == target)
}

fn main() {
    let path = "G003061001000110|G003061001000210|G003061001000310";
    let target = "G003061001000210";

    println!("path = {}", path);
    println!("count = {}", segment_count(path));

    match first_segment(path) {
        Some(seg) => println!("first = {}", seg),
        None => println!("first = None"),
    }

    match last_segment(path) {
        Some(seg) => println!("last = {}", seg),
        None => println!("last = None"),
    }

    println!("reverse = {}", reverse_path(path));

    println!("if path have {} is {}", target, contains_segment(path, target));
}