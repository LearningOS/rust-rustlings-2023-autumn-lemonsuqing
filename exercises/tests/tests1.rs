// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        /*
        //assert!(); 编译通过，差点以为这样就可以了，于是去问了一下智峰他们才发现不是注释
        于是开始写值
        assert!(0); 报错：不是布尔类型
        */
        assert!(true);
    }
}
