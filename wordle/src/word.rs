use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::slice::Iter;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Word {
    inner: Vec<char>,
    counts: HashMap<char, u32>
}

impl Word {
    pub fn new(str: &str) -> Word {
        let mut map: HashMap<char, u32> = HashMap::new();
        for char in str.chars() {
            match map.get_mut(&char) {
                None => map.insert(char, 1),
                Some(count) => { *count += 1; Some(*count) },
            };
        }

        return Word {
            inner: str.chars().collect(),
            counts: map
        }
    }

    pub fn contains(&self, chr: &char) -> bool {
        self.counts.get(chr).unwrap_or(&0) > &0
    }

    pub fn counts(&self) -> &HashMap<char, u32> {
        self.counts.borrow()
    }

    pub fn characters(&self) -> &Vec<char> {
        self.inner.borrow()
    }

    pub fn iter(&self) -> Iter<'_, char> {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a Word {
    type Item = &'a char;
    type IntoIter = <&'a Vec<char> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::into_iter(self.iter())
    }

}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.iter().collect::<String>(), f)
    }
}

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Word::new(s))
    }
}
