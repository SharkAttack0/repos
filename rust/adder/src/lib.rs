#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(1+2, 4,"Result is not equal to 4");
    }



    #[test]
    fn fail() {
        panic!("Failed on purpose");
    }
}
