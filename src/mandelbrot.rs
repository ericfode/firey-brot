
mod orbits {
    use arrayfire::{Array, Dim4, c64, flat, iota, join, transpose};


    fn orbit (z:c64, c:c64 ) -> c64{
        return z.powi(2) + c
    }

    fn orbit_loop (z:c64, c:c64) -> u32{
        return 1;
    }

    pub fn gen_screen_space(x_bound: u64, y_bound:u64) -> Array<i64>{
        let a = iota(Dim4::new(&[x_bound,1,1,1]), 
                                   Dim4::new(&[1,y_bound,1,1]));
        let b = transpose(&a, false);
        let coords = join(1, &flat(&a), &flat(&b));
        return coords
    }

//    pub fn screen_space_to_complex(scale:f64) -> Array<c64>{
////
//    }


}

#[cfg(test)]
mod tests {
    use std::os::raw::c_double;

    use arrayfire::{Array, c64,af_print};

    use crate::orbits;


    #[test]
    fn it_works(){
        let c1: c64 = c64::new(1.0, 0.0);
        let c2: c64 = c64::new(0.0, 1.0);
        assert_eq!(c1 + c2,c64::new(1.0,1.0));
    }

    #[test]
    fn make_index(){
        let idx: Array<i64> =  orbits::gen_screen_space(3, 3);
        af_print!("tse",&idx);
        assert_eq!(1,1);

    }
}

