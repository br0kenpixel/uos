use core::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StackString<const N: usize> {
    len: usize,
    bytes: [u8; N],
}

impl<const N: usize> StackString<N> {
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.bytes[..self.len]) }
    }

    pub fn as_str_mut(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.bytes[..self.len]) }
    }

    pub fn push(&mut self, c: char) -> Result<(), ()> {
        self.push_byte(c as u8)
    }

    pub fn push_byte(&mut self, byte: u8) -> Result<(), ()> {
        let slot = self.next_free_slot_ref().ok_or(())?;
        *slot = byte;

        self.len += 1;

        Ok(())
    }

    fn next_free_slot_ref(&mut self) -> Option<&mut u8> {
        let index = self.next_free_slot()?;

        Some(&mut self.bytes[index])
    }

    const fn next_free_slot(&self) -> Option<usize> {
        let candidate = self.len;

        if candidate >= N {
            return None;
        }

        Some(candidate)
    }
}

impl<const N: usize> TryFrom<&str> for StackString<N> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() >= N {
            return Err(());
        }

        let mut hs = Self::default();

        for ch in value.chars() {
            let _ = hs.push(ch);
        }

        Ok(hs)
    }
}

impl<const N: usize> From<[u8; N]> for StackString<N> {
    fn from(value: [u8; N]) -> Self {
        let length = value
            .iter()
            .enumerate()
            .find(|(_, byte)| *byte == &0)
            .map_or(N, |(index, _)| index);

        let mut copy = value;
        copy[length..].fill(0);

        Self {
            len: length,
            bytes: copy,
        }
    }
}

impl<const N: usize> Deref for StackString<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const N: usize> DerefMut for StackString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_str_mut()
    }
}

impl<const N: usize> AsRef<str> for StackString<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> Default for StackString<N> {
    fn default() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }
}

impl<const N: usize> Display for StackString<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
