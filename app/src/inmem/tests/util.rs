use super::super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TestRecord {
    pub id: i32,
    pub name: String,
    pub info: Option<String>,
}

pub struct TestForm {
    pub name: Option<String>,
    pub info: Option<Option<String>>,
}

impl Insertable<TestRecord> for TestForm {
    fn to_item(self, id: i32) -> TestRecord {
        let name = self.name.expect("TestRecord needs a name");
        let info = match self.info {
            None | Some(None) => None,
            Some(Some(s)) => Some(s),
        };
        TestRecord { id, name, info }
    }

    fn update_item(self, item: &mut TestRecord) {
        if let Some(name) = self.name {
            item.name = name;
        }
        if let Some(info) = self.info {
            item.info = info;
        }
    }
}

pub fn insert_filled_form_to_store(store: &mut MemStore<TestRecord>) -> TestRecord {
    let form = get_test_form_with_name("Name");
    // we clone here so the store can continue to be tested
    store.insert(form).clone()
}

pub fn get_next_id(store: &MemStore<TestRecord>) -> i32 {
    get_list_len(&store) as i32 + 1
}

pub fn get_list_len(store: &MemStore<TestRecord>) -> usize {
    store.fetch_all().len()
}

pub fn get_populated_mem_store() -> MemStore<TestRecord> {
    let records = get_test_records();
    let mut storage = BTreeMap::new();
    let serial = records.len() as i32;
    for record in records.into_iter() {
        storage.insert(record.id, record);
    }

    MemStore { storage, serial }
}

pub fn get_test_form_and_record() -> (TestForm, TestRecord) {
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

pub fn get_empty_mem_store() -> MemStore<TestRecord> {
    MemStore::new()
}

pub fn get_test_form_with_name(name: &str) -> TestForm {
    TestForm {
        name: Some(name.to_string()),
        info: None,
    }
}

pub fn get_test_records() -> Vec<TestRecord> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_to_item() {
        let (form, record) = get_test_form_and_record();
        let id = record.id;
        let generated_record = form.to_item(id);
        assert_eq!(record, generated_record);
    }
}
