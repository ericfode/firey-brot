

#[cfg(test)]
mod tests {
    use std::os::raw::c_double;

    use arrayfire::c64;


    #[test]
    fn it_works(){
        let c1: c64 = c64::new(1.0, 0.0);
        let c2: c64 = c64::new(0.0, 1.0);
        assert_eq!(c1 + c2,c64::new(1.0,1.0));
    }
}