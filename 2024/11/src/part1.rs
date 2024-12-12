use std::io::stdin;
use std::cell::RefCell;
use std::rc::Rc;

macro_rules! debugln {
    ($($tts:tt)*) => {
        #[cfg(feature = "verbose")]
        println!($($tts)*)
    }
}
macro_rules! debug {
    ($($tts:tt)*) => {
        #[cfg(feature = "verbose")]
        print!($($tts)*)
    }
}

struct Stone {
    data: u64,
    prev: Option<Pointer<Stone>>,
    next: Option<Pointer<Stone>>,
}

type Pointer<T> = Rc<RefCell<T>>;

impl Stone {
    fn new(data: u64) -> Pointer<Stone> {
        Rc::new(RefCell::new(Stone {
            data,
            prev: None,
            next: None,
        }))
    }

    fn add_after(this: &Pointer<Stone>, data: u64) -> Pointer<Stone> {
        let new_stone = Stone::new(data);
        new_stone.borrow_mut().prev = Some(this.clone());
        new_stone.borrow_mut().next = this.borrow_mut().next.take();
        this.borrow_mut().next = Some(new_stone.clone());
        new_stone
    }

    fn iter(this: &Pointer<Stone>) -> StoneIter {
        StoneIter::new(this)
    }

    fn count(this: &Pointer<Stone>) -> usize {
        Stone::iter(this).count()
    }
}

impl std::fmt::Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

struct StoneIter {
    current: Pointer<Stone>,
    done: bool,
}

impl StoneIter {
    fn new(start: &Pointer<Stone>) -> StoneIter {
        StoneIter {
            current: start.clone(),
            done: false,
        }
    }
}

impl Iterator for StoneIter {
    type Item = Pointer<Stone>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
       
        let ret = self.current.clone();
        if self.current.borrow().next.is_none() {
            self.done = true;
        } else {
            let next = self.current.borrow().next.as_ref().unwrap().clone();
            self.current = next;
        }
        Some(ret)
    }
}


fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap());
    let nums: Vec<u64> = lines.next().unwrap().split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

    let mut stone_list = Stone::new(nums[0]);
    let mut tmp = stone_list.clone();
    for n in &nums[1..] {
        tmp = Stone::add_after(&tmp, *n);
    }

    // for s in Stone::iter(&stone_list) {
    //     // if s.borrow().next.is_none() {
    //         Stone::add_after(&s, 1);
    //     // }
    //     // debug!("{} ", s.borrow());
    // }
    // for s in Stone::iter(&stone_list) {
    //     debug!("{} ", s.borrow());
    // }
    // debugln!();
    // debugln!("{}", Stone::count(&stone_list));
    // print_stones(&stone_list);

    let times = 25;
    for _ in 0..times {
        for s in Stone::iter(&stone_list) {
            let val = s.borrow().data;

            if val == 0 {
                s.borrow_mut().data = 1;
            } else if digit_count(val) % 2 == 0 {
                let size = digit_count(val) / 2;
                let m = 10u64.pow(size);
                let left = val / m;
                let right = val % m;
                s.borrow_mut().data = left;
                Stone::add_after(&s, right);
            } else {
                s.borrow_mut().data *= 2024;
            }
        }
        // print_stones(&stone_list);
    }
    println!("{}", Stone::count(&stone_list));
}

fn digit_count(n: u64) -> u32 {
    n.ilog10() + 1
}

fn print_stones(stone: &Pointer<Stone>) {
    #[cfg(feature="verbose")]
    {
    for s in Stone::iter(&stone) {
        debug!("{} ", s.borrow());
    }
    debugln!();
    }
}
