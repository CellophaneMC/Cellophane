pub trait State {
    fn id(&self) -> u16;

    fn state_property(&self, name: &str) -> Option<&str>;

    fn is_default(&self) -> bool;
}
