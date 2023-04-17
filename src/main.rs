use std::io;
fn main() {
    let arr = read_array();
    let sum = sum_array(&arr);
    println!("Сумма элементов массива: {}", sum);
    let mut close = String::new();
    println!("Нажмите Enter для закрытия программы");
    io::stdin().read_line(&mut close).unwrap();
}
fn read_array() -> Vec<i32> {
    
    let mut input_length = String::new();
    println!("Введите длину массива:");
    match io::stdin().read_line(&mut input_length){
        Ok(_) => {

        },
        Err(e) => {
            println!("Ошибка: {}", e);
        }
    }
    let length: usize = input_length.trim().parse().unwrap();

    
    let mut array = Vec::with_capacity(length);
    println!("Введите элементы массива, каждый с новой строки:");
    for i in 0..length {
        let mut input_item = String::new();
        match io::stdin().read_line(&mut input_item){
            Ok(_) => {

            },
            Err(e) => {
                println!("Ошибка: {}", e);
            }
        }
        let item: i32 = input_item.trim().parse().unwrap();
        array.push(item);
    }

    array
}

fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;

    for &num in arr {
        sum += num;
    }

    sum
}



    
#[cfg(test)]
mod test {
    use super::sum_array;

    #[test]
    fn test_sum_array() {
        let arr = vec![1, 2, 3, 4, 5];
        assert_eq!(sum_array(&arr), 15);
    }
}
