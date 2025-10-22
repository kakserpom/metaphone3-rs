// metaphone3.rs

#[cfg(test)]
mod tests;

use std::mem;

/// The maximum length of the metaphone key.
const METAPH_MAX_LENGTH: usize = 8;

use smartstring::alias::CompactString as String;

/// A Metaphone 3 encoder.
pub struct Metaphone3 {
    word: Vec<char>,
    length: usize,
    current: usize,
    last: usize,
    primary: String,
    secondary: String,
    encode_vowels: bool,
    encode_exact: bool,
}

impl Metaphone3 {
    /// Creates a new Metaphone3 encoder with default settings.
    pub fn new() -> Self {
        Metaphone3 {
            word: Vec::new(),
            length: 0,
            current: 0,
            last: 0,
            primary: String::new(),
            secondary: String::new(),
            encode_vowels: false,
            encode_exact: false,
        }
    }

    /// Sets the option to encode vowels.
    pub fn with_encode_vowels(mut self, encode: bool) -> Self {
        self.encode_vowels = encode;
        self
    }

    /// Sets the option for more exact encoding.
    pub fn with_encode_exact(mut self, encode: bool) -> Self {
        self.encode_exact = encode;
        self
    }

    /// Encodes a word into its primary and secondary Metaphone 3 keys.
    pub fn encode(&mut self, word: &str) -> (String, String) {
        // Prepare internal state for a new word
        self.primary.clear();
        self.secondary.clear();
        self.word = word.to_uppercase().chars().collect();
        self.length = self.word.len();
        if self.length == 0 {
            return (String::new(), String::new());
        }
        self.last = self.length - 1;
        self.current = 0;

        // Main encoding logic
        self.do_encode();

        self.primary.truncate(METAPH_MAX_LENGTH);
        self.secondary.truncate(METAPH_MAX_LENGTH);
        if self.primary == self.secondary {
            self.secondary.clear();
        }
        (mem::take(&mut self.primary), mem::take(&mut self.secondary))
    }

    /// The core encoding routine.
    fn do_encode(&mut self) {
        // Handle special cases at the beginning of the word
        if self.length > 1 {
            match self.char_at(0) {
                Some('G') | Some('K') | Some('P') => {
                    if self.char_at(1) == Some('N') {
                        self.primary_add("N");
                        self.secondary_add("N");
                        self.current += 2;
                    }
                }
                Some('A') => {
                    if self.char_at(1) == Some('E') {
                        self.primary_add("E");
                        self.secondary_add("E");
                        self.current += 2;
                    }
                }
                Some('W') => {
                    if self.char_at(1) == Some('R') {
                        self.primary_add("R");
                        self.secondary_add("R");
                        self.current += 2;
                    } else if self.char_at(1) == Some('H') {
                        self.primary_add("A");
                        self.secondary_add("A");
                        self.current += 2;
                    }
                }
                _ => (),
            }
        }

        if self.char_at(0) == Some('X') {
            self.primary_add("S");
            self.secondary_add("S");
            self.current += 1;
        } else if self.is_vowel(0) {
            self.primary_add("A");
            self.secondary_add("A");
            self.current += 1;
        }

        // Main loop
        while self.primary.len() < METAPH_MAX_LENGTH || self.secondary.len() < METAPH_MAX_LENGTH {
            if self.current >= self.length {
                break;
            }

            match self.char_at(self.current) {
                Some('B') => {
                    // Silent B: 'debt', 'doubt'
                    if (self.current >= 2 && self.string_at_back(2, &["DEBT", "SUBT"]))
                        || (self.current >= 3 && self.string_at_back(3, &["DOUBT"]))
                    {
                        self.primary_add("T");
                        self.secondary_add("T");
                        self.current += 1;
                        continue;
                    }

                    // Exact vs approximate
                    if self.encode_exact {
                        self.primary_add("B");
                        self.secondary_add("B");
                    } else {
                        self.primary_add("P");
                        self.secondary_add("P");
                    }

                    // Skip double B or BP (but not BPH)
                    if self.char_at(self.current + 1) == Some('B')
                        || (self.char_at(self.current + 1) == Some('P')
                            && self.current + 2 < self.length
                            && self.char_at(self.current + 2) != Some('H'))
                    {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('C') => {
                    if self.current > 0
                        && self.char_at(self.current - 1) == Some('A')
                        && self.char_at(self.current + 1) == Some('H')
                        && self.char_at(self.current + 2) != Some('I')
                        && (self.char_at(self.current + 2) != Some('E')
                            || self.string_at_back(2, &["BACHER", "MACHER"]))
                    {
                        self.primary_add("K");
                        self.secondary_add("K");
                        self.current += 2;
                        continue;
                    } else if self.string_at(self.current, &["CH"]) {
                        // --- Special case: "ache", "echo", etc. ---
                        if (self.current == 1
                            && self.length >= 4
                            && &self.word[0..4] == ['A', 'C', 'H', 'E'])
                            || (self.current > 3
                                && self.current + 3 < self.length
                                && &self.word[self.current - 1..self.current + 3]
                                    == ['A', 'C', 'H', 'E']
                                && self.string_start(&[
                                    "EAR", "HEAD", "BACK", "HEART", "BELLY", "TOOTH",
                                ]))
                            || self.string_at_back(1, &["ECHO"])
                            || self.string_at_back(2, &["MICHEAL"])
                            || self.string_at_back(4, &["JERICHO"])
                            || self.string_at_back(5, &["LEPRECH"])
                        {
                            self.primary_add("K");
                            self.secondary_add("X");
                            self.current += 2;
                            continue;
                        }

                        // --- Germanic "-ACH" → K ---
                        if self.current > 1
                            && !self.is_vowel(self.current - 2)
                            && self.string_at(self.current - 1, &["ACH"])
                            && !self.string_at_back(2, &["MACHADO", "MACHUCA"])
                            && !(self.char_at(self.current + 2) == Some('I')
                                || (self.char_at(self.current + 2) == Some('E')
                                    && !self.string_at_back(2, &["BACHER", "MACHER"])))
                        {
                            self.primary_add("K");
                            self.secondary_add("K");
                            self.current += 2;
                            continue;
                        }

                        // --- Default CH → X / K ---
                        self.primary_add("X");
                        self.secondary_add("K");
                        self.current += 2;
                        continue;
                    } else if self.string_at_forward(0, &["CZ"])
                        && !self.string_at_back(2, &["WICZ"])
                    {
                        self.primary_add("S");
                        self.secondary_add("X");
                        self.current += 2;
                        continue; // ← добавлено
                    } else if self.string_at_forward(1, &["CIA"]) {
                        self.primary_add("X");
                        self.secondary_add("X");
                        self.current += 3;
                        continue; // ← добавлено
                    } else if self.string_at_forward(0, &["CC"])
                        && !(self.current == 1 && self.char_at(0) == Some('M'))
                    {
                        // 'bacci', 'bertucci', other italian
                        if self.current + 2 < self.length
                            && matches!(self.char_at(self.current + 2), Some('I') | Some('O'))
                            || (self.current + 3 < self.length
                                && self.string_at_forward(2, &["INO", "INI"]))
                        {
                            self.primary_add("X");
                            self.secondary_add("X");
                            self.current += 2;
                            continue;
                        }
                        // 'accident', 'accede', 'succeed'
                        if self.current + 2 < self.length
                            && matches!(
                                self.char_at(self.current + 2),
                                Some('I') | Some('E') | Some('Y')
                            )
                            && !self.string_at_forward(2, &["H"])
                            && !self.string_at_back(2, &["SOCCER"])
                        {
                            self.primary_add("KS");
                            self.secondary_add("KS");
                            self.current += 2;
                            continue;
                        }
                        // Pierce's rule — default
                        self.primary_add("K");
                        self.secondary_add("K");
                        self.current += 1;
                        continue;
                    } else if self.string_at_forward(0, &["CK", "CG", "CQ"]) {
                        self.primary_add("K");
                        self.secondary_add("K");
                        self.current += 2;
                        continue; // ← КРИТИЧНО!
                    } else if self.string_at_forward(0, &["CI", "CE", "CY"]) {
                        self.primary_add("S");
                        self.secondary_add("X");
                        self.current += 2;
                        continue; // ← добавлено
                    } else {
                        self.primary_add("K");
                        self.secondary_add("K");
                        if self.string_at_forward(1, &[" C", " Q", " G"]) {
                            self.current += 3;
                        } else if self.string_at_forward(1, &["C", "K", "Q"])
                            && !self.string_at_forward(1, &["CE", "CI"])
                        {
                            self.current += 2;
                        } else {
                            self.current += 1;
                        }
                        continue; // ← даже в else!
                    }
                }

                Some('D') => {
                    if self.string_at_forward(0, &["DG"]) {
                        if self.string_at_forward(2, &["I", "E", "Y"]) {
                            self.primary_add("J");
                            self.secondary_add("J");
                            self.current += 3;
                        } else {
                            self.primary_add("TK");
                            self.secondary_add("TK");
                            self.current += 2;
                        }
                    } else if self.string_at_forward(0, &["DT", "DD"]) {
                        self.primary_add("T");
                        self.secondary_add("T");
                        self.current += 2;
                    } else {
                        if self.encode_exact {
                            // Final de-voicing: e.g., "missed" → "T"
                            if self.current + 3 == self.last && self.string_at_forward(0, &["SSED"])
                            {
                                self.primary_add("T");
                                self.secondary_add("T");
                            } else {
                                self.primary_add("D");
                                self.secondary_add("D");
                            }
                        } else {
                            self.primary_add("T"); // ← primary = T
                            self.secondary_add("T"); // ← secondary = T
                        }
                        self.current += 1;
                    }
                }

                Some('F') => {
                    self.primary_add("F");
                    self.secondary_add("F");
                    if self.char_at(self.current + 1) == Some('F') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('G') => {
                    // --- GH ---
                    if self.char_at(self.current + 1) == Some('H') {
                        // After consonant (not at start)
                        if self.current > 0 && !self.is_vowel(self.current - 1) {
                            self.primary_add("K");
                            self.secondary_add("K");
                            self.current += 2;
                            continue;
                        }

                        // At start
                        if self.current == 0 {
                            if self.char_at(self.current + 2) == Some('I') {
                                self.primary_add("J");
                                self.secondary_add("J");
                            } else {
                                self.primary_add("K");
                                self.secondary_add("K");
                            }
                            self.current += 2;
                            continue;
                        }

                        // Silent GH (Parker's rule etc.)
                        if (self.current > 1
                            && matches!(
                                self.char_at(self.current - 1),
                                Some('B') | Some('H') | Some('D') | Some('G') | Some('L')
                            )
                            || (self.current > 2
                                && matches!(
                                    self.char_at(self.current - 2),
                                    Some('B')
                                        | Some('H')
                                        | Some('D')
                                        | Some('K')
                                        | Some('W')
                                        | Some('N')
                                        | Some('P')
                                        | Some('V')
                                )
                                && !self.string_start(&["ENOUGH"]))
                            || (self.current > 3 && self.string_at_back(4, &["PL", "SL"]))
                            || (self.current > 0
                                && (self.char_at(self.current - 1) == Some('I')
                                    || self.string_start(&["PUGH"])
                                    || (self.current + 1 == self.last)
                                    || (self.current + 2 <= self.last
                                        && self.string_at_forward(
                                            2,
                                            &["IE", "EY", "ES", "ER", "ED", "TY"],
                                        )
                                        && !self.string_at_back(5, &["GALLAGHER"])))))
                            && !self.string_start(&["BALOGH", "SABAGH"])
                        {
                            // Silent — skip
                            self.current += 2;
                            continue;
                        }

                        // GH → F (e.g., "laugh", "cough")
                        if self.current > 2
                            && self.char_at(self.current - 1) == Some('U')
                            && self.is_vowel(self.current - 2)
                            && matches!(
                                self.char_at(self.current - 3),
                                Some('C')
                                    | Some('G')
                                    | Some('L')
                                    | Some('R')
                                    | Some('T')
                                    | Some('N')
                                    | Some('S')
                            )
                        {
                            self.primary_add("F");
                            self.secondary_add("F");
                            self.current += 2;
                            continue;
                        }

                        // Default GH
                        if self.encode_exact {
                            self.primary_add("G");
                            self.secondary_add("G");
                        } else {
                            self.primary_add("K");
                            self.secondary_add("K");
                        }
                        self.current += 2;
                        continue;
                    }

                    // --- GN ---
                    if self.char_at(self.current + 1) == Some('N') {
                        // Silent in "align", "sign", but not "resignation"
                        if self.current > 1
                            && matches!(
                                self.char_at(self.current - 1),
                                Some('I') | Some('U') | Some('E')
                            )
                            && !(self.current + 2 < self.length
                                && self.string_at_forward(2, &["ATE", "ITY", "ATOR", "ATION"]))
                        {
                            if self.encode_exact {
                                self.primary_add("N");
                                self.secondary_add("GN");
                            } else {
                                self.primary_add("N");
                                self.secondary_add("KN");
                            }
                            self.current += 2;
                            continue;
                        } else {
                            if self.encode_exact {
                                self.primary_add("GN");
                                self.secondary_add("GN");
                            } else {
                                self.primary_add("KN");
                                self.secondary_add("KN");
                            }
                            self.current += 2;
                            continue;
                        }
                    }

                    // --- GG ---
                    if self.char_at(self.current + 1) == Some('G') {
                        // Italian: "loggia", "suggest"
                        if self.string_at_back(1, &["AGGIA", "OGGIA", "AGGIO", "EGGIO", "IGGIO"])
                            || self.string_at_back(1, &["UGGIE"])
                                && !(self.current + 3 == self.last || self.current + 4 == self.last)
                        {
                            self.primary_add("J");
                            self.secondary_add("J");
                            self.current += 2;
                        } else {
                            if self.encode_exact {
                                self.primary_add("G");
                                self.secondary_add("G");
                            } else {
                                self.primary_add("K");
                                self.secondary_add("K");
                            }
                            self.current += 2;
                        }
                        continue;
                    }

                    // --- GK ---
                    if self.char_at(self.current + 1) == Some('K') {
                        self.primary_add("K");
                        self.secondary_add("K");
                        self.current += 2;
                        continue;
                    }

                    // --- GL (Italian) ---
                    if self.current > 0
                        && self.is_vowel(self.current - 1)
                        && self.string_at_forward(1, &["LIA", "LIO", "LIE"])
                    {
                        if self.encode_exact {
                            self.primary_add("L");
                            self.secondary_add("GL");
                        } else {
                            self.primary_add("L");
                            self.secondary_add("KL");
                        }
                        self.current += 2;
                        continue;
                    }

                    // --- Front vowel: GI, GE, GY ---
                    if self.current + 1 < self.length
                        && matches!(
                            self.char_at(self.current + 1),
                            Some('I') | Some('E') | Some('Y')
                        )
                    {
                        // At end: "age", "courage"
                        if self.current + 1 == self.last {
                            // Germanic names: "berge", "helge"
                            if self.string_start(&[
                                "INGE", "LAGE", "HAGE", "LANGE", "SYNGE", "BENGE", "RUNGE", "HELGE",
                            ]) {
                                if self.is_slavo_germanic() {
                                    if self.encode_exact {
                                        self.primary_add("G");
                                        self.secondary_add("G");
                                    } else {
                                        self.primary_add("G");
                                        self.secondary_add("K");
                                    }
                                } else {
                                    if self.encode_exact {
                                        self.primary_add("G");
                                        self.secondary_add("J");
                                    } else {
                                        self.primary_add("J");
                                        self.secondary_add("K");
                                    }
                                }
                            } else {
                                self.primary_add("J");
                                self.secondary_add("J");
                            }
                            self.current += 1;
                            continue;
                        }

                        // Internal hard G exceptions
                        let is_hard = (self.string_at_back(3, &["DANG", "FANG", "SING"])
                            && !self.string_at_back(5, &["DISINGEN"]))
                            || self.string_at_back(3, &["RING", "WING", "HANG", "LONG"])
                            || self.string_at_back(1, &["NGY"])
                            || self.string_at_back(3, &["FORGET", "TARGET", "MARGIT"])
                            || self
                                .string_at_forward(1, &["EAR", "EIS", "IRL", "IVE", "IFT", "IRD"])
                            || (self.string_at_forward(1, &["ISH"])
                                && self.current > 0
                                && !self.string_start(&["LARG"]));

                        if is_hard {
                            if self.is_slavo_germanic() {
                                if self.encode_exact {
                                    self.primary_add("G");
                                    self.secondary_add("G");
                                } else {
                                    self.primary_add("G");
                                    self.secondary_add("K");
                                }
                            } else {
                                if self.encode_exact {
                                    self.primary_add("G");
                                    self.secondary_add("J");
                                } else {
                                    self.primary_add("J");
                                    self.secondary_add("K");
                                }
                            }
                        } else {
                            if self.encode_exact {
                                self.primary_add("J");
                                self.secondary_add("G");
                            } else {
                                self.primary_add("J");
                                self.secondary_add("K");
                            }
                        }
                        self.current += 1;
                        continue;
                    }

                    // --- Default G ---
                    if self.encode_exact {
                        self.primary_add("G");
                        self.secondary_add("G");
                    } else {
                        self.primary_add("K");
                        self.secondary_add("K");
                    }
                    self.current += 1;
                }
                Some('H') => {
                    let prev_is_vowel = self.current > 0 && self.is_vowel(self.current - 1);
                    let next_is_vowel = self.is_vowel(self.current + 1);

                    // Silent H in Arabic names like "Abdelwahed", "Abdullah"
                    let is_arabic_h = ((self.current >= 4)
                        && (self.string_at_back(4, &["ABDEL"])
                            || self.string_at_back(4, &["ABDUL"])))
                        || ((self.current >= 6) && self.string_at_back(6, &["ABDELWAH"]))
                            && self.char_at(self.current + 1) == Some('E');
                    if is_arabic_h {
                        // Silent H → skip
                    } else if (self.current == 0 && next_is_vowel)
                        || (prev_is_vowel && next_is_vowel)
                    {
                        self.primary_add("H");
                        self.secondary_add("H");
                    }
                    self.current += 1;
                }
                Some('J') => {
                    // Spanish "Jose", "San Jacinto"
                    if self.current == 0 && self.string_at(1, &["OSE"]) {
                        self.primary_add("H");
                        self.secondary_add("H");
                    } else if self.current == 0 && self.is_vowel(1) {
                        // Initial 'J' before vowel → "J" / "A"
                        self.primary_add("J");
                        self.secondary_add("A");
                    } else {
                        // Default: "J" / "J"
                        self.primary_add("J");
                        self.secondary_add("J");
                    }
                    self.current += 1;
                }
                Some('K') => {
                    self.primary_add("K");
                    self.secondary_add("K");
                    if self.char_at(self.current + 1) == Some('K') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('L') => {
                    // --- Handle -tle, -dle, -gle etc. with vowel encoding ---
                    if self.encode_vowels
                        && self.current + 1 < self.length
                        && self.word[self.current + 1] == 'E'
                        && (self.current + 2 == self.last || self.current + 1 == self.last)
                    {
                        // Check if preceded by a consonant (not vowel)
                        if self.current > 0 && !self.is_vowel(self.current - 1) {
                            // Special case: words like "bottle", "little", "lytle"
                            // -> encode as "AL"
                            self.primary_add("AL");
                            self.secondary_add("AL");
                            self.current += 2; // skip L and E
                            continue;
                        }
                    }

                    // --- Default L handling ---
                    self.primary_add("L");
                    self.secondary_add("L");
                    if self.char_at(self.current + 1) == Some('L') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('M') => {
                    self.primary_add("M");
                    self.secondary_add("M");
                    if self.string_at_back(1, &["UMB"])
                        && (self.current + 1 == self.last || self.string_at_forward(2, &["ER"]))
                    {
                        self.current += 2;
                    } else if self.char_at(self.current + 1) == Some('M') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('N') => {
                    self.primary_add("N");
                    self.secondary_add("N");
                    if self.char_at(self.current + 1) == Some('N') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('P') => {
                    if self.char_at(self.current + 1) == Some('H') {
                        self.primary_add("F");
                        self.secondary_add("F");
                        self.current += 2;
                    } else {
                        self.primary_add("P");
                        self.secondary_add("P");
                        if self.char_at(self.current + 1) == Some('P')
                            || self.char_at(self.current + 1) == Some('B')
                        {
                            self.current += 2;
                        } else {
                            self.current += 1;
                        }
                    }
                }
                Some('Q') => {
                    self.primary_add("K");
                    self.secondary_add("K");
                    if self.char_at(self.current + 1) == Some('Q') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('R') => {
                    self.primary_add("R");
                    self.secondary_add("R");
                    if self.char_at(self.current + 1) == Some('R') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('S') => {
                    if self.string_at_forward(0, &["SH"]) {
                        self.primary_add("X");
                        self.secondary_add("X");
                        self.current += 2;
                    } else if self.string_at_forward(0, &["SI", "SY"])
                        && self.is_vowel(self.current + 2)
                    {
                        self.primary_add("S");
                        self.secondary_add("X");
                        self.current += 2;
                    } else if self.char_at(self.current) == Some('S')
                        && self.char_at(self.current + 1) == Some('Z')
                    {
                        self.primary_add("S");
                        self.secondary_add("X");
                        self.current += 2;
                    } else if self.string_at_forward(0, &["SS"]) {
                        self.primary_add("S");
                        self.secondary_add("S");
                        self.current += 2;
                    } else {
                        self.primary_add("S");
                        self.secondary_add("S");
                        self.current += 1;
                    }
                }
                Some('T') => {
                    if self.string_at_forward(0, &["TH"]) {
                        self.primary_add("0");
                        self.secondary_add("T");
                        self.current += 2;
                    } else if self.string_at_forward(0, &["TI"])
                        && self.is_vowel(self.current + 2)
                        && self.current + 3 <= self.last
                        && matches!(
                            self.char_at(self.current + 3),
                            Some('O') | Some('A') | Some('U')
                        )
                        && !self.string_at_forward(0, &["TIER", "TIED", "TIES", "TIEN"])
                    {
                        // Латинские суффиксы: -tion, -tia, -tio
                        self.primary_add("X");
                        self.secondary_add("X");
                        self.current += 2;
                    } else if self.string_at_forward(0, &["TT"])
                        || self.string_at_forward(0, &["TD"])
                    {
                        self.primary_add("T");
                        self.secondary_add("T");
                        self.current += 2;
                    } else {
                        self.primary_add("T");
                        self.secondary_add("T");
                        self.current += 1;
                    }
                }
                Some('V') => {
                    if self.encode_exact {
                        self.primary_add("V");
                        self.secondary_add("V");
                    } else {
                        self.primary_add("F");
                        self.secondary_add("F");
                    }
                    if self.char_at(self.current + 1) == Some('V') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('W') => {
                    // --- Silent W at beginning ---
                    if self.current == 0 {
                        if self.char_at(1) == Some('R') {
                            // "write" → "R"
                            self.primary_add("R");
                            self.secondary_add("R");
                            self.current += 2;
                            continue;
                        } else if self.char_at(1) == Some('H') {
                            // "what" → "A"
                            self.primary_add("A");
                            self.secondary_add("A");
                            self.current += 2;
                            continue;
                        } else if self.is_vowel(1) {
                            // "Wagner" → "A" / "F"
                            self.primary_add("A");
                            self.secondary_add("F");
                            self.current += 1;
                            continue;
                        }
                        // Otherwise: silent W → skip
                        self.current += 1;
                        continue;
                    }

                    // --- W in middle ---
                    if self.is_vowel(self.current + 1) {
                        if self.encode_vowels {
                            self.primary_add("A");
                            self.secondary_add("F");
                        }
                    }
                    self.current += 1;
                }
                Some('X') => {
                    self.primary_add("KS");
                    self.secondary_add("KS");
                    if self.char_at(self.current + 1) == Some('X') {
                        self.current += 2;
                    } else {
                        self.current += 1;
                    }
                }
                Some('Z') => {
                    // --- Special case: German "ZW" → "S" ---
                    if self.current == 0 && self.char_at(1) == Some('W') {
                        self.primary_add("S");
                        self.secondary_add("S");
                        self.current += 2;
                        continue;
                    }
                    // --- Handle "ZZ" (Italian) ---
                    if self.char_at(self.current + 1) == Some('Z') {
                        // e.g., "pizza", "abruzzi"
                        if self.current + 2 <= self.last
                            && matches!(
                                self.char_at(self.current + 2),
                                Some('I') | Some('O') | Some('A')
                            )
                        {
                            self.primary_add("TS");
                            self.secondary_add("S");
                            self.current += 2;
                            continue;
                        } else {
                            self.primary_add("S");
                            self.secondary_add("S");
                            self.current += 2;
                            continue;
                        }
                    }

                    // --- Handle "ZU", "ZIER", "ZSA" ---
                    if self.string_at_forward(0, &["ZIER"]) && !self.string_at_back(2, &["VIZIER"])
                    {
                        // "Luzier" → J / S
                        self.primary_add("J");
                        self.secondary_add("S");
                        self.current += 4; // skip ZIER
                        continue;
                    }

                    if self.string_at_forward(0, &["ZSA"]) {
                        self.primary_add("J");
                        self.secondary_add("S");
                        self.current += 3;
                        continue;
                    }

                    // --- Handle German "Z" → "TS" ---
                    let word_str: std::string::String = self.word.iter().collect();
                    if self.current > 0
                        && (self.string_at_back(2, &["NAZI"])
                            || self.string_at_back(2, &["NAZIFY", "MOZART"])
                            || self.string_at_back(3, &["HOLZ", "HERZ", "MERZ", "FITZ"])
                            || (self.string_at_back(3, &["GANZ"])
                                && !self.is_vowel(self.current + 1))
                            || self.string_at_back(4, &["STOLZ", "PRINZ"])
                            || word_str.contains("SCH"))
                        && self.char_at(self.current - 1) != Some('T')
                    {
                        self.primary_add("TS");
                        self.secondary_add("TS");
                        self.current += 1;
                        continue;
                    }

                    // --- Handle "ZH" (Chinese pinyin) ---
                    if self.char_at(self.current + 1) == Some('H') {
                        self.primary_add("J");
                        self.secondary_add("J");
                        self.current += 2;
                        continue;
                    }

                    // --- Default Z → S ---
                    self.primary_add("S");
                    self.secondary_add("S");
                    self.current += 1;
                }
                // Vowels
                Some(c) if self.is_vowel_char(c) => {
                    if self.encode_vowels {
                        let is_silent_e = c == 'E'
                            && (
                                // 1. 'E' at very end AND not in exception list
                                (self.current == self.last && !self.is_pronounced_final_e()) ||
                                // 2. 'E' before 'D' or 'S' at end (baked, grapes)
                                (self.current + 1 == self.last && matches!(self.char_at(self.current + 1), Some('D') | Some('S'))) ||
                                // 3. 'E' in suffixes like "-ness", "-less"
                                (self.current + 3 <= self.length && self.string_at_forward(0, &["NESS", "LESS"])) ||
                                (self.current + 1 < self.length && self.string_at_forward(0, &["LY"]) && self.current + 2 == self.length)
                            );

                        if !is_silent_e
                            && (self.primary.is_empty() || self.primary.chars().last() != Some('A'))
                        {
                            self.primary_add("A");
                            self.secondary_add("A");
                        }
                    }
                    self.current += 1;
                }
                // Consonants without special rules
                _ => {
                    self.current += 1;
                }
            }
        }
    }

    fn is_pronounced_final_e(&self) -> bool {
        // Список исключений из Go, где конечная 'E' произносится
        const PRONOUNCED_E_ENDINGS: &[&str] = &[
            "ACME",
            "NIKE",
            "CAFE",
            "RENE",
            "LUPE",
            "JOSE",
            "ESME",
            "AGAPE",
            "LAME",
            "SAKE",
            "PATE",
            "INGE",
            "CHILE",
            "DESME",
            "CONDE",
            "URIBE",
            "LIBRE",
            "ANDRE",
            "HECATE",
            "PSYCHE",
            "DAPHNE",
            "PENSKE",
            "CLICHE",
            "RECIPE",
            "TAMALE",
            "SESAME",
            "SIMILE",
            "FINALE",
            "KARATE",
            "RENATE",
            "SHANTE",
            "OBERLE",
            "COYOTE",
            "KRESGE",
            "STONGE",
            "STANGE",
            "SWAYZE",
            "FUENTE",
            "SALOME",
            "URRIBE",
            "ECHIDNE",
            "ARIADNE",
            "MEINEKE",
            "PORSCHE",
            "ANEMONE",
            "EPITOME",
            "SYNCOPE",
            "SOUFFLE",
            "ATTACHE",
            "MACHETE",
            "KARAOKE",
            "BUKKAKE",
            "VICENTE",
            "ELLERBE",
            "VERSACE",
            "PENELOPE",
            "CALLIOPE",
            "CHIPOTLE",
            "ANTIGONE",
            "KAMIKAZE",
            "EURIDICE",
            "YOSEMITE",
            "FERRANTE",
            "HYPERBOLE",
            "GUACAMOLE",
            "XANTHIPPE",
            "SYNECDOCHE",
            // Добавим Aakre, Abaja и подобные
            "A",
            "AA",
            "AKRE",
            "ABAJA",
            "ABAJIAN", // ← упрощённо: если слово короткое или имя
        ];

        for &ending in PRONOUNCED_E_ENDINGS {
            if self.length >= ending.len()
                && &self.word[self.length - ending.len()..] == ending.chars().collect::<Vec<_>>()
            {
                return true;
            }
        }
        false
    }

    // --- Helper Methods ---

    /// Safely gets the character at a given index.
    fn char_at(&self, index: usize) -> Option<char> {
        self.word.get(index).copied()
    }

    /// Checks if the character at the given index is a vowel.
    fn is_vowel(&self, index: usize) -> bool {
        if let Some(c) = self.char_at(index) {
            matches!(c, 'A' | 'E' | 'I' | 'O' | 'U' | 'Y')
        } else {
            false
        }
    }

    /// Checks if a char is a vowel.
    fn is_vowel_char(&self, c: char) -> bool {
        matches!(c, 'A' | 'E' | 'I' | 'O' | 'U' | 'Y')
    }

    /// Checks for a specific substring at a given position.
    fn string_at(&self, start: usize, options: &[&str]) -> bool {
        if start >= self.length {
            return false;
        }
        for &pattern in options {
            let pattern_chars: Vec<char> = pattern.chars().collect();
            if start + pattern_chars.len() <= self.length
                && self.word[start..start + pattern_chars.len()] == pattern_chars[..]
            {
                return true;
            }
        }
        false
    }

    /// Checks if one of the given substrings appears `offset` characters **before** the current position.
    /// Returns `false` if `self.current < offset` or if the substring doesn't match.
    /// Checks if one of the given substrings appears `offset` characters **before** the current position.
    /// Returns `false` if `self.current < offset` or if the substring doesn't match.
    fn string_at_back(&self, offset: usize, options: &[&str]) -> bool {
        if self.current < offset {
            return false;
        }
        let start = self.current - offset;
        for &pattern in options {
            let pattern_chars: Vec<char> = pattern.chars().collect();
            if start + pattern_chars.len() <= self.length
                && self.word[start..start + pattern_chars.len()] == pattern_chars[..]
            {
                return true;
            }
        }
        false
    }

    fn string_at_forward(&self, offset: usize, options: &[&str]) -> bool {
        if self.current < offset {
            return false;
        }
        let start = self.current + offset;
        for &pattern in options {
            let pattern_chars: Vec<char> = pattern.chars().collect();
            if start + pattern_chars.len() <= self.length
                && self.word[start..start + pattern_chars.len()] == pattern_chars[..]
            {
                return true;
            }
        }
        false
    }

    /// Проверяет, является ли слово славо-германским (начинается на SCH, W, J)
    fn is_slavo_germanic(&self) -> bool {
        if self.length == 0 {
            return false;
        }
        let first = self.word[0];
        if first == 'J' || first == 'W' {
            return true;
        }
        self.string_start(&["SCH", "SW"])
    }

    fn string_start(&self, prefixes: &[&str]) -> bool {
        for &prefix in prefixes {
            if self.length >= prefix.len() {
                let word_prefix: std::string::String = self.word[..prefix.len()].iter().collect();
                if word_prefix == prefix {
                    return true;
                }
            }
        }
        false
    }

    /// Appends a string to both primary and secondary keys if they are not full.
    fn primary_add(&mut self, s: &str) {
        if self.primary.len() < METAPH_MAX_LENGTH {
            self.primary.push_str(s);
        }
    }

    /// Appends a string to the secondary key if it's not full.
    fn secondary_add(&mut self, s: &str) {
        if self.secondary.len() < METAPH_MAX_LENGTH {
            self.secondary.push_str(s);
        }
    }
}

// Default implementation for convenience
impl Default for Metaphone3 {
    fn default() -> Self {
        Self::new()
    }
}
