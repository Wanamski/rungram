use std::{collections::{HashMap, VecDeque}, fs, process::exit};
use rand::seq::SliceRandom;

/// takes path to text file and n and trains the model on the given text
fn train(path: &str, n: usize) -> HashMap<Vec<String>, Vec<String>> {

    let path_str = path.to_string();
    let data = match fs::read_to_string(path) {
        Ok(file_string) => file_string,
        Err(_) => {
            println!(
                "Could not find text file under {}.\nPlease provide a valid Path.",
                path_str
            );
            exit(1);
        }
    };
    let mut window: VecDeque<String> = VecDeque::new();
    let mut successor_map: HashMap<Vec<String>, Vec<String>> = HashMap::new();

    // clean string
    // '.;,-“’”:?—‘!()_'
    // let binding = data.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
    //     .to_ascii_lowercase();
    let binding = data.replace(&[ '.', ';', ',',  '-', '“', '’', '”', ':', '?', '—', '‘', '!', '(', ')', '_' ][..], "")
        .to_ascii_lowercase();
    let data = binding.split_whitespace().into_iter();

    for word in data {
        window.push_back(word.to_string());

        if window.len() == n {

            let mut key: Vec<String> = vec![];
            let val = window[n-1].clone();

            for (i, word) in window.iter().enumerate() {
                if i < n - 1 {
                    key.push(word.clone());
                }
                
            }

            if successor_map.contains_key(&key) {
                successor_map.entry(key).and_modify(|v| v.push(val));
            } else {
                successor_map.insert(key, vec![val]);
            }

            window.pop_front();
        }

    }

    successor_map
}

/// generates a new String from a trained map, a start value and a length, how many words should be generated
fn generate(map: HashMap<Vec<String>, Vec<String>>, start: Vec<String>, len: usize) -> String {

    if !map.contains_key(&start) {
        println!("Start value not found in training data...");
        exit(1);
    }

    let mut key_win = start;
    let mut ret_str = String::new();

    for _ in 0..=len {
        let gen_vec: Vec<_> = map.get(&key_win).unwrap().choose_multiple(&mut rand::thread_rng(), 1).collect();
        let gen_str = gen_vec[0];

        ret_str.push_str(&gen_str);
        ret_str.push_str(" ");

        let mut kw_vd = VecDeque::from(key_win);
        kw_vd.push_back(gen_str.clone());
        kw_vd.pop_front();
        key_win = Vec::from(kw_vd);

    }

    ret_str
}

fn main() {
    let train_data = train("./samples/jekyll_hyde.txt", 2);
    
    // let test_start = vec!["had".to_string(), "not".to_string(), "crossed".to_string(), "the".to_string()];
    let test_start = vec![String::from("the")];
    let response_str = generate(train_data, test_start.clone(), 10);

    println!("start value: {:?}", test_start);
    println!("response: {}", response_str);
}
