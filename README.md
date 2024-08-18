# Caesar Cipher Implementation in Rust

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)

## Introduction

This repository contains an implementation of the Caesar cipher algorithm in Rust. The Caesar cipher is a type of substitution cipher in which each letter in the plaintext is shifted by a fixed number of positions down the alphabet. It's a simple yet classical encryption technique used historically by Julius Caesar.

## Table of Contents

- [Caesar Cipher Algorithm](#caesar-cipher-algorithm)
- [Code Explanation](#code-explanation)

## Caesar Cipher Algorithm

### How It Works

The Caesar cipher works by shifting each letter in the plaintext by a fixed number of positions in the alphabet. For example, with a shift of 3, the letter 'A' would be replaced by 'D', 'B' by 'E', and so on. The alphabet is treated as circular, meaning that after 'Z', it wraps around to 'A'.

### Mathematical Representation

If `P` is the position of a letter in the alphabet (0-indexed, where A=0, B=1, ..., Z=25) and `K` is the shift value, the position `C` of the corresponding letter in the ciphertext can be calculated as:

\[ C = (P + K) \mod 26 \]

Where `26` is the number of letters in the alphabet.

### Example

For a shift of 3:

- Plaintext:  `HELLO`
- Ciphertext: `KHOOR`

Each letter is shifted 3 positions to the right in the alphabet.

## Code Explanation

The following Rust code implements the Caesar cipher:

```rust
fn encrypt(shift: usize, word: String) -> String {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut new_word = String::new();

    for ch in word.chars() {
        // Find the index of the character in the alphabet
        if let Some(index) = alphabet.iter().position(|&c| c == ch) {
            // Calculate the new index with the shift, wrapping around the alphabet
            let new_index = (index + shift) % alphabet.len();
            new_word.push(alphabet[new_index]);
        } else {
            // If the character is not in the alphabet (e.g., punctuation), leave it unchanged
            new_word.push(ch);
        }
    }

    new_word
}

fn main() {
    let shift = 3;
    let word = "hello".to_string();
    let encrypted_word = encrypt(shift, word);
    println!("Encrypted word: {}", encrypted_word);
}
