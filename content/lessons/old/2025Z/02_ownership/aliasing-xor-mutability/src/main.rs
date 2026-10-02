fn next_int() -> i32 {
    42
}

struct Data(i32);
impl Data {
    fn new() -> Self {
        Self(0)
    }

    fn read(&self) -> i32 { self.0 }

    fn write(&mut self, n: i32) { self.0 = n }
}

fn thread1(shared_data: &mut Data) {
    loop {
        shared_data.write(next_int());
    }
}

fn thread2(shared_data: &Data) {
    loop {
        println!("{}", shared_data.read());
    }
}

fn main() {
    let mut shared_data = Data::new();

    std::thread::scope(|s| {
        let t1 = s.spawn(|| {
            thread1(&mut shared_data);
        });
        let t2 = s.spawn(|| {
            thread2(&shared_data);
        });
    });
}