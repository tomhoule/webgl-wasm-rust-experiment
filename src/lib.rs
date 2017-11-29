mod positions;

#[no_mangle]
pub fn do_the_thing(n: i32) -> i32 {
    9001 + n
}

mod imports {
    extern {
        pub fn alert(input: i32);
    }
}

#[no_mangle]
pub fn initial_positions() -> *mut f32 {
    unsafe { imports::alert(33) };
    let mut vec: Vec<f32> = vec![
         1.0,  1.0,
        -1.0,  1.0,
         1.0, -1.0,
        -1.0, -1.0,
    ];
    vec.as_mut_ptr()
}

#[no_mangle]
pub fn update_positions(vertices: *mut f32) { //-> *mut f32 {
    use std::slice;
    unsafe {
        let slice: &mut [f32] = slice::from_raw_parts_mut(vertices, 8);
        let sum: f32 = slice.iter().sum();
        for vertex in slice.iter_mut() {
            if sum > 2.0 { // expanding
                *vertex *= 1.01
            } else { // collapsing
                *vertex *= 0.99
            }

        }
    }
}
