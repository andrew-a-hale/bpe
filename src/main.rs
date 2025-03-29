use std::collections::HashMap;
use std::{fs, io::Read};

#[derive(Clone, Debug)]
struct Vocabulary {
    symbols: HashMap<String, usize>,
    reverse: HashMap<usize, String>,
    len: usize,
    cap: usize,
    token_size: usize,
}

impl Vocabulary {
    fn new() -> Vocabulary {
        let sym: HashMap<String, usize> = HashMap::new();
        let reverse_sym: HashMap<usize, String> = HashMap::new();
        Vocabulary {
            symbols: sym,
            reverse: reverse_sym,
            len: 0,
            cap: 500,
            token_size: 8,
        }
    }

    fn insert(&mut self, key: String) {
        if self.len == self.cap {
            return;
        }

        if !self.symbols.contains_key(&key) {
            self.symbols.entry(key.clone()).or_insert(self.len);
            self.reverse.entry(self.len).or_insert(key.clone());
            self.len += 1;
        }
    }

    fn get_reverse(&self, key: &usize) -> String {
        self.reverse.get(key).unwrap().clone()
    }

    fn get(&self, key: &String) -> &usize {
        self.symbols.get(key).expect(
            "symbol missing from vocabulary due to vocabulary cap being too small for given text",
        )
    }
}

fn main() {
    let mut file = fs::File::open("para.txt").expect("failed to open file");
    let mut buf: Vec<u8> = vec![];
    let _ = file.read_to_end(&mut buf);
    let mut vocab = Vocabulary::new();

    let mut string_vec: Vec<String> = vec![];
    for u in buf {
        string_vec.push(String::from_utf8(vec![u]).unwrap())
    }

    for s in string_vec.clone() {
        vocab.insert(s);
    }

    let mut encoding: Vec<usize> = vec![];
    for s in string_vec {
        encoding.push(*vocab.get(&s))
    }

    let _ = bpe(&encoding, &mut vocab);
    println!("{:?}", vocab.symbols);
}

fn bpe(buf: &Vec<usize>, vocab: &mut Vocabulary) -> Vec<usize> {
    if vocab.len == vocab.cap {
        return buf.clone();
    }

    let mut frequencies: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..(buf.len() - 1) {
        let left_id = buf.get(i).unwrap();
        let right_id = buf.get(i + 1).unwrap();
        let pair = (*left_id, *right_id);
        *frequencies.entry(pair).or_insert(0) += 1;
    }

    let most_freq_pair = max_pair(&frequencies).unwrap();
    if most_freq_pair.1 < 2 {
        return buf.clone();
    }

    let mut new_buf: Vec<usize> = vec![];
    let mut skip_flag = false;
    for i in 0..(buf.len() - 1) {
        if skip_flag {
            skip_flag = false;
            continue;
        }

        let left_id = buf.get(i).unwrap();
        let right_id = buf.get(i + 1).unwrap();
        let pair_string = format!(
            "{}{}",
            vocab.get_reverse(left_id),
            vocab.get_reverse(right_id)
        );
        let pair = (*left_id, *right_id);

        if pair == most_freq_pair.0 {
            if pair_string.chars().count() < vocab.token_size {
                vocab.insert(pair_string.clone());
                new_buf.push(*vocab.get(&pair_string));
                skip_flag = true;
            }
        } else {
            new_buf.push(*left_id);
            if i == buf.len() - 2 {
                new_buf.push(*right_id);
            }
            skip_flag = false;
        }
    }

    bpe(&new_buf, vocab)
}

fn max_pair(frequencies: &HashMap<(usize, usize), usize>) -> Option<((usize, usize), usize)> {
    let mut max = usize::MIN;
    let mut l = 0;
    let mut r = 0;
    for ((tl, tr), v) in frequencies {
        if *v > max {
            max = *v;
            l = *tl;
            r = *tr;
        }
    }

    Some(((l, r), max))
}
