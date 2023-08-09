## 时间系统 -- rust缺少一个简单时间系统

rust 使用	Duration 类型代表时间跨度，通常用于系统超时。
### 生成Duration的方式
Duration中存储个时间跨度的数据

+ 使用new()函数  `Duration(秒,纳秒)->Duration`
+ 从u64类型转化成时间跨度 `Duration::from_xxx(u64)->Duration`


### 获取当前时间戳
== (当前时间-Unix开始时间) as 秒
=即
`SystemTime::now().duration_since(UNIX_EPOCH).expect("获取错误");`