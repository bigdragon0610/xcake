fn main() {
    let emojis = "🍰🎂🎉🎅🛷🎄🍗🎁🍾🔔".chars().collect::<Vec<char>>();
    let i = fastrand::usize(..emojis.len());
    println!("{}", emojis[i]);
}
