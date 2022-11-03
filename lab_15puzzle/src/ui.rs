use ux::u4;

pub fn print_field(arr: [u4;16]) {

    let mut vertical = "|".to_string();
    vertical.push_str(&" ".repeat(11));
    vertical = vertical.repeat(4);
    vertical.push_str("|\n");
    let mut vertical_with_line = "|".to_string();
    vertical_with_line.push_str(&"_".repeat(11));

    
    print!(" ");
    println!("{}","_".repeat(47));
    for i in 0..4 {
        print!("{}",vertical.repeat(2));
    for j in 0..4 {

        if arr[i*4+j] == u4::new(0) {
            print!("|{}"," ".repeat(11));
        }
        else if arr[i*4+j] < u4::new(10) {
        print!("|{}{}{}"," ".repeat(5),arr[i*4+j]," ".repeat(5));
        } else {
            print!("|{}{}{}"," ".repeat(5),arr[i*4+j]," ".repeat(4));
        }
    }
    println!("|");
    print!("{}",vertical);
    println!("{}|",vertical_with_line.repeat(4));
    }
}