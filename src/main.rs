use std::env;
use rand::prelude::*;


// Much easier to understand but insanely slower
// Really naive and careless implem
fn fitness_easy(obj: String, wrd: String) -> usize {
    // if they are equal the distance is null
    if obj == wrd {
        return 0;
    }
    let l1 = obj.len();
    let l2 = wrd.len();

    // if either one is empty, the distance is the length of the other
    if l1 == 0 {
        return l2;
    }
    if l2 == 0 {
        return l1;
    }

    // the distance of the "rest" of the string (excluding current both characters)
    let bc = fitness_easy(obj.chars().skip(1).collect::<String>(),
                          wrd.chars().skip(1).collect::<String>());

    // if current chars are the same, the distance of the string is the distance of the "rest" of the string
    if obj.as_bytes()[0] != wrd.as_bytes()[0] {
        let fc = fitness_easy(obj.chars().skip(1).collect::<String>(), wrd.clone());
        let sc = fitness_easy(obj, wrd.chars().skip(1).collect::<String>());

        // if the chars are different, at least 1 change, and then takes the shortest distance for the rest of the string
        // bc is changing  a letter (so both current char are equal, going to the rest)
        // fc is inserting a letter (insert right letter, so the distance is the rest of the guessing word (after the insertion))
        // sc is deleting  a letter (still on the same char for obj, but skip the "deleted" one in wrd)
        return 1 + if bc < fc && bc < sc { bc } else
                   if fc < sc && fc < bc { fc } else
                   { sc };
    }
    return bc;
}

// Just a tit more obscure but fast
// just trust me this one does the same as the one above
fn fitness_opti(obj: &str, wrd: &str) -> usize {
    let mut r = 0;

    if obj == wrd {
        return r;
    }

    let l1 = obj.chars().count();
    let l2 = wrd.chars().count();

    if l1 == 0 {
        return l2;
    }
    if l2 == 0 {
        return l1;
    }

    let mut c = (1..=l1).collect::<Vec<usize>>();
    let mut d1;
    let mut d2;

    for (i2, c2) in wrd.chars().enumerate() {
        r = i2;
        d1 = i2;

        for (i1, c1) in obj.chars().enumerate() {
            d2 = if c1 == c2 { d1 } else { d1 + 1 };

            d1 = c[i1];

            r = if d1 > r {
                if d2 > r { r + 1 } else { d2 }
            } else if d2 > d1 { d1 + 1 } else { d2 };
            c[i1] = r;
        }
    }
    r
}

fn mutation(s: &mut String) {
    let mut rng = thread_rng();
    // either remove a letter, add one, or change one (because the distance was calculated with those operations)
    if s.len() > 0 {
        match rng.gen_range(0..3) {
            0 => { s.remove(rng.gen_range(0..s.len())); },
            1 => { s.insert(rng.gen_range(0..=s.len()), rng.gen_range('a'..='z')); },
            _ => {
                // yes dirty, can't find better option
                let n = rng.gen_range(0..s.len());
                s.replace_range(n..n+1, &rng.gen_range('a'..='z').to_string());
                // stop judging me
            },
        };
    }
    else {
        s.push(rng.gen_range('a'..='z'));
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut obj = String::from("thisisaverylongstringlikereallyverylongiswearitslikeverylongomgsolongabcdefghijklmnopqrstuvwxyz");
    let mut popsize = 100;

    // getting user inputs (yeah I should use a library, this is not safe and dirty but idc)
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut i = 0;
    let usage = "Usage: ./gapoc [-w|--word objective_word] [-p|--population population_size]";
    if args.len()%2 == 1 { println!("{}", usage); return; }
    while i + 1 < args.len() {
        match args[i].as_str() {
            "-w" | "--word" => obj = args[i + 1].clone(),
            "-p" | "--population" => popsize = args[i + 1].parse::<usize>().unwrap(),
            _ => { println!("{}", usage); return; },
        }
        i+=2;
    }
    if !obj.chars().all(|i| i >= 'a' && i <= 'z') {
        println!("Error: The word must only contain lowercase, non accentuated letters (no spaces either)");
        return;
    }

    // generating random population
    let mut population = vec![(String::from(""), 0); popsize];
    for i in 0..popsize {
        for _ in 0..rng.gen_range(0..=obj.len()*2) {
            population[i].0.push(rng.gen_range('a'..='z'));
        }
    }
    for i in 0..popsize {
        population[i].1 = fitness_opti(&obj, &population[i].0);
    }
    population.sort_by(|a, b| a.1.cmp(&b.1));

    let mut gencount = 0;
    while population[0].1 != 0 {
        gencount += 1;
        // selection of the fittests
        for i in 0..popsize/2 {
            population[i + popsize/2 + popsize%2].0 = population[i].0.clone();
        }

        // mutation
        for i in popsize/4..popsize {
            mutation(&mut population[i].0)
        }

        // calculating new fitness
        for i in 0..popsize {
            population[i].1 = fitness_opti(&obj, &population[i].0);
        }
        population.sort_by(|a, b| a.1.cmp(&b.1));

        println!("gen:{:6} distance:{:3} Best candidate:{:20}", gencount, population[0].1, population[0].0);
    }
}
