fn main() {
    let mut raw_description = clean_string(String::from(
        "  Smartphone 128GB. Condition: New. Category: Electronics.   ",
    ));
    println!("{raw_description}");
    let len = calculate_length(&raw_description);
    println!("{len}");
    append_watermark(&mut raw_description);
    println!("{raw_description}");
    if let Some(cat) = catogary(&raw_description) {
        println!("{cat}");
    }
}
fn clean_string(mut s: String) -> String {
    s = s.trim().to_string();
    s
}
fn calculate_length(s: &str) -> usize {
    s.len()
}
fn append_watermark(s: &mut String) {
    &s.push_str(" [VARIFIED]");
}
fn catogary(s: &str) -> Option<&str> {
    if let Some(f_i) = s.find("Category:") {
        let t_s = &s[f_i..];
        if let Some(l_i) = t_s.find(".") {
            return Some(&t_s[..=l_i]);
        } else {
            return Some(t_s);
        }
    }
    None
}

