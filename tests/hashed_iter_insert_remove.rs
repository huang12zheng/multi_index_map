use multi_index_map::MultiIndexMap;
use std::collections::BTreeSet;
#[derive(MultiIndexMap, Debug, Clone, PartialEq)]
pub(crate) struct Order {
    pub(crate) order_id: u32,
    pub(crate) timestamp: u64,
    #[multi_index(hashed_iter)]
    pub(crate) trader_name: BTreeSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get_by_trader_name() {
        let mut order_map = MultiIndexOrderMap::with_capacity(10);

        let order1 = Order {
            order_id: 1,
            timestamp: 123456,
            trader_name: BTreeSet::from(["John".to_string(), "Alice".to_string()]),
        };
        order_map.insert(order1.clone());

        let order2 = Order {
            order_id: 2,
            timestamp: 789012,
            trader_name: BTreeSet::from(["Bob".to_string()]),
        };
        order_map.insert(order2.clone());

        let order3 = Order {
            order_id: 3,
            timestamp: 345678,
            trader_name: BTreeSet::from(["John".to_string()]),
        };
        order_map.insert(order3.clone());

        let orders_john = order_map.get_by_trader_name(&"John".to_string());
        assert_eq!(orders_john.len(), 2);
        assert!(orders_john.contains(&&order1));
        assert!(orders_john.contains(&&order3));

        let orders_bob = order_map.get_by_trader_name(&"Bob".to_string());
        assert_eq!(orders_bob.len(), 1);
        assert!(orders_bob.contains(&&order2));

        let orders_alice = order_map.get_by_trader_name(&"Alice".to_string());
        assert_eq!(orders_alice.len(), 1);
        assert!(orders_alice.contains(&&order1));

        let orders_unknown = order_map.get_by_trader_name(&"Unknown".to_string());
        assert_eq!(orders_unknown.len(), 0);
        insta::assert_debug_snapshot!(order_map._trader_name_index);
    }

    #[test]
    fn test_remove_by_trader_name() {
        let mut order_map = MultiIndexOrderMap::with_capacity(10);

        let order1 = Order {
            order_id: 1,
            timestamp: 123456,
            trader_name: BTreeSet::from(["John".to_string(), "Alice".to_string()]),
        };
        order_map.insert(order1.clone());

        let order2 = Order {
            order_id: 2,
            timestamp: 789012,
            trader_name: BTreeSet::from(["Bob".to_string()]),
        };
        order_map.insert(order2.clone());

        let order3 = Order {
            order_id: 3,
            timestamp: 345678,
            trader_name: BTreeSet::from(["John".to_string()]),
        };
        order_map.insert(order3.clone());

        let removed_orders = order_map.remove_by_trader_name(&"John".to_string());
        assert_eq!(removed_orders.len(), 2);
        assert!(removed_orders.contains(&order1));
        assert!(removed_orders.contains(&order3));

        let orders_john = order_map.get_by_trader_name(&"John".to_string());
        assert_eq!(orders_john.len(), 0);

        let orders_alice = order_map.get_by_trader_name(&"Alice".to_string());
        assert_eq!(orders_alice.len(), 0);

        let orders_bob = order_map.get_by_trader_name(&"Bob".to_string());
        assert_eq!(orders_bob.len(), 1);
        assert!(orders_bob.contains(&&order2));
        insta::assert_debug_snapshot!(order_map._trader_name_index);
    }
}
