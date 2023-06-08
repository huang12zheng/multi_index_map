use multi_index_map::MultiIndexMap;
use std::collections::BTreeSet;
#[derive(MultiIndexMap, Debug, Clone)]
pub(crate) struct Order {
    pub(crate) order_id: u32,
    #[allow(dead_code)]
    pub(crate) timestamp: u64,
    #[multi_index(hashed_iter)]
    pub(crate) trader_name: BTreeSet<String>,
}

#[test]
fn iter_after_modify() {
    let o1 = Order {
        order_id: 1,
        timestamp: 111,
        trader_name: BTreeSet::from(["John".to_string()]),
    };

    let o2 = Order {
        order_id: 2,
        timestamp: 22,
        trader_name: BTreeSet::from(["Mike".to_string()]),
    };

    let o3 = Order {
        order_id: 3,
        timestamp: 33,
        trader_name: BTreeSet::from(["Tom".to_string(), "John".to_string()]),
    };

    let o4 = Order {
        order_id: 4,
        timestamp: 44,
        trader_name: BTreeSet::from(["Jerry".to_string()]),
    };

    let mut map = MultiIndexOrderMap::default();

    map.insert(o1.clone());
    map.insert(o2);
    map.insert(o3);
    map.insert(o4);

    {
        let mut it = map.iter_by_trader_name();
        assert_eq!(it.next().unwrap().order_id, 3);
        assert_eq!(it.next().unwrap().order_id, 4);
        assert_eq!(it.next().unwrap().order_id, 1);
        assert_eq!(it.next().unwrap().order_id, 3);
        assert_eq!(it.next().unwrap().order_id, 2);
    }

    map.modify_by_trader_name(&"John".to_string(), |o| {
        o.order_id = 5;
    });

    {
        let mut it = map.iter_by_trader_name();
        assert_eq!(it.next().unwrap().order_id, 5);
        assert_eq!(it.next().unwrap().order_id, 4);
        assert_eq!(it.next().unwrap().order_id, 5);
        assert_eq!(it.next().unwrap().order_id, 5);
        assert_eq!(it.next().unwrap().order_id, 2);
    }

    insta::assert_debug_snapshot!(map._trader_name_index);
    insta::assert_debug_snapshot!(map._store);
}
