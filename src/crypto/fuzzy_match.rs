use strsim::levenshtein;

pub fn find_closest_word<'a>(input: &str, wordlist: &[&'a str]) -> (String, f32) {
    let (closest, dist) = wordlist
        .iter()
        .map(|w| (*w, levenshtein(input, w)))
        .min_by_key(|(_, d)| *d)
        .unwrap_or(("", usize::MAX));

    let confidence = 1.0 - (dist as f32 / input.len().max(1) as f32).min(1.0);
    (closest.to_string(), confidence)
}

pub fn top_n_closest<'a>(input: &str, wordlist: &[&'a str], n: usize) -> Vec<&'a str> {
    let mut scored: Vec<(&str, usize)> = wordlist
        .iter()
        .map(|w| (*w, levenshtein(input, w)))
        .collect();

    scored.sort_by_key(|(_, d)| *d);
    scored.into_iter().take(n).map(|(w, _)| w).collect()
}

pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    levenshtein(a, b)
}

pub fn calculate_confidence(word: &str, candidate: &str) -> f32 {
    let dist = levenshtein(word, candidate);
    1.0 - (dist as f32 / word.len().max(1) as f32).min(1.0)
}
