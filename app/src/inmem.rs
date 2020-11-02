use std::collections::BTreeMap;

pub mod repository;
mod updator;

#[derive(Debug)]
pub struct MemStore<T> {
    storage: BTreeMap<i32, T>,
    serial: i32,
}

pub trait Insertable<T> {
    fn to_item(self, id: i32) -> T;
    fn update_item(self, item: &mut T);
}

impl<T> MemStore<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            storage: BTreeMap::new(),
            serial: 0,
        }
    }

    pub fn fetch_all(&self) -> Vec<T> {
        self.storage.iter().map(|(_, x)| x.clone()).collect()
    }

    pub fn fetch_one(&self, id: i32) -> Option<T> {
        self.storage.get(&id).map(|item| item.clone())
    }

    pub fn insert<F: Insertable<T>>(&mut self, form: F) -> T {
        self.serial += 1;
        let item = form.to_item(self.serial);
        if let Some(_) = self.storage.insert(self.serial, item.clone()) {
            panic!("id should be empty at this address");
        }
        item
    }

    pub fn update<F: Insertable<T>>(&mut self, id: i32, form: F) -> T {
        let item = self.storage.get_mut(&id).expect("item in MemStore");
        form.update_item(item);
        item.clone()
    }

    pub fn delete(&mut self, id: i32) -> Option<T> {
        self.storage.remove(&id)
    }

    pub fn next_id(&self) -> i32 {
        self.serial + 1
    }
}

#[cfg(test)]
mod tests {
    mod util;
    use util::*;

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
            assert_eq!(record, &list);
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
        assert_eq!(record, stored_record);
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
        assert_eq!(stored_record, result.unwrap());
    }

    #[test]
    fn mem_store_update_one_property_exiting_record() {
        let mut store = get_populated_mem_store();
        let original_record = store
            .fetch_one(store.next_id() - 1)
            .expect("get filled record from store");
        let new_name = "Updated Name";
        let form = get_test_form_with_name(new_name);
        let updated_record = store.update(original_record.id, form);
        assert_ne!(original_record, updated_record);
        assert_eq!(original_record.id, updated_record.id);
        assert_eq!(original_record.info, updated_record.info);
        assert_eq!(new_name, updated_record.name);
    }

    #[test]
    fn mem_store_delete_removes_one() {
        let mut store = get_populated_mem_store();
        let list_len = get_list_len(&store);
        store.delete(store.next_id() - 1);
        assert_eq!(list_len - 1, get_list_len(&store));
    }

    #[test]
    fn mem_store_delete_twice_only_removes_one() {
        let mut store = get_populated_mem_store();
        let list_len = get_list_len(&store);
        let id = store.next_id() - 1;
        let result1 = store.delete(id);
        let result2 = store.delete(id);
        assert_eq!(list_len - 1, get_list_len(&store));
        assert!(result1.is_some());
        assert!(result2.is_none());
    }

    #[test]
    fn mem_store_inserting_one_after_delete_has_new_id() {
        let mut store = get_empty_mem_store();
        let record1 = insert_filled_form_to_store(&mut store);
        store.delete(record1.id);
        let record2 = insert_filled_form_to_store(&mut store);
        assert!(record1.id < record2.id);
    }
}
