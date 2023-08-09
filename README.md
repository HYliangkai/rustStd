## Rust标准库 学习
rust的标准库分为两个部分:
1. 核心库 + alloc库,不需要导入(预内置导入),提供最基本的常用原语实现
2. 标准库,需要导入
[推荐学习](https://github.com/Warrenren/inside-rust-std-library/blob/main/01-RUST%E5%BA%93%E4%BD%93%E7%B3%BB%E6%A6%82%E8%BF%B0.md)

学习顺序从核心库->标准库

### 学习重点
关注标准库本身,需要关心的几个知识点有:
1. 基础的rust trait代表什么意思,基本上标准库的中带的fn都是实现了某些trair得来的,所以关心trait的实现远比关心fn的作用及其用法更有用
2. std的类型转化实现,譬如迭代器<->集合 Astruct<->Bstruct
3. 系统宏
4. 操作系统的具体api
