fn main() {
    let mut doc = clean_d(String::from("   # Q2 Financial Report. Status: Final. Author: Zubair.   "));
    println!("{doc}");
    if has(&doc){
        println!("# is the 1st letter !");
    }else{ 
       println!("# is not 1st letter !"); 
    }
    prepend(&mut doc);
    println!("{doc}");
    if let Some(auth) = search(&doc){
        println!("{auth}");
    }
    
}
fn clean_d(mut s:String) -> String{ 
    s = s.trim().to_string();
    s
    
}
fn has(s:&str)-> bool{
    s.starts_with("#")
}
fn prepend(s:&mut String){
    s.insert_str(0, "[CONFIDENTIAL] ");
}
fn search(mut s:&str)->Option<&str>{
    let target = "Author:";
    let t_l = target.len();
    if let Some(f_i) = s.find("Author:"){
        s = &s[f_i+t_l+1..];

        if let Some(l_i) = s.find("."){
            return Some(&s[..l_i]);
        }else {
            return Some(s);
        }
    }
    None
}