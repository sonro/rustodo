use std::collections::BTreeMap;

#[derive(Debug)]
struct MemStore<T> {
    storage: BTreeMap<i32, T>,
    serial: i32,
}

trait Insertable<T> {
    fn to_record(self, id: i32) -> T;
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

    fn fetch_all(&self) -> Vec<&T> {
        self.storage.iter().map(|(_, x)| x).collect()
    }

    fn fetch_one(&self, id: i32) -> Option<&T> {
        self.storage.get(&id)
    }

    fn insert<F: Insertable<T>>(&mut self, form: F) -> &T {
        self.serial += 1;
        let item = form.to_record(self.serial);
        self.storage.insert(self.serial, item);
        self.fetch_one(self.serial).expect("just inserted item")
    }

    fn next_id(&self) -> i32 {
        self.serial + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_store_fetch_empty_list() {
        let store = get_empty_mem_store();
        let list = store.fetch_all();
        assert_eq!(0, list.len());
    }

    #[test]
    fn mem_store_fetch_populated_list() {
        let store = get_populated_mem_store();
        let test_records = get_test_records();
        let list = store.fetch_all();
        assert_eq!(test_records.len(), list.len());
        for (record, list) in test_records.iter().zip(list) {
            assert_eq!(record, list);
        }
    }

    #[test]
    fn mem_store_insert_to_empty_list() {
        let mut store = get_empty_mem_store();
        let record = insert_filled_form_to_store(&mut store);
        assert_eq!(1, get_list_len(&store));
        assert_eq!(1, record.id);
    }

    #[test]
    fn mem_store_insert_to_populated_list() {
        let mut store = get_populated_mem_store();
        let existing_list_len = get_list_len(&store);
        let record = insert_filled_form_to_store(&mut store);
        let expected_list_len = existing_list_len + 1;
        assert_eq!(expected_list_len, get_list_len(&store));
        assert_eq!(expected_list_len, record.id as usize);
    }

    #[test]
    fn mem_store_inserted_record_is_as_expected() {
        let (form, mut record) = get_test_form_and_record();
        let mut store = get_empty_mem_store();
        record.id = store.next_id();
        let stored_record = store.insert(form);
        assert_eq!(&record, stored_record);
    }

    #[test]
    fn mem_store_fetch_one_from_empty_list() {
        let store = get_empty_mem_store();
        let result = store.fetch_one(1);
        assert!(result.is_none());
    }

    #[test]
    fn mem_store_fetch_one_from_populated_list() {
        let store = get_populated_mem_store();
        let records = store.fetch_all();
        let result = store.fetch_one(1);
        assert!(result.is_some());
        assert_eq!(records[0], result.unwrap());
    }

    #[test]
    fn mem_store_fetch_one_out_of_bounds_from_populated_list() {
        let store = get_populated_mem_store();
        let result = store.fetch_one(99);
        assert!(result.is_none());
    }

    #[test]
    fn mem_store_fetch_one_after_insert() {
        let mut store = get_populated_mem_store();
        let expected_id = store.next_id();
        let stored_record = insert_filled_form_to_store(&mut store);
        assert_eq!(expected_id, stored_record.id);
        let result = store.fetch_one(stored_record.id);
        assert!(result.is_some());
        assert_eq!(&stored_record, result.unwrap());
    }

    #[test]
    fn test_form_to_record() {
        let (form, record) = get_test_form_and_record();
        let id = record.id;
        let generated_record = form.to_record(id);
        assert_eq!(record, generated_record);
    }

    fn insert_filled_form_to_store(store: &mut MemStore<TestRecord>) -> TestRecord {
        let form = get_filled_test_form();
        // we clone here so the store can continue to be tested
        store.insert(form).clone()
    }

    fn get_list_len(store: &MemStore<TestRecord>) -> usize {
        store.fetch_all().len()
    }

    fn get_populated_mem_store() -> MemStore<TestRecord> {
        let records = get_test_records();
        let mut storage = BTreeMap::new();
        let serial = records.len() as i32;
        for record in records.into_iter() {
            storage.insert(record.id, record);
        }

        MemStore { storage, serial }
    }

    fn get_test_form_and_record() -> (TestForm, TestRecord) {
        (
            TestForm {
                name: Some("Name".to_string()),
                info: None,
            },
            TestRecord {
                id: 1,
                name: "Name".to_string(),
                info: None,
            },
        )
    }

    fn get_empty_mem_store() -> MemStore<TestRecord> {
        MemStore::new()
    }

    fn get_filled_test_form() -> TestForm {
        TestForm {
            name: Some("Name".to_string()),
            info: None,
        }
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

    #[derive(Debug, Clone, PartialEq)]
    struct TestRecord {
        pub id: i32,
        pub name: String,
        pub info: Option<String>,
    }

    struct TestForm {
        pub name: Option<String>,
        pub info: Option<Option<String>>,
    }

    impl Insertable<TestRecord> for TestForm {
        fn to_record(self, id: i32) -> TestRecord {
            let name = self.name.expect("TestRecord needs a name");
            let info = match self.info {
                None | Some(None) => None,
                Some(Some(s)) => Some(s),
            };
            TestRecord { id, name, info }
        }
    }
}
