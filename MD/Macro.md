## 函数宏

### 调试

+ `file!()` : 获取代码调用的文件地址

+ `column!()` : 获取代码调用的列

+ `line!()` : 获取代码调用的行

+ `compile_error!(&str)` : 编译失败提示

+ `env!(name,error_msg)` : 输出名为name的环境变量,如果不存在则panic!(error_msg)

+ `module_path!()` : 当前模块的路径

### 简化操作
+ `write!(&mut target,&str)` : 为实现`core::fmt::Write - trait`的数据结构提供一个将&str写入的方法targer的方法

+ `wirteln(&mut target,&str)` : 和wirte!()效果一致,区别是写入完毕后会追加`\n`

+ `concat!(...)->&str` : 将任意可转成&str的数据进行整合并输出&str

+ `stringify!(any)->&str` : 将any直接转化为&str(不需要可&str化,写什么代码就会转什么)

+ `matches!(A,B)->bool` : 返回A和B模式匹配的结果

+ `include!(path)->String`  :  获取同目录下的path文件的数据

+ `format!()` : 将&str整形化为String : [语法](https://rustwiki.org/zh-CN/std/macro.format.html)

### 语意标志

+ `todo!()` : 表示需要完成的代码

+ `unimplemented!()` : 表示未实现的代码,如果存在会导致panic -> (其实就是一个panic简写)

+ `unreachable!()` : 用于表示访问不到的代码的占位符,如果代码执行到了本句说明访问得到,就会发生panic
```rust
fn foo(x: Option<i32>) {
    match x {
        Some(n) if n >= 0 => println!("Some(Non-negative)"),
        Some(n) if n <  0 => println!("Some(Negative)"),
        Some(_)           => unreachable!(), // 如果注释掉，就会编译错误
        None              => println!("None")
    }
}
```