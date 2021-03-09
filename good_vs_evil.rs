fn good_vs_evil(good: &str, evil: &str) -> String {
    let good_team_score: u32 = good
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .enumerate()
        .map(|(i, x)| {
            match i 
            {
                0 => x,
                1 => x * 2,
                2 => x * 3,
                3 => x * 3,
                4 => x * 4,
                5 => x * 10,
                _ => unreachable!()
            }
        })
        .sum();
    
    let evil_team_score: u32 = evil
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .enumerate()
        .map(|(i, x)| {
            match i 
            {
                0 => x,
                1 => x * 2,
                2 => x * 2,
                3 => x * 2,
                4 => x * 3,
                5 => x * 5,
                6 => x * 10,
                _ => unreachable!()
            }
        })
        .sum();

    if good_team_score > evil_team_score {String::from("Battle Result: Good triumphs over Evil")}
    else if good_team_score == evil_team_score {String::from("Battle Result: No victor on this battle field")}
    else {String::from("Battle Result: Evil eradicates all trace of Good")}
}

fn main() {
    println!("{:?}", good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"));
}
