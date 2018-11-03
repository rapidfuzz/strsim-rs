// Copyright 2018 Lyndon Brown
//
// Licensed under the MIT license. You may not copy, modify, or distribute this
// file except in compliance with said license. You can find a copy of this
// license either in the LICENSE file, or alternatively at
// <http://opensource.org/licenses/MIT>.

/// Checks both strings for a common prefix, splitting them after it, returning
/// a tuple of the prefix along with the two suffixes
/// `(prefix, a-suffix, b-suffix)`.
pub(crate) fn split_on_common_prefix<'a, 'b>(a: &'a str, b: &'b str)
    -> (&'a str, &'a str, &'b str)
{
    let i = get_diverge_indice(a, b);
    unsafe {
        (a.get_unchecked(..i), a.get_unchecked(i..), b.get_unchecked(i..))
    }
}

/// Finds the byte offset of the next char following a prefix common to both
/// strings.
pub(crate) fn get_diverge_indice(a: &str, b: &str) -> usize {
    a.char_indices()
     .zip(b.char_indices())
     .take_while(|&((_, a_char), (_, b_char))| a_char == b_char)
     .last()
     .map_or(0, |((a_indice, a_char), (_, _))| a_indice + a_char.len_utf8())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_on_common_prefix() {
        assert_eq!(("", "", ""), split_on_common_prefix("", ""));
        assert_eq!(("", "a", ""), split_on_common_prefix("a", ""));
        assert_eq!(("", "", "a"), split_on_common_prefix("", "a"));
        assert_eq!(("a", "", ""), split_on_common_prefix("a", "a"));
        assert_eq!(("", "thank", "you"), split_on_common_prefix("thank", "you"));
        assert_eq!(("", "hello world!", "foo bar"), split_on_common_prefix("hello world!", "foo bar"));
        assert_eq!(("hello w", "orld!", "urld?"), split_on_common_prefix("hello world!", "hello wurld?"));
        assert_eq!(("kit", "ten", "es"), split_on_common_prefix("kitten", "kites"));
        assert_eq!(("kitten", "", ""), split_on_common_prefix("kitten", "kitten"));
        assert_eq!(("ki", "香ten", "tten"), split_on_common_prefix("ki香ten", "kitten"));
        assert_eq!(("ki", "tten", "香ten"), split_on_common_prefix("kitten", "ki香ten"));
        assert_eq!(("ki香ten", "", "s"), split_on_common_prefix("ki香ten", "ki香tens"));
        assert_eq!(("ki香", "ten", "zen"), split_on_common_prefix("ki香ten", "ki香zen"));
    }

    #[test]
    fn test_get_diverge_indice() {
        assert_eq!(0, get_diverge_indice("", ""));
        assert_eq!(0, get_diverge_indice("a", ""));
        assert_eq!(0, get_diverge_indice("", "a"));
        assert_eq!(1, get_diverge_indice("a", "a"));
        assert_eq!(0, get_diverge_indice("thank", "you"));
        assert_eq!(0, get_diverge_indice("hello world!", "foo bar"));
        assert_eq!(7, get_diverge_indice("hello world!", "hello wurld?"));
        assert_eq!(3, get_diverge_indice("kitten", "kites"));
        assert_eq!(6, get_diverge_indice("kitten", "kitten"));
        assert_eq!(2, get_diverge_indice("ki香ten", "kitten"));
        assert_eq!(2, get_diverge_indice("kitten", "ki香ten"));
        assert_eq!(8, get_diverge_indice("ki香ten", "ki香tens"));
        assert_eq!(5, get_diverge_indice("ki香ten", "ki香zen"));
    }
}
