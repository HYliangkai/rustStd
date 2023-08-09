/** ### 泛型对于rust结构体实现的意义
impl Sized 表示固定内存的数据类型 , 而 ?Sized就表示可以不固定内存的类型,即所有类型
因此rust可以根据泛型  < T: ?Sized >  来为所有类型做impl

譬如:
```rust
//T:?Sized是所有的类型， 不带约束的T实际是 T:Sized
//即类型内存空间固定，所以 T:?Sized才是全部的类型
impl<T: ?Sized> Borrow<T> for T {
    fn borrow(&self) -> &T {
        self
    }
}
impl<T: ?Sized> BorrowMut<T> for T {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}
```
上面的代码表示为所有 ?Sized泛型实现借用的特征,语法为

`impl  <泛型约束> trait<泛型名>  for 泛型{}`

相当于用泛型来实现泛型约束类型的扩展

直接针对泛型做方法和trait的实现是强大的工具，它的作用：

针对泛型的代码会更内聚，方法总比函数具备更明显的模块性
+ 逻辑更清晰及系统化更好
+ 具备更好的可扩展性
+ 更好的支持函数式编程 (import,直接为泛型实现trait可以做到天然的解耦,所以使得函数更为内部无关性)

### 泛型的层次
RUST的泛型从一般到特殊会形成一种层次结构，有些类似于面对对象的基类和子类关系：
最基层： T:?Sized ?Sized的约束表明了所有的类型

一级子层： 默认内存空间固定类型T；裸指针类型`* const T / * mut T; 切片类型[T]`; 数组类型`[T;N]`; 引用类型&T/&mut T; trait约束类型T:trait; 泛型元组(T, U...); 泛型复合类型struct <T>; enum <T>; union<T> 及具体类型 u8/u16/i8/bool/f32/&str/String...

二级子层： 对一级子层的T赋以具体类型 如：* const u8; [i32]，或者将一级子层中的T再次做一级子层的具化，例如：`* const [T]; [*const T]; &(*const T); * const T where T:trait; struct <T:trait>`

### 基于泛型实现函数的流程
之前的要想为一部分struct实现一部分fn,是使用trait来做实现的,即:

先设计trait -> 再设计struct -> 为struct实现trait

现在可以基于泛型实现fn,流程如下:

先设计类型/triat -> 为类型/trait做泛型实现  -> 设计泛型struct -> 根据泛型生成struct

两者相比,区别是:第一种方式是基于特定类型一对一实现的过程,如果写一两个struct还好,一但写多了就没有扩展性了;而第二种方式是基于满足条件的类型一对多实现的过程,具有很高的扩展性,

### RUST的安全本质上仍然是一批高水平程序员实现的一个语言安全框架

 */
pub mod core;
pub mod stda;
