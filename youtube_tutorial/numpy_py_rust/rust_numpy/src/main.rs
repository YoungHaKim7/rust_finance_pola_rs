use ndarray::{Array, Array1, Array3, ShapeBuilder};

fn main() {
    let b01 = Array::from_shape_vec((3, 3), Array1::range(0., 9., 1.).to_vec());
    let b02 = Array::from_shape_vec((2, 2).strides((1, 2)), vec![1., 2., 3., 4.]).unwrap();
    println!("create array : {:?}", b01);
    println!("create array : {:?}", b02);

    println!("~~~~~~3D array~~~~~~~~");
    let mut array = Array3::zeros((3, 3, 3));
    array[[0, 0, 0]] = 111;
    println!("3D array {array:?}");
    println!();
    array[[1, 0, 0]] = 211;
    println!("3D array {array:?}");
    println!();
    array[[2, 1, 0]] = 321;
    println!("3D array {array:?}");
    println!();
    array[[2, 2, 0]] = 421;
    println!("3D array {array:?}");
    println!();
    array[[2, 2, 2]] = 444;
    println!("3D array {array:?}");
    println!();
}
