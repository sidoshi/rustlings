// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
use std::collections::HashSet;

struct Difference<'first, 'second> {
    first_only: Vec<&'first str>,
    second_only: Vec<&'second str>,
}

fn find_difference<'fst, 'snd>(
    sentence1: &'fst str,
    sentence2: &'snd str,
) -> Difference<'fst, 'snd> {
    let sentence_1_words: HashSet<&'fst str> = sentence1.split(" ").collect();
    let sentence_2_words: HashSet<&'snd str> = sentence2.split(" ").collect();

    Difference {
        first_only: (&sentence_1_words - &sentence_2_words)
            .into_iter()
            .collect(),
        second_only: (&sentence_2_words - &sentence_1_words)
            .into_iter()
            .collect(),
    }
}

fn main() {
    let first_sentence = String::from("I love the surf and the sand.");
    let second_sentence = String::from("I hate the surf and the sand.");

    let first_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&first_sentence, &third_sentence);
        diff.first_only
    };

    assert_eq!(first_only, vec!["hate", "surf"]);

    let second_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&third_sentence, &second_sentence);
        diff.second_only
    };

    assert_eq!(second_only, vec!["snow"]);
}
