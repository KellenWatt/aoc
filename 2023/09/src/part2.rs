use std::io::stdin;

struct Sequence {
    nums: Vec<i32>,
    subseq: Option<Box<Sequence>>,
}

impl Sequence {
    fn new(nums: Vec<i32>) -> Sequence {
        Sequence{nums, subseq: None}
    }

    fn find_pattern(&mut self) {
        if self.is_final() {return;}

        let sub = (0..(self.nums.len() - 1)).map(|i| {
            self.nums[i+1] - self.nums[i]
        }).collect();
        let mut sub = Sequence::new(sub);
        sub.find_pattern();
        self.subseq = Some(Box::new(sub));
    }

    fn is_final(&self) -> bool {
        self.nums.iter().all(|n| n == &0)
    }

    fn next(&self) -> i32 {
        if self.is_final() {
            0
        } else {
            self.subseq.as_ref().unwrap().next() + self.nums[self.nums.len() - 1]
        }
    }

    fn prev(&self) -> i32 {
        if self.is_final() {
            0
        } else {
            self.nums[0] - self.subseq.as_ref().unwrap().prev() 
        }
    }
}


fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());

    let total: i32 = lines.map(|line| {
        let seq = line.split(" ").map(|n| n.parse().unwrap()).collect();
        let mut seq = Sequence::new(seq);
        seq.find_pattern();
        seq.prev()
    }).sum();

    println!("{}", total);
}
