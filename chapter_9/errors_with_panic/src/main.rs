fn main() {
    if false {
        panic!("crash and burn");
    }

    if true {
        let v = vec![1, 2, 3];

        // buffer overread
        v[99];
    }
}
