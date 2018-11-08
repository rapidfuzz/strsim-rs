// Copyright 2018 Lyndon Brown
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

/// Checks both strings for a common prefix, splitting them after it.
///
/// It returns a tuple consisting of the prefix, the two suffixes, and the
/// `char` count of the prefix: `(prefix, a-suffix, b-suffix,
/// prefix-char-count)`.
#[inline(always)]
pub(crate) fn split_on_common_prefix<'a, 'b>(a: &'a str, b: &'b str)
    -> (&'a str, &'a str, &'b str, usize)
{
    let (i, cc) = get_diverge_indice(a, b);
    unsafe {
        (a.get_unchecked(..i), a.get_unchecked(i..), b.get_unchecked(i..), cc)
    }
}

/// Finds the byte offset of the next `char` following a prefix common to both
/// strings, and returns this along with the count of `char`s that make up the
/// prefix.
#[inline(always)]
pub(crate) fn get_diverge_indice(a: &str, b: &str) -> (usize, usize) {
    let mut char_count = 0;
    let indice = a.char_indices()
                  .zip(b.char_indices())
                  .take_while(|&((_, a_char), (_, b_char))| a_char == b_char)
                  .inspect(|_| char_count += 1)
                  .last()
                  .map_or(0, |((a_indice, a_char), (_, _))| a_indice + a_char.len_utf8());
    (indice, char_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_on_common_prefix() {
        assert_eq!(("", "", "", 0), split_on_common_prefix("", ""));
        assert_eq!(("", "a", "", 0), split_on_common_prefix("a", ""));
        assert_eq!(("", "", "a", 0), split_on_common_prefix("", "a"));
        assert_eq!(("a", "", "", 1), split_on_common_prefix("a", "a"));
        assert_eq!(("", "thank", "you", 0), split_on_common_prefix("thank", "you"));
        assert_eq!(("", "hello world!", "foo bar", 0), split_on_common_prefix("hello world!", "foo bar"));
        assert_eq!(("hello w", "orld!", "urld?", 7), split_on_common_prefix("hello world!", "hello wurld?"));
        assert_eq!(("kit", "ten", "es", 3), split_on_common_prefix("kitten", "kites"));
        assert_eq!(("kitten", "", "", 6), split_on_common_prefix("kitten", "kitten"));
        assert_eq!(("ki", "香ten", "tten", 2), split_on_common_prefix("ki香ten", "kitten"));
        assert_eq!(("ki", "tten", "香ten", 2), split_on_common_prefix("kitten", "ki香ten"));
        assert_eq!(("ki香ten", "", "s", 6), split_on_common_prefix("ki香ten", "ki香tens"));
        assert_eq!(("ki香", "ten", "zen", 3), split_on_common_prefix("ki香ten", "ki香zen"));
    }

    #[test]
    fn test_get_diverge_indice() {
        assert_eq!((0, 0), get_diverge_indice("", ""));
        assert_eq!((0, 0), get_diverge_indice("a", ""));
        assert_eq!((0, 0), get_diverge_indice("", "a"));
        assert_eq!((1, 1), get_diverge_indice("a", "a"));
        assert_eq!((0, 0), get_diverge_indice("thank", "you"));
        assert_eq!((0, 0), get_diverge_indice("hello world!", "foo bar"));
        assert_eq!((7, 7), get_diverge_indice("hello world!", "hello wurld?"));
        assert_eq!((3, 3), get_diverge_indice("kitten", "kites"));
        assert_eq!((6, 6), get_diverge_indice("kitten", "kitten"));
        assert_eq!((2, 2), get_diverge_indice("ki香ten", "kitten"));
        assert_eq!((2, 2), get_diverge_indice("kitten", "ki香ten"));
        assert_eq!((8, 6), get_diverge_indice("ki香ten", "ki香tens"));
        assert_eq!((5, 3), get_diverge_indice("ki香ten", "ki香zen"));
    }
}
