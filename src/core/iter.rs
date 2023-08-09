/** ## iterator的重要性
Iterator在函数式编程中是居于最核心的地位。在函数式编程中，最关键的就是把问题的解决方式设计成能够使用Iterator方案来解决。RUST基本上可以说是原生的Iterator语言，几乎所有的核心关键复合类型都对Iterator作出实现。
 */


pub struct NIterator {
    index: i32,
    value: Vec<String>,
}
/** 首先要明确的是本数据结构是原生就是迭代器还是本身是数据结构,只是可以转成迭代器
如果是前者,则直接实现Iterator-triat即可
如果是后者,则实现下面的类型
 */

/**  ### iteratro类型一 将数据类型变成迭代器
    注意: into_iter返回的迭代器迭代时，会消费变量及容器，完全迭代后容器将不再存在
*/
impl IntoIterator for NIterator {
    type Item = String; //类型转化的最终目标

    type IntoIter = std::vec::IntoIter<Self::Item>; //类型转化的第一层目标,第一层目标实现Iterator-trait 就可以把最终转化目标进行包装,结果就是最终层目标不需要实现Iterator即可转化

    fn into_iter(self) -> Self::IntoIter {
        return self.value.into_iter();
    }
}
//类型一的反面 将迭代器类型变成数据类型
impl FromIterator<String> for NIterator {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        //思路: 将迭代器生成value值,组装成本struct
        let value: Vec<String> = iter.into_iter().collect();
        return NIterator { index: 0, value };
    }
}
// 类型二: 进行不可变引用迭代 : pub fn iter(&self) -> I:Iterator
// 类型三: 进行可变引用迭代 : pub fn iter_mut(&self) -> I:Iterator
//rust要求三种迭代关系要满足:
//1.T::iter() 等同于 &T::into_iter()
//2.T::iter_mut() 等同于 &mut T::into_iter()

/** ### 适配器
Iterator的adapter还有很多，如StepBy, Filter, Zip, Intersperse等等。具体请参考标准库手册。基本上所有的adapter都是遵循Adapter的设计模式来实现的。且每一个适配器的结构及代码逻辑都是比较简单且易理解的
RUST的Iterater的adapter是突出的体现RUST的语法优越性的特性，借助Trait和强大的泛型机制，与c/c++/java相比较，RUST以很少的代码在标准库就实现了最丰富的adapter。而其他语言标准库往往不存在这些适配器，需要其他库来实现。 Iterator的adapter实现了强大的基于Iterator的函数式编程基础设施。函数式编程的基础框架之一便是基于Iterator和闭包实现丰富的adapter。这也凸显了RUST在语言级别对函数式编程的良好支持。
 */
fn sqp() {}
