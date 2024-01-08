use cellophanemc_ident::Ident;

trait Registry<T> {
    fn get(&self, key: Ident<&str>) -> Option<&T>;
}

struct MapRegistry<T> {
    items: indexmap::IndexMap<Ident<String>, T>,
}

impl<T> Registry<T> for MapRegistry<T> {
    fn get(&self, key: Ident<&str>) -> Option<&T> {
        self.items.get(key.as_str())
    }
}
