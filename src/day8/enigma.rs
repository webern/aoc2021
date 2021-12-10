use crate::day8::parse::parse;
use anyhow::{bail, ensure, Context, Error, Result};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

pub(super) const ZERO: &str = "abcef";
pub(super) const ZERO_LEN: usize = 5;

pub(super) const ONE: &str = "cf";
pub(super) const ONE_LEN: usize = 2;

pub(super) const TWO: &str = "acdeg";
pub(super) const TWO_LEN: usize = ZERO_LEN;

pub(super) const THREE: &str = "acdfg";
pub(super) const THREE_LEN: usize = ZERO_LEN;

pub(super) const FOUR: &str = "bcdf";
pub(super) const FOUR_LEN: usize = 4;

pub(super) const FIVE: &str = "abdfg";
pub(super) const FIVE_LEN: usize = ZERO_LEN;

pub(super) const SIX: &str = "abdefg";
pub(super) const SIX_LEN: usize = 6;

pub(super) const SEVEN: &str = "acf";
pub(super) const SEVEN_LEN: usize = 3;

pub(super) const EIGHT: &str = "abcdefg";
pub(super) const EIGHT_LEN: usize = 7;

pub(super) const NINE: &str = "abcdfg";
pub(super) const NINE_LEN: usize = SIX_LEN;

pub(super) const UNIQUE_LENGTHS: [usize; 4] = [ONE_LEN, FOUR_LEN, SEVEN_LEN, EIGHT_LEN];

pub(super) const A: usize = 0;
pub(super) const B: usize = 1;
pub(super) const C: usize = 2;
pub(super) const D: usize = 3;
pub(super) const E: usize = 4;
pub(super) const F: usize = 5;
pub(super) const G: usize = 6;

pub(crate) fn char_to_position(c: char) -> Result<usize> {
    match c {
        'a' => Ok(A),
        'b' => Ok(B),
        'c' => Ok(C),
        'd' => Ok(D),
        'e' => Ok(E),
        'f' => Ok(F),
        'g' => Ok(G),
        _ => bail!("Bad char '{}'", c),
    }
}

pub(super) struct Enigma<'a> {
    samples: Vec<&'a str>,
    readouts: Vec<&'a str>,
}

impl<'a> Enigma<'a> {
    pub(super) fn new(samples: Vec<&'a str>, readouts: Vec<&'a str>) -> Self {
        Self { samples, readouts }
    }

    pub(super) fn samples(&self) -> &[&'a str] {
        self.samples.as_slice()
    }

    pub(super) fn readouts(&self) -> &[&'a str] {
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
