fn main() {
    println!("Hello, world!");
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x+1
    }; // expressions do not end with a semicolon!


    println!("print five function return value: {}", five());

    println!("print plus_one: {}", plus_one(y));

    println!("print loop_result: {}", loop_result());
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn loop_result() -> u32 {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2;
        }
    }; // result will be counter*2=20

    result
}

fn nested_loop_result() {
    let mut count: u32 = 0;

    // use this weird single single quote for counting up.
    'counting_up: loop{
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // breaks the outter loop
            }
            remaining -= 1;
        }
        count += 1;
    };
    
}