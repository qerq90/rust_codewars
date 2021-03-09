fn josephus_survivor_non_recursive(n: i32, k: i32) -> i32 {
    (2..=n).fold(0, |acc, x| (acc + k) % x) + 1
}

fn josephus_survivor(n: i32, k: i32) -> i32 {
    if n == 1 { return 1 }
    (josephus_survivor(n - 1, k) + k - 1).rem_euclid(n) + 1
}


fn main() {
    println!("{:?}", josephus_survivor(20, 30));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(josephus_survivor(7, 3), 4);
        assert_eq!(josephus_survivor(11, 19), 10);
        assert_eq!(josephus_survivor(40, 3), 28);
        assert_eq!(josephus_survivor(14, 2), 13);
        assert_eq!(josephus_survivor(100, 1), 100);
        assert_eq!(josephus_survivor(1, 300), 1);
        assert_eq!(josephus_survivor(2, 300), 1);
        assert_eq!(josephus_survivor(5, 300), 1);
        assert_eq!(josephus_survivor(7, 300), 7);
        assert_eq!(josephus_survivor(300, 300), 265);
    }

    #[test]
    fn basic_tests_for_non_recursive() {
        assert_eq!(josephus_survivor_non_recursive(7, 3), 4);
        assert_eq!(josephus_survivor_non_recursive(11, 19), 10);
        assert_eq!(josephus_survivor_non_recursive(40, 3), 28);
        assert_eq!(josephus_survivor_non_recursive(14, 2), 13);
        assert_eq!(josephus_survivor_non_recursive(100, 1), 100);
        assert_eq!(josephus_survivor_non_recursive(1, 300), 1);
        assert_eq!(josephus_survivor_non_recursive(2, 300), 1);
        assert_eq!(josephus_survivor_non_recursive(5, 300), 1);
        assert_eq!(josephus_survivor_non_recursive(7, 300), 7);
        assert_eq!(josephus_survivor_non_recursive(300, 300), 265);
    }
}