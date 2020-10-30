use std::collections::BTreeMap;

struct MemStore<T> {
    storage: BTreeMap<i32, T>,
    serial: i32,
}

impl<T> MemStore<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self {
            storage: BTreeMap::new(),
            serial: 0,
        }
    }

    fn fetch_all(&self) -> Vec<T> {
        self.storage.iter().map(|(_, x)| x.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_store_fetch_empty_list() {
        let store = MemStore::<TestRecord>::new();
        let list = store.fetch_all();
        assert_eq!(0, list.len());
    }

    #[test]
    fn mem_store_fetch_populated_list() {
        let store = get_test_mem_store();
        let test_records = get_test_records();
        let list = store.fetch_all();
        assert_eq!(test_records.len(), list.len());
        assert_eq!(test_records[0], list[0]);
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestRecord {
        pub id: i32,
        pub name: String,
        pub info: Option<String>,
    }

    fn get_test_mem_store() -> MemStore<TestRecord> {
        let records = get_test_records();
        let mut storage = BTreeMap::new();
        let serial = records.len() as i32;
        for record in records.into_iter() {
            storage.insert(record.id, record);
        }

        MemStore { storage, serial }
    }

    fn get_test_records() -> Vec<TestRecord> {
        vec![
            TestRecord {
                id: 1,
                name: "Test Record 1".to_string(),
                info: None,
            },
            TestRecord {
                id: 2,
                name: "Second Test Record".to_string(),
                info: Some("This is aka Test Record 2".to_string()),
            },
            TestRecord {
                id: 3,
                name: "More Interesting Name".to_string(),
                info: Some("The third Test Record".to_string()),
            },
        ]
    }
}
