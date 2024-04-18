use std::ptr::null;

use crate::{tessAddContour, tessNewTess};


#[test]
fn test() {
    unsafe { 
        let tess = tessNewTess(std::ptr::null_mut());
        tessAddContour(tess, 2, std::ptr::null_mut(), 1, 10);
     }
}