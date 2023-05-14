pub mod arr_module {
    pub fn slice_operations() {
        let arr: [i32; 5] = [0, 1, 2, 3, 4];
        let _complete = &arr[..];
        let _middle = &arr[0..4];

        for element in _complete.iter() {
            println!("{}", element);
        }

        println!("{:?}", _middle);
    }
}
