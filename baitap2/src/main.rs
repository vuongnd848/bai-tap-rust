use std::io;

fn main() -> io::Result<()> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    // Chuỗi ký tự
    let mut input = String::from("adbcdaDd");

    // Nhập từ bàn phím 1 ký tự
    println!("nhập vào 1 ký tự: ");
    stdin.read_line(&mut user_input);
    println!("Ký tự vừa nhập là: {} ", user_input);

    // Xử lý
    let mut input_check = &input.to_lowercase();
    let user_input_check = &user_input.to_lowercase();
    // println!("{} {}", input_check, user_input_check);

    let mut count = 0;
    for (i, c) in input_check.chars().enumerate() {
        match c {
            user_input_check => {
                print!("{} ", c);
                input.remove(i);
            }
        }
    }

    println!("{} {}", count, input);

    Ok(())
}
