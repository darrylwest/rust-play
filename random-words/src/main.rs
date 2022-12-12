/// generate a list of random passphrase words
use std::env;

const BIP39_WORDS: &str = include_str!("../assets/bip39-english.txt");

fn main() {
    // command line arg: --seed
    if env::args().len() == 5 {
        fastrand::seed(8766);
    }

    // command line arg: --size
    let size = 12_usize;

    // command line arg: --count
    let count = 20_usize;

    let rng = fastrand::Rng::new();
    (1..=count).for_each(|x| {
        let idx_list = generate_idx(&rng, size);
        println!("{} {:?}", x, &idx_list);
        let words = get_words(idx_list);

        println!("{} {}", x, words.join("-"));
    });
}

fn generate_idx(rng: &fastrand::Rng, len: usize) -> Vec<usize> {
    (0..len).map(|_| rng.usize(..2048) ).collect()
}

fn get_words(nlist: Vec<usize>) -> Vec<String> {
    nlist.iter().map(|n| BIP39_WORDS.lines().nth(*n).unwrap().to_string() ).collect()
}

