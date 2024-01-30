// 再帰関数を用いたユーグリッド
fn main() {
    let ans = gcd(128, 512);

    println!("{}", ans);
}

fn gcd(m: i32, n: i32) -> i32 {
    let res = if n == 0 {
        m
    } else {
        gcd(n, m % n)
    };

    res
}