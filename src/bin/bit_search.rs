// bit全探索

fn main() {
    let amount = 4;
    let buf: Vec<u32> = vec![41, 10, 20, 3];
    let target = 23;
    let mut flag = false;

    for bit in 0..(1 << amount) {
        let mut sum = 0;

        for i in 0..amount {
            if bit & (1 << i) > 0 {
                sum += &buf[i];
            }
        }
        if target == sum {
            flag = true;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}

/*
    10000   => 16

    0   0000
    1   0001
    2   0010
    3   0011
    4   0100
    5   0101
    6   0110
    7   0111
    8   1000
    .   .
    .   .
    .   .
    15  1111
*/