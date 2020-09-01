#[test]
fn my_example() {
    #[allow(dead_code)]
    mod example {
        include!("../../examples/my_example.rs");

        pub fn test() {
            match run() {
                Ok(()) => (),
                Err(msg) => panic!("{}", msg),
            }
        }
    }

    example::test();
}
