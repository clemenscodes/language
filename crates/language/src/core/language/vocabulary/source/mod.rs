use crate::core::language::vocabulary::name::Name;
use crate::core::language::vocabulary::word::Word;
use ddd::{DomainLayer, Layered, ValueObject};
use std::iter::Flatten;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Source {
    names: Vec<Name>,
}

impl Layered for Source {
    type Layer = DomainLayer;
}

impl ValueObject for Source {}

impl Source {
    fn code(text: &str) -> &str {
        const TESTS: &str = "#[cfg(test)]";
        match text.find(TESTS) {
            Some(position) => &text[..position],
            None => text,
        }
    }

    fn strip(text: &str) -> String {
        let letters: Vec<char> = text.chars().collect();
        let mut kept_word = String::new();
        let mut position = 0;
        while position < letters.len() {
            let letter = letters[position];
            let next_letter = letters.get(position + 1).copied();
            if letter == '"' {
                kept_word.push(' ');
                position += 1;
                while position < letters.len() {
                    let quoted_letter = letters[position];
                    if quoted_letter == '\\' {
                        position += 2;
                        continue;
                    }
                    if quoted_letter == '"' {
                        break;
                    }
                    position += 1;
                }
                position += 1;
                continue;
            }
            if letter == '/' && next_letter == Some('/') {
                kept_word.push(' ');
                while position < letters.len() && letters[position] != '\n' {
                    position += 1;
                }
                continue;
            }
            if letter == '/' && next_letter == Some('*') {
                kept_word.push(' ');
                position += 2;
                while position + 1 < letters.len() {
                    let terminator = letters[position] == '*' && letters[position + 1] == '/';
                    if terminator {
                        break;
                    }
                    position += 1;
                }
                position += 2;
                continue;
            }
            if letter == '0' && Self::opens_hexadecimal(&letters, position) {
                kept_word.push(' ');
                position += 2;
                while position < letters.len() {
                    let digit = letters[position];
                    if !digit.is_ascii_hexdigit() && digit != '_' {
                        break;
                    }
                    position += 1;
                }
                continue;
            }
            if letter.is_ascii_digit() && Self::opens_number(&letters, position) {
                kept_word.push(' ');
                while position < letters.len() {
                    let digit = letters[position];
                    if digit.is_ascii_alphanumeric() || digit == '_' {
                        position += 1;
                        continue;
                    }
                    let fraction =
                        digit == '.' && letters.get(position + 1).is_some_and(char::is_ascii_digit);
                    if !fraction {
                        break;
                    }
                    position += 1;
                }
                continue;
            }
            if letter == '\'' && Self::opens_letter(&letters, position) {
                kept_word.push(' ');
                position += 1;
                while position < letters.len() {
                    let quoted_letter = letters[position];
                    if quoted_letter == '\\' {
                        position += 2;
                        continue;
                    }
                    if quoted_letter == '\'' {
                        break;
                    }
                    position += 1;
                }
                position += 1;
                continue;
            }
            kept_word.push(letter);
            position += 1;
        }
        kept_word
    }

    fn opens_hexadecimal(letters: &[char], position: usize) -> bool {
        let next_letter = letters.get(position + 1).copied();
        let hexadecimal_marker = next_letter == Some('x') || next_letter == Some('X');
        if !hexadecimal_marker {
            return false;
        }
        let digit = letters
            .get(position + 2)
            .is_some_and(char::is_ascii_hexdigit);
        if !digit {
            return false;
        }
        match position.checked_sub(1) {
            None => true,
            Some(before) => {
                let letter = letters[before];
                !letter.is_ascii_alphanumeric() && letter != '_'
            }
        }
    }

    fn opens_number(letters: &[char], position: usize) -> bool {
        match position.checked_sub(1) {
            None => true,
            Some(before) => {
                let letter = letters[before];
                !letter.is_ascii_alphanumeric() && letter != '_'
            }
        }
    }

    fn opens_letter(letters: &[char], position: usize) -> bool {
        match letters.get(position + 1) {
            Some('\\') => true,
            Some(_) => letters.get(position + 2) == Some(&'\''),
            None => false,
        }
    }

    fn split(text: &str) -> Vec<Name> {
        text.split(|letter: char| !letter.is_ascii_alphanumeric() && letter != '_')
            .filter_map(|identifier| Name::try_from(identifier).ok())
            .collect()
    }

    pub(super) fn names(&self) -> &[Name] {
        &self.names
    }
}

impl From<&str> for Source {
    fn from(text: &str) -> Self {
        let code = Self::code(text);
        let names = Self::split(&Self::strip(code));
        Self { names }
    }
}

impl<'a> IntoIterator for &'a Source {
    type Item = &'a Word;
    type IntoIter = Flatten<Iter<'a, Name>>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hexadecimal_at_the_very_first_letter_is_no_word() {
        let text = "0xFF";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert!(taken_words.is_empty());
    }

    #[test]
    fn a_word_of_the_code_is_taken_out() {
        let text = "let treasure = 1;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "treasure"]);
    }

    #[test]
    fn an_identifier_falls_apart_at_the_underscore() {
        let text = "let season_profile = 1;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "season", "profile"]);
    }

    #[test]
    fn an_identifier_is_one_name_however_many_words_it_is_made_of() {
        let text = "let season_profile = text.is_empty();";

        let source = Source::from(text);

        let names = source.names().len();
        assert_eq!(names, 4);
    }

    #[test]
    fn an_identifier_falls_apart_at_an_upper_case_letter() {
        let text = "struct MatchPlayerStatistic;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["struct", "Match", "Player", "Statistic"]);
    }

    #[test]
    fn upper_case_letters_are_one_word_until_a_lower_case_letter_opens_the_next() {
        let text = "struct MMRLeaderboard;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["struct", "MMR", "Leaderboard"]);
    }

    #[test]
    fn a_digit_belongs_to_the_word_it_follows() {
        let text = "let ak47Weapon = 1;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "ak47", "Weapon"]);
    }

    #[test]
    fn what_is_inside_a_string_is_no_word_of_the_code() {
        let text = "let kept = \"unlisted\";";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "kept"]);
    }

    #[test]
    fn an_escaped_quotation_mark_does_not_end_a_string() {
        let text = "let kept = \"a \\\" unlisted\"; let after = 1;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "kept", "let", "after"]);
    }

    #[test]
    fn what_is_inside_a_line_comment_is_no_word_of_the_code() {
        let text = "let kept = 1; // unlisted\nlet after = 2;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "kept", "let", "after"]);
    }

    #[test]
    fn what_is_inside_a_block_comment_is_no_word_of_the_code() {
        let text = "let kept = 1; /* unlisted */ let after = 2;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "kept", "let", "after"]);
    }

    #[test]
    fn a_source_without_code_yields_no_word() {
        let text = "";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert!(taken_words.is_empty());
    }

    #[test]
    fn a_literal_of_the_code_is_no_word() {
        let text = "let length = 20;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "length"]);
    }

    #[test]
    fn a_literal_of_base_sixteen_is_no_word() {
        let text = "let polynomial = 0x04C11DB7; let highest = 0x80000000;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "polynomial", "let", "highest"]);
    }

    #[test]
    fn the_suffix_of_a_literal_is_no_word() {
        let text = "static final long serialVersionUID = 1L;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(
            taken_words,
            vec!["static", "final", "long", "serial", "Version", "UID"]
        );
    }

    #[test]
    fn a_fraction_is_no_word_and_neither_is_what_follows_its_point() {
        let text = "double share = 1.5;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["double", "share"]);
    }

    #[test]
    fn a_zero_that_opens_no_hexadecimal_is_a_number_all_the_same() {
        let text = "int count = 0;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["int", "count"]);
    }

    #[test]
    fn a_hexadecimal_under_an_upper_case_marker_is_no_word() {
        let text = "int mask = 0X1F;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["int", "mask"]);
    }

    #[test]
    fn a_text_that_opens_with_a_number_loses_it() {
        let text = "42 apples";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["apples"]);
    }

    #[test]
    fn a_name_that_ends_in_digits_is_still_a_word() {
        let text = "let ak47 = 1; let box0x = 2;";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "ak47", "let", "box0x"]);
    }

    #[test]
    fn what_a_letter_names_is_no_word_of_the_code() {
        let text = "lines.push('\\n');";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["lines", "push"]);
    }

    #[test]
    fn a_letter_of_a_single_character_is_no_word_of_the_code() {
        let text = "lines.push('a');";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["lines", "push"]);
    }

    #[test]
    fn a_quote_that_closes_nothing_opens_no_letter() {
        let text = "let name = text'";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "name", "text"]);
    }

    #[test]
    fn a_lifetime_keeps_its_name_because_it_is_no_letter() {
        let text = "impl<'dependencies> IntoIterator for &'dependencies Dependencies {}";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(
            taken_words,
            vec![
                "impl",
                "dependencies",
                "Into",
                "Iterator",
                "for",
                "dependencies",
                "Dependencies"
            ]
        );
    }

    #[test]
    fn what_a_test_module_names_is_no_word_of_the_code() {
        let text = "let kept = 1;\n#[cfg(test)]\nmod tests { fn the_padding_is_cut_away() {} }";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert_eq!(taken_words, vec!["let", "kept"]);
    }

    #[test]
    fn a_source_of_punctuation_alone_yields_no_word() {
        let text = "{ } ; , ( )";

        let source = Source::from(text);

        let taken_words: Vec<String> = source.into_iter().map(Word::to_string).collect();
        assert!(taken_words.is_empty());
    }

    #[test]
    fn the_same_text_yields_the_same_source_every_time() {
        let text = "let treasure = 1;";

        let first_error = Source::from(text);
        let second_error = Source::from(text);

        assert_eq!(first_error, second_error);
    }
}
