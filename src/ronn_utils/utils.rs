pub fn prettify_int(int: f64) -> String {
    let mut s = String::new();
    let int_str = int.to_string();
    let a = int_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ' ')
        }
        s.insert(0, val)
    }
    s
}
