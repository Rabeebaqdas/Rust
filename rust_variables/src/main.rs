fn main() {
    let mut a = 5;
    a = 2;
    println!("Output : {}", a);
    let b = 5;
    println!("Output : {}", b);
    let b = "six";
    println!("Output : {}", b);
    const CONSTANT:u32 = 10_000_000;
    println!("Output : {}", CONSTANT);

    let tup = ("hello", 12_000);
    let (val1, val2) = tup;
    println!("Output : {}", b);
    println!("Output : {}, {}", val1, val2);

    let error_codes = [200,404,500];
    let error_code = error_codes[2];
    println!("Output : {}", error_code);

    let preDefinedArray = [2; 8];
    println!("Output : {}", preDefinedArray[1]);
    second_function();
    let sum = third_function(2,4);    
    println!("Output : {}", sum);


}

fn second_function() {
    println!("Hello from second function");
}


// fn third_function(x:i32, y:i32) -> i32 {
//     println!("Output : {}", x);
//     println!("Output : {}", y);
//     let sum = x + y;
//     return sum;
// }


fn third_function(x:i32, y:i32) -> i32 {
    println!("Output : {}", x);
    println!("Output : {}", y);
    x + y
}