pub fn is_palindrome(n: i64) -> bool {
    let mut num = n;
    let mut reverse: i64 = 0;
    while num != 0 {
        let remainder = num % 10;
        reverse = reverse * 10 + remainder;
        num /= 10;
    }
    n == reverse
}
