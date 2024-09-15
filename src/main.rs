use rand::Rng;
use rand::seq::IndexedRandom;
use rand::rngs::ThreadRng;

const PATTERNS: [&str; 5] = ["vcv", "cvvc", "cv", "vc", "ccv"];
const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'y'];
const CONSONANTS: &[char] = &['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];

fn ran_vow(rng: &mut ThreadRng) -> char
{
    VOWELS[rng.gen_range(0..VOWELS.len())]
}

fn ran_con(rng: &mut ThreadRng) -> char
{
    CONSONANTS[rng.gen_range(0..CONSONANTS.len())]
}

fn parse_pattern(rng: &mut ThreadRng) -> String
{
    let pattern: String = PATTERNS.choose(rng).unwrap().to_string();
    if pattern.len() == 0
    {
        panic!("Empty pattern!")
    }
    let mut ret_val: String = "".to_string();
    for c in pattern.chars()
    {
        if c == 'v'
        {
            ret_val.push(ran_vow(rng));
        }
        else if c == 'c'
        {
            ret_val.push(ran_con(rng));
        }
    }
    ret_val
}

fn random_word(rng: &mut ThreadRng) -> String
{
    let mut ret_val: String = "".to_string();
    for _i in 1..3
    {
        let substr: String = parse_pattern(rng);
        ret_val += substr.as_str();
    }
    ret_val
}

fn init_sources(source: &mut Vec<String>, rng: &mut ThreadRng)
{
    for _i in 0..10
    {
        source.push(random_word(rng))
    }
}

fn mutate_word(str: &mut String, rng: &mut ThreadRng) -> String {
    let mut old_str: String = str.to_string();
    let mut new_str: String = old_str.clone();
    while new_str.eq(&old_str)
    {
        new_str = "".to_string();
        for c in str.chars()
        {
            if rng.gen_range(0..8) != 0
            {
                new_str.push(c);
                continue
            }
            if CONSONANTS.contains(&c)
            {
                new_str.push(ran_con(rng));
            }
            else
            {
                new_str.push(ran_vow(rng));
            }
        }
    }
    new_str
}

fn mutate_sources(sources: &mut Vec<String>, rng: &mut ThreadRng) {
    for str in sources
    {
        println!("{}", str);
        for _i in 0..10
        {
            println!("-> {}", mutate_word(str, rng))
        }
    }
}

fn main()
{
    let mut rng = rand::thread_rng();
    let mut source: Vec<String> = Vec::new();
    init_sources(&mut source, &mut rng);
    mutate_sources(&mut source, &mut rng);
}
