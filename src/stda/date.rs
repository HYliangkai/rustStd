use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn tta() {
    let fives = Duration::from_micros(21);
    let now_timestemp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取错误");
    println!("时间戳是 : {:?}", now_timestemp)
}
