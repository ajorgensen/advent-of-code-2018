use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Unable to open file");

    let mut twos = 0;
    let mut threes = 0;

    for id in contents.lines() {
        let freq = frequencies(id);

        if two_of_the_same(&freq) {
            twos += 1;
        }

        if three_of_the_same(&freq) {
            threes += 1;
        }
    }

    println!("Part One: {}", twos * threes);

    for id in contents.lines() {
        for id2 in contents.lines() {
            if id != id2 && levenshtein(id, id2) == 1 { 
                println!("{}, {}, {}", id, id2, common_chars(id, id2));
            }
        }
    }
}

fn common_chars(id: &str, id2: &str) -> String {
    let id_chars: Vec<char> = id.chars().collect();
    let id2_chars: Vec<char> = id2.chars().collect();

    let mut result = String::new();
    for idx in 0..id_chars.len() {
        if id_chars[idx] == id2_chars[idx] {
            result.push(id_chars[idx]);
        }
    }

    result
}

fn two_of_the_same(map: &HashMap<char, u32>) -> bool  {
    for (_c, v) in map {
        if v == &2 {
            return true;
        }
    }

    false
}

fn three_of_the_same(map: &HashMap<char, u32>) -> bool  {
    for (_c, v) in map {
        if v == &3 {
            return true;
        }
    }

    false
}

fn frequencies(id: &str) -> HashMap<char, u32> {
    let mut freq: HashMap<char, u32> = HashMap::new();

    for letter in id.chars() {
        *freq.entry(letter).or_insert(0) +=1;
    }

    freq
}

fn levenshtein(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    let row: Vec<usize> = vec![0; len_b + 1];
    let mut matrix: Vec<Vec<usize>> = vec![row; len_a + 1];

    // initialize string a
    for i in 0..len_a {
        matrix[i+1][0] = matrix[i][0] + 1;
    }

    // initialize string b
    for i in 0..len_b {
        matrix[0][i+1] = matrix[0][i] + 1;
    }

    // calculate matrix
    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let alternatives = [
                // deletion
                matrix[i][j+1] + 1,
                // insertion
                matrix[i+1][j] + 1,
                // match or substitution
                matrix[i][j] + if ca == cb { 0 } else { 1 }];
            matrix[i+1][j+1] = *alternatives.iter().min().unwrap();
        }
    }

    matrix[len_a][len_b]
}