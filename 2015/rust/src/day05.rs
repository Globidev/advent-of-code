fn is_nice_string_p1(s: &str) -> bool {
    let vowels = "aeiou";
    let anomalies = ["ab", "cd", "pq", "xy"];
    let has_3_vowels = s.chars().filter(|c| vowels.contains(*c))
                                .count() >= 3;
    let has_doubles = s.chars().zip(s.chars().skip(1))
                               .any(|(c1, c2)| c1 == c2);
    let has_anomalies = anomalies.iter().any(|pat| s.contains(pat));

    has_3_vowels && has_doubles && !has_anomalies
}

pub fn p1(input: &str) -> usize {
    input.trim().split('\n')
                .filter(|s| is_nice_string_p1(s))
                .count()
}

fn is_nice_string_p2(s: &str) -> bool {
    let has_2_doubles = s.chars().zip(s.chars().skip(1))
                                 .zip(0..)
                                 .any(|(pair, i)| {
        s.chars().skip(i + 2).zip(s.chars().skip(i + 3))
                             .find(|pair2| pair == *pair2).is_some()
    });

    let has_gapped_doubles = s.chars().zip(s.chars().skip(1))
                                      .zip(s.chars().skip(2))
                                      .any(|((c1, _), c3)| c1 == c3);

    has_2_doubles && has_gapped_doubles
}

pub fn p2(input: &str) -> usize {
    input.trim().split('\n')
                .filter(|s| is_nice_string_p2(s))
                .count()
}
