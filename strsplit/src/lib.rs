// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    // impl<'haystack> Iterator for StrSplit<'haystack> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimter)
        // } else if self.remainder.is_empty() {
        //     None
        } else {
            // let rest = self.remainder;
            // self.remainder = "";
            // Some(rest)
            self.remainder.take()
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

pub fn until_char(s: &str, c: char) -> &str {
    // let delim = format!("{}", c);
    // StrSplit::new(s, &*delim)
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

        let haystack = "a b c d e";
        let letters = StrSplit::new(haystack, " ");
        assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

        let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);

        let haystack = "a b c d e ";
        let letters: Vec<&str> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", "e", ""]);

        assert_eq!(until_char("hello world", 'o'), "hell");
    }
}

// https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=1&t=2s
// https://www.youtube.com/watch?v=MSi3E5Z8oRw
