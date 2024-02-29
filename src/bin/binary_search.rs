// 二分探索

// 二分探索するときはleft, rightを活用するのがいいのかも


fn main() {
    // 配列
    let buf = vec![3, 5, 8, 10, 14, 17, 21, 39];
    // 目標値
    let target = 14;

    let response = binary_search(&buf, target);

    println!("{:?}", response)
}

// 目標値があったらSome(i), なかったらNoneを返す
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = (arr.len() - 1) as i32;
    let mut flag = None;

    // 条件式は2分割された配列の個数によって判別
    while left <= right {
        let mid = ((right + left) / 2) as usize;
        
        if arr[mid] == target {
            flag = Some(mid);
            break;
        } else if arr[mid] > target {
            // left, rightを利用して，2分割を行う
            right = (mid - 1) as i32;
        } else if arr[mid] < target {
            left = (mid + 1) as i32;
        }
    };

    flag
}