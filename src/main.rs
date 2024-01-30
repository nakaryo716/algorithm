use proconio::input;

mod test;

fn main() {
    let mut cost = Vec::new();
    input! {
        data_amount: usize,
    }

    for _i in 0..data_amount {
        input! {
            value: usize,
        }
        cost.push(value);
    }

    let mut dp: Vec<usize> = vec![0];

    for i in 1..cost.len() {
        if i == 1 {
            let delta = (cost[i] as i32 - cost[i - 1] as i32).abs();
            let total_cost = dp[i - 1] + delta as usize;
            dp.push(total_cost);
        } else {
            let delta = (cost[i] as i32 - cost[i - 1] as i32).abs();
            let delta2 = (cost[i] as i32 - cost[i - 2] as i32).abs();

            let total_cost = dp[i - 1] + delta as usize;
            let total_cost2 = dp[i - 1] + delta2 as usize;
            
            if total_cost > total_cost2 {
                dp.push(total_cost2);
            } else {
                dp.push(total_cost);
            }
        }
    }

    println!("answer is: {}", dp[dp.len() - 1]);
}
