use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait Palette<T> {
    fn len(&self) -> usize;
    fn get(&self, idx: usize) -> Option<T>;
    fn index(&self, val: T) -> Option<usize>;
}

pub trait PaletteMut<T>: Palette<T> {
    fn insert(&mut self, val: T) -> usize;
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct EmptyPalette<T>(PhantomData<T>);

impl<T> EmptyPalette<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Palette<T> for EmptyPalette<T> {
    fn len(&self) -> usize { 0 }
    fn get(&self, _idx: usize) -> Option<T> { None }
    fn index(&self, _val: T) -> Option<usize> { None }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SinglePalette<T>(pub T);

impl<T> SinglePalette<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
}

impl<T: Copy + PartialEq> Palette<T> for SinglePalette<T> {
    fn len(&self) -> usize { 1 }
    fn get(&self, idx: usize) -> Option<T> { if idx == 0 { Some(self.0) } else { None } }
    fn index(&self, val: T) -> Option<usize> { if self.0 == val { Some(0) } else { None } }
}

// TODO FromIterator
#[derive(Debug, Clone, PartialEq)]
pub struct LinerPalette<T>(pub Vec<T>);

impl<T> LinerPalette<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl<T: PartialEq + Copy> Palette<T> for LinerPalette<T> {
    fn len(&self) -> usize { self.0.len() }
    fn get(&self, idx: usize) -> Option<T> { self.0.get(idx).map(|x| *x) }
    fn index(&self, val: T) -> Option<usize> { self.0.iter().position(|x| *x == val) }
}

impl<T: Copy + PartialEq> PaletteMut<T> for LinerPalette<T> {
    fn insert(&mut self, val: T) -> usize {
        let idx = self.index(val);
        match idx {
            Some(idx) => idx,
            None => {
                let idx = self.len();
                self.0.push(val);
                idx
            }
        }
    }
}

impl<T> From<Vec<T>> for LinerPalette<T> {
    fn from(entries: Vec<T>) -> Self {
        Self(entries)
    }
}

impl<T> FromIterator<T> for LinerPalette<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashPalette<T: Hash + Eq + Copy> {
    table: HashMap<T, usize>,
    pub entries: Vec<T>,
}

impl<T: Copy + Eq + Hash> HashPalette<T> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            entries: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(capacity),
            entries: Vec::with_capacity(capacity),
        }
    }
}

impl<T: Copy + Eq + std::hash::Hash> From<Vec<T>> for HashPalette<T> {
    fn from(entries: Vec<T>) -> Self {
        let mut table = HashMap::with_capacity(entries.len());
        for (idx, entry) in entries.iter().enumerate() {
            table.insert(*entry, idx);
        }
        Self {
            table,
            entries,
        }
    }
}

impl<T: Copy + Eq + std::hash::Hash> FromIterator<T> for HashPalette<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut table = HashMap::new();
        let mut entries = Vec::new();
        for (idx, entry) in iter.into_iter().enumerate() {
            table.insert(entry, idx);
            entries.push(entry.into());
        }
        Self {
            table,
            entries,
        }
    }
}

impl<T: Copy + Eq + std::hash::Hash> Palette<T> for HashPalette<T> {
    fn len(&self) -> usize { self.entries.len() }
    fn get(&self, idx: usize) -> Option<T> { self.entries.get(idx).map(|x| *x) }
    fn index(&self, val: T) -> Option<usize> { self.table.get(&val).map(|x| *x) }
}

impl<T: Copy + Eq + std::hash::Hash> PaletteMut<T> for HashPalette<T> {
    fn insert(&mut self, val: T) -> usize {
        let idx = self.index(val);
        match idx {
            Some(idx) => idx,
            None => {
                let idx = self.len();
                self.entries.push(val);
                self.table.insert(val, idx);
                idx
            }
        }
    }
}
