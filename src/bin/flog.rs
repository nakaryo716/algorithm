use proconio::input;

fn main() {
    let mut buf = Vec::new();
    let mut pool = vec![0];

    input! {
        data_amount: u32,
    }

    for _i in 0..data_amount {
        input! {
            number: u32,
        }
        buf.push(number);
    }

    for i in 1..buf.len() {
        if i == 1 {
            pool.push(buf[i] - buf[i - 1])
        } else {
            let cost1: i32 = (buf[i] as i32  - buf[i - 1] as i32).abs();
            let total_cost1 = pool[i - 1] + cost1 as u32;

            let cost2: i32 = (buf[i] as i32 - buf[i - 2] as i32).abs();
            let total_cost2 = pool[i - 2] + cost2 as u32;

            if total_cost1 > total_cost2 {
                pool.push(total_cost2);
            } else if total_cost1 < total_cost2 { pool.push(total_cost1);
            }
        }
    }
    println!("{:?}", buf);
    println!("{:?}", pool);
}