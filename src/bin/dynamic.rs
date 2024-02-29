use std::{cmp, vec};

fn main() {
    let capa = 9;
    let weight = vec![2, 1, 3, 2, 1, 5];
    let values = vec![3, 2, 6, 1, 3, 85];
    let ans = knapsack(&capa, &weight, &values);

    println!("{}", ans);
}

fn knapsack(capa: &usize, weight: &[usize], values: &[usize]) -> usize {
    let n = weight.len();
    

    let mut dp = vec![vec![0; capa + 1]; n + 1]; 

    for i in 0..=n {
        for j in 0..=*capa {
            if i == 0 || j == 0 {
                dp[i][j] = 0;
            } else if weight[i - 1] <= j {
                dp[i][j] = cmp::max(values[i - 1] + dp[i - 1][j - weight[i - 1]], dp[i - 1][j]);
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    dp[n][*capa]
}