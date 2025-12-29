fn generator(limit: u32) -> impl Iterator<Item = u32> {
    let mut current = 0;
    let mut list = vec![1, 1, 1];
    std::iter::from_fn(move || {
        if current < limit {
            println!("generate: {current}");
            let value = current;
            current += 1;
            Some(value)
        } else {
            println!("generator finished");
            None
        }
    })
}

fn main() {
    let mut iter = generator(3);

    for i in 0..5 {
        let num = iter.next();
        match num {
            Some(x) => {
                print!("{}", x)
            }
            None => {}
        }
        println!("{:?}", num);
    }
}
