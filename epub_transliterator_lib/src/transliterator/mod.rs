use std::str::Chars;

use log::{debug, error};

pub fn transliterate_segment(segment: Chars) -> String {
    segment
        .map(transliterate_to_elder_fruthar)
        .collect::<String>()
}

fn transliterate_to_elder_fruthar(b: char) -> char {
    let as_lowercase = b.to_ascii_lowercase();
    for (latin_char, fruthark_char) in ELDER_FRUTHAR_MAPPING {
        if as_lowercase == latin_char {
            if fruthark_char == '?' {
                error!("Unmappable transliteration character: {}", b);
            }
            return fruthark_char;
        }
    }
    debug!("Unmapped character: {}", b);
    b
}

const ELDER_FRUTHAR_MAPPING: [(char, char); 26] = [
    ('a', 'ᚨ'),
    ('b', 'ᛒ'),
    ('c', 'ᚲ'),
    ('d', 'ᛞ'),
    ('e', 'ᛖ'),
    ('f', 'ᚠ'),
    ('g', 'ᚷ'),
    ('h', 'ᚻ'),
    ('i', 'ᛁ'),
    ('j', 'ᛃ'),
    ('k', 'ᚲ'),
    ('l', 'ᛚ'),
    ('m', 'ᛗ'),
    ('n', 'ᚾ'),
    ('o', 'ᛟ'),
    ('p', 'ᛈ'),
    ('q', 'ᛜ'), // self made up
    ('r', 'ᚱ'),
    ('s', 'ᛋ'),
    ('t', 'ᛏ'),
    ('u', 'ᚢ'),
    ('v', 'ᚹ'),
    ('w', 'ᚹ'),
    ('x', 'ᛝ'), // self made up
    ('y', 'ᛁ'),
    ('z', 'ᛉ'),
];

/*
ᚨ => a
ᛒ => b
ᚲ => c
ᛞ => d
ᛖ => e
ᚠ => f
ᚷ => g
ᚻ => h
ᛁ => i
ᛃ => j
ᚲ => k
ᛚ => l
ᛗ => m
ᚾ => n
ᛟ => o
ᛈ => p
ᛜ => q
ᚱ => r
ᛋ => s
ᛏ => t
ᚢ => u
ᚹ => v
ᚹ => w
ᛝ => x
ᛁ => y
ᛉ => z
*/
