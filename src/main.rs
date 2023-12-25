use std::io::{self, Read};

fn main() {
    let emojis = "🍰🎂🎉🎅🛷🎄🍗🎁🍾🔔".chars().collect::<Vec<char>>();
    let ends = ['。', '!', '?', '！', '？'];

    let create_emoji = || {
        let i = fastrand::usize(..emojis.len());
        emojis[i]
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input = input
        .chars()
        .map(|c| if ends.contains(&c) { create_emoji() } else { c })
        .collect::<String>();

    println!("{}", input);
}
