use std::io;

fn main() {
    let mut sum = 0;

    for i in 0..10{
        let mut c_str = String::new();
        match io::stdin().read_line(&mut c_str){
            Ok(_) => {

            },
            Err(e) => {
                println!("Ошибка: {}", e);
            }
        }
        
        let c: i32 = c_str.trim().parse().unwrap();
        sum += c;
    }
    println!("Сумма введенных чисел: {}", sum);
    let mut close = String::new();
    println!("Нажмите Enter для закрытия программы");
    io::stdin().read_line(&mut close).unwrap();
}
