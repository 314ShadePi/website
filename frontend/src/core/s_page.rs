#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    pub to: &'static str,
    pub name: &'static str,
}
