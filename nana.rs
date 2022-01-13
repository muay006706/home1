fn is_palindrome(string: &str) -> bool {
    let half_len = string.len() / 2;
    string
        .chars()
        .take(half_len)
        .eq(string.chars().rev().take(half_len))
}
fn main() {
    print!("Input string: "); 
    let mut string: String = String::new();
    std::io::stdin().read_line(&mut string);
    let mut last = string.len() - 1;
    let mut first = 0;
    let mut myvec =string.as_bytes();
   
    while first < last {
        if myvec[first] == myvec[last]{
            println!("Given string is not Palindrome"); 
        }
            first +=1;
            last -=1;
    {
        println!("Given string is Palindrome"); 
    }


    }
}