use std::collections::BTreeMap;

struct MemStore<T> {
    tree: BTreeMap<i32, T>,
    serial: i32,
}

impl<T> MemStore<T> {
    fn new() -> Self {
        Self {
            tree: BTreeMap::new(),
            serial: 0,
        }
    }

    fn fetch_all(&self) -> Vec<T> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRecord {
        pub id: i32,
        pub name: String,
        pub info: Option<String>,
    }

    #[test]
    fn mem_store_fetch_empty_list() {
        let store = MemStore::<TestRecord>::new();
        let result = store.fetch_all();
        assert_eq!(0, result.len());
    }
}
