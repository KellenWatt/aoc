mod multirange;
use crate::multirange::{SubRange, MultiRange};

fn main() {
    let s = SubRange::from(10..=20);
    let mut m = MultiRange::new();
    m.add(s);
    m.add(SubRange::from(54..783));
    println!("{}", m);
    let res = m.invert(SubRange::from(0..=4000)).unwrap();
    println!("{}", res);

    // m.add(s);
    // m.add(SubRange::from(30..=40));
    // let mut n = MultiRange::new();
    // n.add(SubRange::from(18..=32));
    // let res = &m & &n;
    // println!("{}", res);

    // m.add(s);
    // println!("{}", m);
    // m.add(SubRange::from(15..40));
    // println!("{}", m);
    // m.add(SubRange::from(0..5));
    // println!("{}", m);
    // m.add(SubRange::from(50..100));
    // println!("{}", m);
    // m.add(SubRange::from(39..=50));
    // println!("{}", m);
    // 
    // m.add(SubRange::from(40..45));
    // println!("{}", m);
    // 
    // m.remove(SubRange::from(50..51));
    // println!("{}", m);
    // 
    // m.remove(SubRange::from(30..61));
    // println!("{}", m);
    // 
    // m.remove(SubRange::from(100..1000));
    // println!("{}", m);
    // 
    // m.add(SubRange::from(35..46));
    // println!("{}", m);
    // 
    // m.add(SubRange::from(0..15));
    // println!("{}", m);
    // 
    // m.add(SubRange::from(29..=61));
    // println!("{}", m);
    // 
    // m.remove(SubRange::from(0..100));
    // println!("{}", m);
}
