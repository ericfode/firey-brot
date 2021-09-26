
mod orbits {
    use std::{convert::TryInto, iter::FromIterator};

    use arrayfire::{Array, Convertable, Dim4, c64, cplx, cplx2, dim4, flat, iota, join, seq, transpose, view};


    fn orbit_inner (z:c64, c:&c64 ) -> c64{
        return z.powi(2) + c
    }

    pub fn orbit (c:&c64) -> u32{
        // 
        const ESCAPE_LIMIT : f64 = 2.0;
        const MAX_STEPS :u32 = 50;
        let z = c64::new(0.0,0.0);
    
        fn inner (c:&c64, z:c64, count:u32) -> u32{
            let new_z = orbit_inner(z,c);
            if count >= MAX_STEPS {
                return 0;
            }
            if new_z.norm() > ESCAPE_LIMIT {
                return count;
            } else {
               return inner(c, new_z, count+1);
            }
        }
       return inner(c,z,0)
    }

    pub fn gen_screen_space(x_bound: u64, y_bound:u64) -> Array<f64>{
        let a = iota(Dim4::new(&[x_bound,1,1,1]), 
                                   Dim4::new(&[1,y_bound,1,1]));
        let b = transpose(&a, false);
        let coords = join(1, &flat(&a), &flat(&b));
        return coords
    }

    // scale is the ratio between the size of the complex window to 
    // the pixel window
    pub fn build_scale(screen_dims: (f64,f64), complex_dim: (f64,f64)) -> Array<f64> {
       let x = screen_dims.0 /complex_dim.0;
       let y = screen_dims.1 /complex_dim.1;
       let values: [f64;2] = [x,y];
       return Array::new(&values, Dim4::new(&[2,1,1,1]));
    }

    // Offset is relative to 0,0
    pub fn build_offset(screen_dims: (f64,f64), complex_dim: (f64,f64)) -> Array<f64> {
       let x = 0.0 - complex_dim.0;
       let y = 0.0 - complex_dim.1;
       let values: [f64;2] = [x,y];
       return Array::new(&values, Dim4::new(&[2,1,1,1]));
    }


    

    fn _screen_space_to_complex(space:Array<f64>, 
                                    scale:Array<f64>, 
                                    offset: Array<f64>) -> Array<c64> {
        
        let scale_space = space * scale;
        let offset_space = scale_space- offset;

        let right_value = seq!(1:1:1);
        let left_value = seq!(0:0:1);
        let every_row= seq!();
        let real_values =  view!(offset_space[every_row,right_value]);
        let i_values =  view!(offset_space[every_row,left_value]);

        let cplx_values : Array<c64> =  cplx2(&real_values,&i_values,false).cast();

        return cplx_values;
    }

    pub complex_space(screen_dims:(u64,u64), complex_dims(f64,f64)){
        
    }
    pub fn render(complex_space:Array<c64>) -> Array<u32> {
        let mut vec = vec![c64::default();complex_space.elements()];
        complex_space.host(&mut vec);
        let render =  vec.iter().map(orbit);
        let values = Vec::from_iter(render);
        return Array::new(&values, dim4!(complex_space.elements().try_into().unwrap(),1,1,1));
    }


}

#[cfg(test)]
mod tests {
    use std::os::raw::c_double;

    use arrayfire::{Array, seq, af_print, c64, view};

    use crate::orbits;


    #[test]
    fn it_works(){
        let c1: c64 = c64::new(1.0, 0.0);
        let c2: c64 = c64::new(0.0, 1.0);
        assert_eq!(c1 + c2,c64::new(1.0,1.0));
    }

    #[test]
    fn basic_orbit(){
        assert_eq!(orbits::orbit(&c64::new(1.0,1.0)),1);
        assert_eq!(orbits::orbit(&c64::new(0.5,0.5)),4);
        assert_eq!(orbits::orbit(&c64::new(0.0,0.0)),0);
    }

    #[test]
    fn make_index(){
        let idx: Array<f64> =  orbits::gen_screen_space(3, 3);
        let everyRow= seq!();

        let right_value = seq!(1:1:1);
        let left_value = seq!(0:0:1);
        let _sub = view!(idx[everyRow,right_value]);
        af_print!("tse",&_sub);
        let _sub = view!(idx[everyRow,left_value]);
        af_print!("tse",&_sub);
        assert_eq!(1,1);

    }
}

