use std::hash::Hash;

use crate::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Compound<S = String> {
    map: Map<S>,
}

type Map<S> = std::collections::BTreeMap<S, Value>;

impl<S> PartialEq for Compound<S>
    where
        S: Ord + Hash
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}
