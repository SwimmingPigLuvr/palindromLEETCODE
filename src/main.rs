fn palindrome(x: i32) -> bool{
    //string it
    x.to_string()==x.to_string().chars().rev().collect::<String>()
}

fn main() {
    println!("{:?}", palindrome(696));
    println!("{:?}", palindrome(123321));
    println!("{:?}", palindrome(69696));
    println!("{:?}", palindrome(42024));
    println!("{:?}", palindrome(110));
}
