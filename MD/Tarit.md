## rust命名规则
一个好的规则能使得系统统一规范,并且足够的语义化能更好地读懂代码

rust的规范是使用某一类snake_case前缀来表示某一类代码

+ `try_` : 表示*函数返回Result类型数据*

+ `get_` : 表示*函数返回Option类型数据*

+ `into_` : 表示*有代价的类型转换*,一般都是实现了From trait

+ `from_` : 表示*将一个其他类型转为本类型*

+ `to_` : 表示*类型转换*

+ `new_` : 表示*创建一个新的实例*

+ `with_` : 表示*创建一个带有特定属性或参数的实例*

+ `as_` : 表示*无代价的类型转换*,一般都是实现了AsRef和AsMut trait

+ `is_`/`has_` : 表示*函数返回bool值*

+ `can_`/`should_`/`needs_` : 表示*函数检查某个条件是否满足*


## 介绍一些常用的triat 

### convert : 转换
#### `From` -> `Into` 
: 实现From会自动实现Into , 表示类型转化的两种方法,达成效果一致,区别是调用方式不同:
+ `B::from(A)->B` 
+ `:B=A.into()`
从调用结果来看效果一致
#### `TryFrom` -> `TryInto`
: 实现类型转化,返回一个`Result`:
+ `B::try_from(A)->B`
+ `:B=A.try_into()`

### Copy - Clone : 克隆
+ Copy trait 表示实现了浅克隆 
+ Clone trait 表示实现了深克隆

### Default : 默认值
+ 表示可实现默认值:可直接在struct上声明 #[derive(Default)] 来进行默认声明