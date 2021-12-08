use anyhow::{bail, ensure, Result};

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) struct SubmarineDisplay {
    // map: [char; 7],
    inner: [bool; 7],
}

impl Default for SubmarineDisplay {
    fn default() -> Self {
        Self {
            // map: ['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            inner: [false, false, false, false, false, false, false],
        }
    }
}

impl SubmarineDisplay {
    pub(super) fn lighted(&self) -> String {
        let mut s = String::new();
        if self.inner[0] {
            s.push('a');
        }
        if self.inner[1] {
            s.push('b');
        }
        if self.inner[2] {
            s.push('c');
        }
        if self.inner[3] {
            s.push('d');
        }
        if self.inner[4] {
            s.push('e');
        }
        if self.inner[5] {
            s.push('f');
        }
        if self.inner[6] {
            s.push('g');
        }
        s
    }
}

impl SubmarineDisplay {
    pub(super) fn parse(s: &str) -> Result<Self> {
        let mut display = SubmarineDisplay::default();
        for char in s.chars() {
            match char {
                'a' => display.inner[0] = true,
                'b' => display.inner[1] = true,
                'c' => display.inner[2] = true,
                'd' => display.inner[3] = true,
                'e' => display.inner[4] = true,
                'f' => display.inner[5] = true,
                'g' => display.inner[6] = true,
                bad => bail!("Unexpected char '{}'", bad),
            }
        }
        Ok(display)
    }
}
pub(super) struct Enigma {
    samples: [SubmarineDisplay; 10],
    readouts: [SubmarineDisplay; 4],
}

impl Enigma {
    pub(super) fn new(samples: [SubmarineDisplay; 10], readouts: [SubmarineDisplay; 4]) -> Self {
        Self { samples, readouts }
    }

    pub(super) fn sample(&self, i: usize) -> Result<&SubmarineDisplay> {
        ensure!(i < 10, "Out of range");
        Ok(&self.samples[i])
    }

    pub(super) fn readout(&self, i: usize) -> Result<&SubmarineDisplay> {
        ensure!(i < 4, "Out of range");
        Ok(&self.readouts[i])
    }
}
