trait Registry<T> {
    fn get(&self, key: &str) -> Option<&T>;
}
