fn main() {
    let num1 = 10;
    let num2 = 20;
    let result = get_ref(&num1, &num2);
    println!("{}", result);
}

fn get_ref<'a>(param: &'a i32, param2: &'a i32) -> &'a i32 {
    if param > param2 {
        param
    } else {
        param2
    }
}
