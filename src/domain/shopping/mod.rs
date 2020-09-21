pub struct List {}

impl List {
    pub fn new() -> List {
        List {}
    }

    pub fn add(&self) {
        Item::new();
    }
}

pub struct Item {}

impl Item {
    pub fn new() -> Item {
        Item {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_adds_items() {
        assert_eq!(2 + 2, 4);
    }
}
