struct A {
    name: String,
}
struct B {
    name: String,
}

//实现 A->B 即在B端实现from
impl From<A> for B {
    fn from(value: A) -> Self {
        B { name: value.name }
    }
}


#[test]
fn t1() {
    //为B实现from : B::from(A)->B
    //同时带来into : A.into() ->B
    //所以其实
    let a = A {
        name: "jiojio".to_string(),
    };
    let b: B = a.into();
    let aa = A {
        name: String::from("abc"),
    };
    let bb = B::from(aa);
}
