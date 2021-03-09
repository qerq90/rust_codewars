use std::collections::HashMap;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut needed_values: HashMap<i8, i8> = HashMap::new();
    for value in ints.iter() {
        if let Some(x) = needed_values.get(value) {
            return Some((*x, *value))
        }

        let need_to_find = s - value;
        needed_values.insert(need_to_find, *value);
    }

    return None
}

fn main() {
    println!("{:?}", sum_pairs(vec!(10, 5, 2, 3, 7, 5).as_slice(), 10));
}
