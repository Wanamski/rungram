use std::{collections::{HashMap, VecDeque}, fs, process::exit};

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
    let binding = data.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "")
        .to_ascii_lowercase();
    // let binding = data.replace(&[ '.', ';', ',',  '-', '“', '’', '”', ':', '?', '—', '‘', '!', '(', ')', '_' ][..], "")
    //     .to_ascii_lowercase();
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

fn main() {
    let train_data = train("./samples/jekyll_hyde.txt", 2);
    println!("{:#?}", train_data);
    println!("{}", train_data.len());
}
