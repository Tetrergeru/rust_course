fn main() {
    let mut string = "foo".to_string();
    string.push(')');
    println!("string = {string}");
}

fn foo(string: &mut String) {
    string.push(')');
}