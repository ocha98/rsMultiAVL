## これは何
Rustで実装したmultisetです。内部はAVL木になっています。

## 使い方
```
let mut tree = MultiAVL::new();
tree.insert(1);
tree.insert(2);
tree.insert(3);
tree.insert(1);
tree.insert(2);
tree.insert(3);

assert_eq!(tree.min_value(), Some(1));
assert_eq!(tree.max_value(), Some(3));

assert_eq!(tree.size(), 6);
assert_eq!(tree.contains(1), true);
assert_eq!(tree.contains(2), true);
assert_eq!(tree.contains(3), true);

tree.erase(1);
assert_eq!(tree.size(), 5);

tree.erase(1);
tree.erase(1);
assert_eq!(tree.contains(1), false);
```