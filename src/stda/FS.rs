use std::{ fs::{ read_to_string, File, OpenOptions }, io::Write };

#[test]
fn tty() {
    //以追加模式
    let mut file = OpenOptions::new().append(true).open("m1.txt").unwrap();
    file.write("你好".as_bytes()).expect("错误的");
    println!("xxxoo")
}
