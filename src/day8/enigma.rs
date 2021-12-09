use crate::day8::parse::parse;
use anyhow::{bail, ensure, Context, Error, Result};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

pub(super) const ZERO: &str = "abcef";
pub(super) const ONE: &str = "cf";
pub(super) const TWO: &str = "acdeg";
pub(super) const THREE: &str = "acdfg";
pub(super) const FOUR: &str = "bcdf";
pub(super) const FIVE: &str = "abdfg";
pub(super) const SIX: &str = "abdefg";
pub(super) const SEVEN: &str = "acf";
pub(super) const EIGHT: &str = "abcdefg";
pub(super) const NINE: &str = "abcdfg";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(super) struct DisplayedDigit(String);

impl Default for DisplayedDigit {
    fn default() -> Self {
        Self(String::from(ZERO))
    }
}

impl AsRef<str> for DisplayedDigit {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for DisplayedDigit {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Deref for DisplayedDigit {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Borrow<String> for DisplayedDigit {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl Borrow<str> for DisplayedDigit {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Display for DisplayedDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for DisplayedDigit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl FromStr for DisplayedDigit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        DisplayedDigit::new(s)
    }
}

impl TryFrom<&str> for DisplayedDigit {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        DisplayedDigit::new(value)
    }
}

impl TryFrom<&String> for DisplayedDigit {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self> {
        DisplayedDigit::new(value)
    }
}

impl TryFrom<String> for DisplayedDigit {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        DisplayedDigit::new(value)
    }
}

impl DisplayedDigit {
    pub(super) fn new<S>(s: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let mut display = [false; 7];
        for char in s.as_ref().chars() {
            match char {
                'a' => display[0] = true,
                'b' => display[1] = true,
                'c' => display[2] = true,
                'd' => display[3] = true,
                'e' => display[4] = true,
                'f' => display[5] = true,
                'g' => display[6] = true,
                bad => bail!("Unexpected char '{}'", bad),
            }
        }
        let mut value = String::new();
        if display[0] {
            value.push('a');
        }
        if display[1] {
            value.push('b');
        }
        if display[2] {
            value.push('c');
        }
        if display[3] {
            value.push('d');
        }
        if display[4] {
            value.push('e');
        }
        if display[5] {
            value.push('f');
        }
        if display[6] {
            value.push('g');
        }
        ensure!(!value.is_empty(), "Cannot have an empty one of these");
        Ok(Self(value))
    }

    /// Does this digit have a unique number of lighted segments? Digits 1, 4, 7 and 8 have unique
    /// numbers of segments (2, 4, 3 and 7, respectively).
    pub(super) fn is_unique(&self) -> bool {
        let l = self.0.len();
        l == 2 || l == 4 || l == 3 || l == 7
    }
}
pub(super) struct Enigma {
    samples: Vec<DisplayedDigit>,
    readouts: Vec<DisplayedDigit>,
}

impl Enigma {
    pub(super) fn new(samples: Vec<DisplayedDigit>, readouts: Vec<DisplayedDigit>) -> Self {
        Self { samples, readouts }
    }

    pub(super) fn sample(&self, i: usize) -> Result<&DisplayedDigit> {
        ensure!(i < 10, "Out of range");
        Ok(&self.samples[i])
    }

    pub(super) fn samples(&self) -> &[DisplayedDigit] {
        self.samples.as_slice()
    }

    pub(super) fn readout(&self, i: usize) -> Result<&DisplayedDigit> {
        ensure!(i < 4, "Out of range");
        Ok(&self.readouts[i])
    }

    pub(super) fn readouts(&self) -> &[DisplayedDigit] {
        self.readouts.as_slice()
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(super) struct Cypher([Option<char>; 7]);

impl Cypher {
    pub(super) fn is_complete(&self) -> bool {
        for c in self.0 {
            if c.is_none() {
                return false;
            }
        }
        true
    }

    pub(super) fn set(&mut self, index: usize, c: char) -> Result<bool> {
        ensure!(index < 7, "Out of range");
        let was_empty = self.0[index].is_none();
        self.0[index] = Some(c);
        Ok(was_empty)
    }

    pub(super) fn map_char(&mut self, from: char, to: char) -> Result<bool> {
        let index = char_pos(from)?;
        self.set(index, to)
    }

    pub(super) fn decode_char(&self, c: char) -> Result<char> {
        let c_pos = char_pos(c)?;
        self.0[c_pos].context("Char mapping unknown")
    }
}

fn char_pos(c: char) -> Result<usize> {
    match c {
        'a' => Ok(0),
        'b' => Ok(1),
        'c' => Ok(2),
        'd' => Ok(3),
        'e' => Ok(4),
        'f' => Ok(5),
        'g' => Ok(6),
        bad => bail!("Bad char '{}'", bad),
    }
}

#[test]
fn is_unique_test() {
    let enigma = parse(
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
    )
    .unwrap();
    assert!(enigma
        .get(0)
        .unwrap()
        .readouts()
        .get(0)
        .unwrap()
        .is_unique());
    assert!(!enigma
        .get(0)
        .unwrap()
        .readouts()
        .get(1)
        .unwrap()
        .is_unique());
    assert!(!enigma
        .get(0)
        .unwrap()
        .readouts()
        .get(2)
        .unwrap()
        .is_unique());
    assert!(enigma
        .get(0)
        .unwrap()
        .readouts()
        .get(3)
        .unwrap()
        .is_unique());
}
