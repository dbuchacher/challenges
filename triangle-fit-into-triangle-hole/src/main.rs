/*
    Create a function that takes the dimensions of two triangles (as arrays) and checks if the first triangle fits into the second one. - https://edabit.com/challenge/7e2Aq87tDpW2CK7XH
*/

// using triangle inequality theorem to check if object is a triangle
fn is_triangle(triangle: &[f32; 3]) -> bool {
    triangle[0] + triangle [1] > triangle[2] &&
    triangle[0] + triangle [2] > triangle[1] &&
    triangle[1] + triangle [2] > triangle[0]
}

// using heron's formula to find area
fn area(triangle: &[f32; 3]) -> f32 {
    let p = triangle.iter().sum::<f32>()/2.0;
    (p*(p-triangle[0])*(p-triangle[1])*(p-triangle[2])).sqrt()
}

fn does_triangle_fit(peg: &[f32; 3], hole: &[f32; 3]) -> bool {
    is_triangle(peg) && is_triangle(hole) && area(peg) <= area(hole)
}

fn main() {
    println!("{}", does_triangle_fit(&[1.0, 1.0, 1.0], &[1.0, 1.0, 1.0]));
    println!("{}", does_triangle_fit(&[1.0, 1.0, 1.0], &[2.0, 2.0, 2.0]));
    println!("{}", does_triangle_fit(&[1.0, 2.0, 3.0], &[1.0, 2.0, 2.0]));
    println!("{}", does_triangle_fit(&[1.0, 2.0, 4.0], &[1.0, 2.0, 6.0]));
}
