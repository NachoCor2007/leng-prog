fn main() {
    println!("Hello, world!");
}

fn swap<'a, T: Copy> (x: &'a mut T, y: &'a mut T) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
