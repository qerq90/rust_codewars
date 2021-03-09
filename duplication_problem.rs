#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name { Sheldon, Leonard, Penny, Rajesh, Howard }

/// Just to make code look a bit nicer
type Names = Vec<Name>;

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next(names: &Names, mut n: usize) -> Name {
    if n < names.len() { return *names.get(n-1 ).unwrap() }

    let mut ln = names.len();
    let mut counter = 0;
    while ln < n {
        n -= ln;
        ln *= 2;
        counter += 1;
    }

    *names.get(n / (2usize.pow(counter))).unwrap()
}

fn main() {
    println!("{:?}", who_is_next(&vec![Name::Sheldon, Name::Leonard, Name::Penny, Name::Rajesh, Name::Howard], 7_230_702_951));
}