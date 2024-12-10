use std::io::stdin;

fn main() {
    let mut lines = stdin().lines().map(|l| l.unwrap());
    let descriptor = lines.next().unwrap();

    // descriptor.len() * 9 provides an absolute maximum for buffer size
    let mut mem: Vec<isize> = vec![-1; descriptor.len() * 9]; 

    let mut active = false;
    let mut i = 0usize;
    let mut id = 0isize;
    for b in descriptor.bytes() {
        active = !active;
        let n = b - b'0';
        if !active {
            i += n as usize;
            continue;
        }
        for _ in 0..n {
            mem[i] = id;
            i += 1;
        }
        id += 1;
    }

    mem.truncate(i);
    mem.shrink_to_fit();

    let mut head = 0;
    let mut tail = mem.len() - 1;

    while head < tail {
        if mem[tail] < 0 {
            while mem[tail] < 0 {
                tail -= 1;
            }
            mem.truncate(tail+1);
        }
        if mem[head] < 0 {
            mem.swap_remove(head);
            tail -= 1;
        }
        head += 1;
    }

    let mut checksum = 0;

    // this can be done during swaps
    for (i, b) in mem.iter().enumerate() {
        checksum += i * *b as usize;
    }
    println!("{}", checksum);

    // for b in mem {
    //     if b < 0 {
    //         print!(".");
    //     } else {
    //         print!("{}", b);
    //     }
    // }
    // println!();
}
