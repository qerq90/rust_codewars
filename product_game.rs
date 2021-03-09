fn check_for_product(x: i32, m: i32) -> (i32, Option<i32>) {
    for i in 1..=m {
        let sum_without_xi: i64 = ((1 + m as i64) * m as i64 / 2) - i as i64 - x as i64;
        if sum_without_xi == i as i64 * x as i64 { return (x, Some(i)) }
    }

    (x, None)
}

fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    (1..=m)
        .map(|x| check_for_product(x, m))
        .filter(|x| match x { (_, None) => false, _ => true })
        .map(|(a, x)| (a, x.unwrap()))
        .collect()
}


fn main() {
    println!("{:?}", remove_nb(5000));
}
