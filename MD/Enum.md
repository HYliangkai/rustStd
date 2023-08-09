## 常用枚举

### Ordering
一般是使用比较函数`a.cmp(b)`产生的比较结果
####  数据结构
```rust
Enum Ordering{
  Less // a<b
  Equal //a==b
  Greater //a>b
}
```
#### 处理函数
+ `is_eq()`：检查比较结果是否为Equal。

+ `is_lt()`：检查比较结果是否为Less。

+ `is_le(self)` : 如果排序的是 Less 或 Equal 变体，则返回 true。

+ `is_ge(self)` : 如果排序的是 Greater 或 Equal 变体，则返回 true。

+ `is_gt()`：检查比较结果是否为Greater。

+ `reverse()` : 反转 Ordering。

+ `then(self, other: Ordering) -> Ordering` : 用给定的函数链接顺序。如果不是 Equal，则返回 self。 否则，调用 f 并返回结果。

