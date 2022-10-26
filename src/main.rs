fn palindrome(num: i32) -> bool{
    //string it
    let n = format!("{:?}", num);
    // reverse
    let rev = n.chars().rev().collect::<String>();
    match n {
        x if x == rev => {
            true
        }
        _ => {false}
    }
}

fn main() {
    println!("{:?}", palindrome(696));
    println!("{:?}", palindrome(69));
}
