#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![warn(unused_variables)]
#![allow(unreachable_code)]

use std::fmt::{self, Display, Formatter};

mod test_module;
mod test;

#[derive(Debug, Default)]
struct VectorStruct {
    a: i32,
    b: i32,
}

/// function to sum components of vectorstruct.
fn sum_function(vec_str: VectorStruct) -> i32 {
    let x: i32 = vec_str.a;
    let y: i32 = vec_str.b;

    return x + y;
}

struct VectorOperation {
    vec_sum: VectorStruct,
    vec_mul: VectorStruct,
}

#[derive(Debug, Default)]
struct Colors {
    red: u32,
    green: u32,
    blue: u32,
}

impl Display for Colors {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "red:{}, green:{}, blue{}",
            self.red, self.green, self.blue
        )
    }
}

fn reverse(tup: (bool, i64)) -> (i64, bool) {
    let (first_el, second_el) = tup;
    return (second_el, first_el);
}

fn transpose_matrix(mat: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    return (mat.0, mat.2, mat.1, mat.3); // access tuple elements as tuple.index
}

#[derive(Debug)]
struct Point {
    x_coordinate: f32,
    y_coordinate: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

/**
calculates the area of a rectangle.
*/
fn rect_area(rect: Rectangle) -> f32 {
    let length: f32 = rect.bottom_right.x_coordinate - rect.top_left.x_coordinate;
    let width: f32 = rect.top_left.y_coordinate - rect.bottom_right.y_coordinate;

    length * width // you can return from a function without writing return, dont put ; at the end
}

fn referenceFunction() -> () {
    let ref1: &i32 = &10;
    println!("ref1 is: {:?}", ref1); // WHY is it printing 10? Should it not print the address of ref1?
    println!("value of ref1 is: {:?}", *ref1);

    match ref1 {
        &val => println!("Matched with val {:?}", val),
        // _ => println!("No match"),  // unreachable pattern
    }

    let ref2: i32 = 20;
    println!("ref2 is: {:?}", ref2);
    println!("value of ref2 is: {:?}", &ref2);
    // println!("value of ref2 is: {:?}", *ref2);  // ERROR: type `i32` cannot be dereferenced

    match ref2 {
        val => println!("Matched with val {:?}", val),
    }
    match ref2 {
        ref val => println!("Matched with ref2 val {:?}", val),
    }

    let ref ref3 = 30;
    println!("ref3 is: {:?}", ref3); // WHY is it printing 10? Should it not print the address of ref1?
    println!("value of ref3 is: {:?}", *ref3);

    match ref3 {
        val => println!("Matched with ref3 val {:?}", val),
    }

    let mut ref4 = 40;
    println!("ref4 is: {:?}", ref4);
    println!("value of ref4 is: {:?}", &ref4);

    match ref4 {
        ref mut val => {
            *val += 11;
            println!("Matched with ref4 {:?}", *val)
        }
    }

    let ref mut ref5 = 50;
    println!("ref5 is: {:?}", ref5);
    println!("value of ref5 is: {:?}", &ref5);

    match ref5 {
        val => {
            *val += 31;
            println!("Matched with ref5 {:?}", *val)
        }
    }
}

// CLASS in Rust
struct Animal;
impl Animal {
    fn name(&self) -> () {
        println!("This is an animal");
    }
}

struct Dog {
    parent: Animal,
}
impl Dog {
    fn name(&self) -> () {
        println!("This is a dog");
    }

    fn sound(&self) -> () {
        println!("Woof");
    }
}

fn classFunction() -> () {
    let dog_obj: Dog = Dog { parent: (Animal) };
    dog_obj.name();
    dog_obj.sound();
    dog_obj.parent.name();
}

/**
This is the main function.
*/
fn main() {
    println!("Hello World.");
    println!("This is a second line.");

    let x: i32 = 5;
    println!("x: {}", x);
    println!("2x: {}", 2 * x);

    let vec: VectorStruct = VectorStruct { a: 10, b: 20 };
    println!("a: {}", vec.a);
    println!("b: {}", vec.b);

    let vec_op: VectorOperation = VectorOperation {
        vec_sum: VectorStruct { a: (12), ..vec },
        vec_mul: VectorStruct { a: (25), b: (35) },
    };
    // if you want to use the elements of vec, then use: ... vec_sum:  VectorStruct{..vec}, ...

    let sum_val: i32 = sum_function(vec_op.vec_sum);
    println!("vec: {:?}", vec);
    println!("sum_val: {:?}", sum_val);

    let color: Colors = Colors {
        red: (128),
        green: (255),
        blue: (90),
    };
    println!("The color is: {:?}", color);
    println!("The color in hex is: {:X?}", color);
    println!(
        "RGB ({},{},{}) 0x{:X?}{:X?}{:X?}",
        color.red, color.green, color.blue, color.red, color.green, color.blue
    );

    let tuple_obj: (bool, i64) = (true, 12);
    println!("Tuple is: {:?}", reverse(tuple_obj));

    let matrix_obj: (i32, i32, i32, i32) = (1, 2, 3, 4);
    // getting the length of a tuple doesn't exist in Rust
    println!(
        "Transpose of a matrix is: {:?}",
        transpose_matrix(matrix_obj)
    );

    let array_x: [i32; 5] = [2, 3, 9, 8, 32];
    let val_to_match: i32 = 3;
    for i in 0..array_x.len() {
        match array_x.get(i) {
            Some(val) => println!("Matched with value: {}", val),
            None => println!("Didn't match!"),
        }
    }

    /*
    // WONT WORK with MATCH
    for i in 0..array_x.len()
    {
        match array_x.get(i)
        {
            val_to_match => println!("Matched with {:?}", val_to_match),
            _ => println!("Didn't match {:?}", array_x.get(i)),
        }
    }
    https://stackoverflow.com/questions/49886160/why-can-i-compare-a-string-to-a-str-using-if-but-not-when-using-match/49889545#49889545
    */
    for i in 0..array_x.len() {
        match val_to_match {
            _ if val_to_match == array_x[i] => {
                println!("Found {:?} at position {:?} of the array", val_to_match, i)
            }
            _ => println!("Not found"),
        }
    }

    let point1: Point = Point {
        x_coordinate: (12.1),
        y_coordinate: (45.3),
    };
    let point2: Point = Point {
        x_coordinate: (22.2),
        y_coordinate: (15.4),
    };

    let rect: Rectangle = Rectangle {
        top_left: (point1),
        bottom_right: (point2),
    };
    println!("Rectangle: {:?}", rect);
    let rect_area: f32 = rect_area(rect);
    println!("Area of the rectange is {:?}", rect_area);

    let mut mut_var: i8 = 1; // variables are not mutable by default
    mut_var += 1;
    println!("mutable variable is: {:?}", mut_var);

    // let shadow_bind:i32 = 1;
    // let shadow_bind:i32 = 2;    // variable shadowing

    // println!("Shadow bind value: {:?}", shadow_bind);

    // converting a string
    let radius: i32 = 10;
    println!("Radius is: {:?}", radius.to_string());

    let radius_string: &str = "6";
    let radius_int: i32 = radius_string.parse::<i32>().unwrap();
    println!("Radius squared: {:?}", radius_int * radius_int);

    // looping
    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("Started count = {:?}", count);
        if count == 3 {
            println!("Inside loop count = 3");
            continue; // will exit the loop here and start with count = 4
        }
        println!("Reached count = {:?}", count);

        if count == 5 {
            println!("Inside loop count = 5");
            break; // will exit the loop and stop
        }
        println!("Finished count = {:?}", count);
    }

    count = 0;
    'outer: loop {
        println!("Inside outer loop");
        count += 1;

        println!("Point 0 for count : {:?}", count);

        'inner: loop {
            count += 1;
            println!("Inside inner loop");

            println!("Point 1 for count : {:?}", count);

            if count == 3 {
                println!("Breaking inner loop for count = 3");
                break 'inner;
            }

            println!("Point 2 for count : {:?}", count);

            if count == 5 {
                println!("Breaking outer loop for count = 5");
                break 'outer;
            }
            println!("Point 3 for count : {:?}", count);
        }

        println!("Point 4 for count : {:?}", count);
    }
    println!("Broken the outer loop and returned to main");

    referenceFunction();

    classFunction();

    test_module::print_test();
    test_module::sub_test::sub_testing_function();      // to use sub_test module, write the sub_test module as pub in test_module.rs
}
