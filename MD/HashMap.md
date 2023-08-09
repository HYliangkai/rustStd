## 哈希Map

常用函数: 
+ `new()`：创建一个空的HashMap。

+ `insert(key, value)`：插入一个键值对到HashMap中。

+ `get(key)`：获取指定键的值。

+ `contains_key(key)`：检查HashMap中是否包含指定的键。

+ `remove(key)`：从HashMap中删除指定的键值对。

+ `len()`：获取HashMap中键值对的数量。

+ `is_empty()`：检查HashMap是否为空。

+ `iter()`：返回一个迭代器，用于遍历HashMap中的键值对。

+ `keys()`：返回一个迭代器，用于遍历HashMap中的键。

+ `values()`：返回一个迭代器，用于遍历HashMap中的值。

+ `clear()`：清空HashMap中的所有键值对。

+ `retain(predicate)`：根据指定的谓词函数，保留满足条件的键值对，删除不满足条件的键值对。

+ `drain()`：返回一个迭代器，用于遍历并删除HashMap中的所有键值对。

+ `clone()`：创建一个HashMap的副本。

+ `get_mut(key)`：获取指定键的可变引用，可以用于修改值。

+ `entry(key)->Entry`：返回一个Entry枚举类型的值，用于对HashMap中的键值对进行插入、更新和删除操作。

#### Entry讲解
Entry枚举表示HM某个key的存在情况,数据结构为:
```rust
Enum Entry{
  Occupied(OccupiedEntry<'a, K, V>), //已经存在key
  Vacant(VacantEntry<'a, K, V>),//key不存在
}
```
可根据Entry情况来进行操作,操作函数为:
+ `or_insert(val)->&mut val` : 如果为空，则通过插入默认值来确保该值在条目中，并返回对条目中值的可变引用。

+ `or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V` : 如果为空执行Fn

+ `and_modify(self, f: Fn) -> Self` : 在任何潜在的插入 map 之前，提供对占用条目的就地可变访问 (相当于插入拦截器)

+ `key(&self) -> &K` : 返回此条目的键的引用。