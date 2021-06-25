pub fn loop_sum(max: usize) -> usize {
    let mut ans = 0;

    for i in 0..max {
        ans += i
    }
    ans
}
