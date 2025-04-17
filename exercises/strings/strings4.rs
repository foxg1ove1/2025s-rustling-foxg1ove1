// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

/*
* String 是可修改、拥有所有权的字符串类型，适用于需要动态处理字符串的场景。
* &str 是不可变的字符串切片，是对字符串数据的引用，适用于不需要修改字符串且只需要临时访问的场景。
*/
fn main() {
    string_slice("blue");
    string("red".to_string());                  // why to_string()是生成了一个String类型
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);   //  form、trim()都是&str的切片部分
    string_slice("  hello there ".trim());      // 
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
