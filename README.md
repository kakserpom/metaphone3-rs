# `metaphone3-rs`

`metaphone3-sys` is a Rust crate providing bindings to the Metaphone 3 algorithm, enabling phonetic encoding of words.
The Metaphone 3 algorithm is designed for generating a primary and secondary phonetic encoding of a word, which can be
used for fuzzy matching, searching, and comparison tasks involving English words and names.

This crate allows you to interact with the Go library for Metaphone 3, providing an easy-to-use Rust API. It provides
bindings for the Go-based [Metaphone 3 library](https://github.com/dlclark/metaphone3).

## Features

- **Primary and Secondary Encoding:** It generates both a primary and secondary phonetic encoding of a word, offering
  flexibility in phonetic matching.
- **Vowel Encoding:** The ability to encode vowels separately for finer control over phonetic matching.
- **Exact Matching:** Option to encode exact matches without simplifications, providing a more accurate representation
  of the word.
- **C bindings:** The crate provides FFI bindings to the Metaphone 3 C library, making it suitable for integration with
  other C-based libraries or systems.

## Usage

To use the `metaphone3-sys` crate, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
metaphone3-sys = "0.1"
```

### Functions

#### `metaphone3`

This function encodes a word using the Metaphone 3 algorithm and returns a tuple containing the primary and secondary
encodings.

```rust
pub fn metaphone3(word: &str, encode_vowels: bool, encode_exact: bool) -> (String, String)
```

##### Parameters:

- `word`: The word to encode (type: `&str`).
- `encode_vowels`: Setting EncodeVowels to true will include non-first-letter vowel sounds in the output.
- `encode_exact`: Setting EncodeExact to true will tighten the output so that certain sounds will be differentiated.
  E.g. more separation between hard "G" sounds and hard "K" sounds.

##### Returns:

- A tuple of two `String`s: the primary and secondary phonetic encodings of the word.

### Example

Here’s a simple usage example:

```rust
use metaphone3_sys::metaphone3;

fn main() {
    let word = "SMITH";
    let (primary, secondary) = metaphone3(word, false, false);
    println!("Primary: {}, Secondary: {}", primary, secondary);
}
```

### Tests

The crate includes some tests to ensure proper functionality. To run the tests, use the following command:

```bash
cargo test
```

## Safety

Since the crate interacts with C code via FFI, it is marked as `unsafe` in places where direct memory manipulation
occurs. Please ensure that you are working with valid input and handle the results safely.

## License

This crate is licensed under the MIT License.
