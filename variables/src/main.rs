fn main() {

    let _p = 2.0; // f64, underscore marks variable as deliberately unused

    let _q: f32 = 3.0; // f32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;

    // remainder
    let _remainder = 43 % 5;

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;

    println!("The value of y is: {}", _y);

    let _a = [1, 2, 3, 4, 5]; // infered declaration
    
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit declaration
    let _first = a[0];
    let _second = a[1];

    let _a = [3; 5]; // same as: let a = [3, 3, 3, 3, 3];

}