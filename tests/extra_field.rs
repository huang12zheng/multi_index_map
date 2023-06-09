use multi_index_map::MultiIndexMap;

#[derive(Hash, PartialEq, Eq, Clone)]
struct TestNonPrimitiveType(u64);

#[derive(MultiIndexMap, Clone)]
#[multi_index(extra_field(hashed_unique, field2_extra, String))]
struct TestElement {
    #[multi_index(hashed_unique)]
    field1: TestNonPrimitiveType,
    field2: String,
}
impl TestElement {
    pub fn field2_extra(&self) -> String {
        format!("{}{}", self.field2, "111")
    }
}
#[test]
fn test_insert_and_get() {
    let mut map = MultiIndexTestElementMap::default();
    let elem1 = TestElement {
        field1: TestNonPrimitiveType(42),
        field2: "ElementOne".to_string(),
    };
    map.insert(elem1);

    assert_eq!(map.len(), 1);
    assert_eq!(
        map.get_by_field2_extra(&"ElementOne111".to_owned())
            .unwrap()
            .field2,
        "ElementOne"
    );
}
