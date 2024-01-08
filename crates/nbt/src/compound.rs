use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

use crate::value::Value;

#[derive(Debug, Clone, Default)]
pub struct Compound<S = String> {
    map: Map<S>,
}

type Map<S> = std::collections::BTreeMap<S, Value<S>>;

impl<S> Compound<S> {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn new() -> Self {
        Self { map: Map::new() }
    }
}

impl<S> Compound<S>
where
    S: Ord + Hash,
{
    pub fn get<K>(&self, key: &K) -> Option<&Value<S>>
    where
        K: ?Sized + Ord,
        S: Borrow<K>,
    {
        self.map.get(key)
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value<S>>
    where
        K: Into<S>,
        V: Into<Value<S>>,
    {
        self.map.insert(key.into(), value.into())
    }

    pub fn remove<K>(&mut self, key: &K) -> Option<Value<S>>
    where
        K: ?Sized + Ord,
        S: Borrow<K>,
    {
        self.map.remove(key)
    }

    pub fn iter(&self) -> Iter<S> {
        Iter {
            iter: self.map.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<S> {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }

    pub fn keys(&self) -> Keys<S> {
        Keys {
            iter: self.map.keys(),
        }
    }

    pub fn values(&self) -> Values<S> {
        Values {
            iter: self.map.values(),
        }
    }

    pub fn values_mut(&mut self) -> ValuesMut<S> {
        ValuesMut {
            iter: self.map.values_mut(),
        }
    }
}

impl<S> PartialEq for Compound<S>
where
    S: Ord + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

#[cfg(feature = "serde")]
impl<Str> serde::Serialize for Compound<Str>
where
    Str: Ord + Hash + serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.map.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, S> serde::Deserialize<'de> for Compound<S>
where
    S: Ord + Hash + serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Map::<S>::deserialize(deserializer).map(|map| Self { map })
    }

    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Map::<S>::deserialize_in_place(deserializer, &mut place.map)
    }
}

/// Trait that can be used as a key to query a compound. Basically something
/// that can be converted to a type `B` such that `S: Borrow<B>`.
pub trait AsBorrowed<S> {
    type Borrowed: ?Sized;

    fn as_borrowed(&self) -> &Self::Borrowed;
}

impl<Q: ?Sized> AsBorrowed<String> for Q
where
    String: Borrow<Q>,
{
    type Borrowed = Q;

    #[inline]
    fn as_borrowed(&self) -> &Q {
        self
    }
}

impl<'a, Q: ?Sized> AsBorrowed<Cow<'a, str>> for Q
where
    Cow<'a, str>: Borrow<Q>,
{
    type Borrowed = Q;

    #[inline]
    fn as_borrowed(&self) -> &Q {
        self
    }
}

#[cfg(feature = "java_string")]
impl<Q: ?Sized> AsBorrowed<java_string::JavaString> for Q
where
    for<'a> &'a Q: Into<&'a java_string::JavaStr>,
{
    type Borrowed = java_string::JavaStr;

    fn as_borrowed(&self) -> &Self::Borrowed {
        self.into()
    }
}

#[cfg(feature = "java_string")]
impl<Q: ?Sized> AsBorrowed<Cow<'_, java_string::JavaStr>> for Q
where
    for<'a> &'a Q: Into<&'a java_string::JavaStr>,
{
    type Borrowed = java_string::JavaStr;

    fn as_borrowed(&self) -> &Self::Borrowed {
        self.into()
    }
}

impl<S> Extend<(S, Value<S>)> for Compound<S>
where
    S: Ord + Hash,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (S, Value<S>)>,
    {
        self.map.extend(iter)
    }
}

impl<S> FromIterator<(S, Value<S>)> for Compound<S>
where
    S: Ord + Hash,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (S, Value<S>)>,
    {
        Self {
            map: Map::from_iter(iter),
        }
    }
}

pub enum Entry<'a, S = String> {
    Vacant(VacantEntry<'a, S>),
    Occupied(OccupiedEntry<'a, S>),
}

impl<'a, S> Entry<'a, S>
where
    S: Hash + Ord,
{
    pub fn key(&self) -> &S {
        match self {
            Entry::Vacant(ve) => ve.key(),
            Entry::Occupied(oe) => oe.key(),
        }
    }

    pub fn or_insert(self, default: impl Into<Value<S>>) -> &'a mut Value<S> {
        match self {
            Entry::Vacant(ve) => ve.insert(default),
            Entry::Occupied(oe) => oe.into_mut(),
        }
    }

    pub fn or_insert_with<F, V>(self, default: F) -> &'a mut Value<S>
    where
        F: FnOnce() -> V,
        V: Into<Value<S>>,
    {
        match self {
            Entry::Vacant(ve) => ve.insert(default()),
            Entry::Occupied(oe) => oe.into_mut(),
        }
    }

    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut Value<S>),
    {
        match self {
            Entry::Vacant(ve) => Entry::Vacant(ve),
            Entry::Occupied(mut oe) => {
                f(oe.get_mut());
                Entry::Occupied(oe)
            }
        }
    }
}

pub struct VacantEntry<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    ve: std::collections::btree_map::VacantEntry<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    ve: indexmap::map::VacantEntry<'a, S, Value<S>>,
}

impl<'a, S> VacantEntry<'a, S>
where
    S: Ord + Hash,
{
    pub fn key(&self) -> &S {
        self.ve.key()
    }

    pub fn insert(self, v: impl Into<Value<S>>) -> &'a mut Value<S> {
        self.ve.insert(v.into())
    }
}

pub struct OccupiedEntry<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    oe: std::collections::btree_map::OccupiedEntry<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    oe: indexmap::map::OccupiedEntry<'a, S, Value<S>>,
}

impl<'a, S> OccupiedEntry<'a, S>
where
    S: Hash + Ord,
{
    pub fn key(&self) -> &S {
        self.oe.key()
    }

    pub fn get(&self) -> &Value<S> {
        self.oe.get()
    }

    pub fn get_mut(&mut self) -> &mut Value<S> {
        self.oe.get_mut()
    }

    pub fn into_mut(self) -> &'a mut Value<S> {
        self.oe.into_mut()
    }

    pub fn insert(&mut self, v: impl Into<Value<S>>) -> Value<S> {
        self.oe.insert(v.into())
    }

    pub fn remove(self) -> Value<S> {
        self.oe.remove()
    }
}

impl<S, Q> Index<&'_ Q> for Compound<S>
where
    S: Borrow<Q> + Ord + Hash,
    Q: ?Sized + Ord + Hash,
{
    type Output = Value<S>;

    fn index(&self, index: &Q) -> &Self::Output {
        self.map.index(index)
    }
}

impl<S, Q> IndexMut<&'_ Q> for Compound<S>
where
    S: Borrow<Q> + Hash + Ord,
    Q: ?Sized + Ord + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.map.get_mut(index).expect("no entry found for key")
    }
}

macro_rules! impl_iterator_traits {
    (($name:ident $($generics:tt)*) => $item:ty) => {
        impl $($generics)* Iterator for $name $($generics)* {
            type Item = $item;
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }
        }

        #[cfg(feature = "preserve_order")]
        impl $($generics)* DoubleEndedIterator for $name $($generics)* {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.iter.next_back()
            }
        }

        impl $($generics)* ExactSizeIterator for $name $($generics)* {
            #[inline]
            fn len(&self) -> usize {
                self.iter.len()
            }
        }

        impl $($generics)* std::iter::FusedIterator for $name $($generics)* {}
    }
}

impl<'a, S> IntoIterator for &'a Compound<S> {
    type Item = (&'a S, &'a Value<S>);
    type IntoIter = Iter<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.map.iter(),
        }
    }
}

#[derive(Clone)]
pub struct Iter<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    iter: std::collections::btree_map::Iter<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    iter: indexmap::map::Iter<'a, S, Value<S>>,
}

impl_iterator_traits!((Iter<'a, S>) => (&'a S, &'a Value<S>));

impl<'a, S> IntoIterator for &'a mut Compound<S> {
    type Item = (&'a S, &'a mut Value<S>);
    type IntoIter = IterMut<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
}

pub struct IterMut<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    iter: std::collections::btree_map::IterMut<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    iter: indexmap::map::IterMut<'a, S, Value<S>>,
}

impl_iterator_traits!((IterMut<'a, S>) => (&'a S, &'a mut Value<S>));

impl<S> IntoIterator for Compound<S> {
    type Item = (S, Value<S>);
    type IntoIter = IntoIter<S>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
}

pub struct IntoIter<S = String> {
    #[cfg(not(feature = "preserve_order"))]
    iter: std::collections::btree_map::IntoIter<S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    iter: indexmap::map::IntoIter<S, Value<S>>,
}

impl_iterator_traits!((IntoIter<S>) => (S, Value<S>));

#[derive(Clone)]
pub struct Keys<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    iter: std::collections::btree_map::Keys<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    iter: indexmap::map::Keys<'a, S, Value<S>>,
}

impl_iterator_traits!((Keys<'a, S>) => &'a S);

#[derive(Clone)]
pub struct Values<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    iter: std::collections::btree_map::Values<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    iter: indexmap::map::Values<'a, S, Value<S>>,
}

impl_iterator_traits!((Values<'a, S>) => &'a Value<S>);

pub struct ValuesMut<'a, S = String> {
    #[cfg(not(feature = "preserve_order"))]
    iter: std::collections::btree_map::ValuesMut<'a, S, Value<S>>,
    #[cfg(feature = "preserve_order")]
    iter: indexmap::map::ValuesMut<'a, S, Value<S>>,
}

impl_iterator_traits!((ValuesMut<'a, S>) => &'a mut Value<S>);
