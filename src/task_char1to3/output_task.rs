pub fn msg_numbers_1(){
    // Вывод числе 31, 18, 79, с одним пробелом между ними
          println!("{} {} {}\n", 31, 18, 79);
}

pub fn msg_numbers_2(){
    println!("{} {} {}\n", 47, 52, 150);
}

pub fn msg_numbers_3(){
    println!("{}\n{}", 50, 10);
}

pub fn msg_numbers_4(){
    println!("{}\n{}\n{}\n", 5, 10, 21);
}

pub fn msg_numbers_5(){
    println!("{}\n\n{}", 1, 2);
}

pub fn msg_numbers_6(){
    //Выводит до число пи до 3 знака
    println!("{:.3}", std::f32::consts::PI);
}

pub fn msg_numbers_7(){
    println!("{:.1}", std::f32::consts::E);
}

pub fn output_numbers_1() {
    println!("Введите число");
    let mut input_n = String::new();
    match std::io::stdin().read_line(&mut input_n) {
        Ok(_) => {}
        Err(_e) =>{
            println!("Не корректный ввод!");
        }
    }
    let num: i32 = input_n.trim().parse().unwrap();
    println!("Вы ввели число: {}", num);
}

pub fn output_numbers_2(){
    println!("Введите число");
    let mut num = String::new();
    match std::io::stdin().read_line(&mut num) {
        Ok(_) => {}
        Err(_e) => {
            println!("Не корректны ввод!");
        }
    }
    let convert_num: i32 = num.trim().parse().unwrap();
    println!("{} – вот какое число Вы ввели", convert_num);
}

pub fn output_name(){
    println!("Введите ваше имя:");
    let mut name = String::new();
    match std::io::stdin().read_line(&mut name) {
        Ok(_) => {}
        Err(_e) => {
            println!("Ошибка ввода");
        }
    }
    println!("{}", name);
}

pub fn output_soccer_command(){
    println!("Введите название футбольной команды:");
    let mut soccer_command = String::new();
    match std::io::stdin().read_line(&mut soccer_command) {
        Ok(_) => {}
        Err(_e) => {
            println!("Ошибка ввода");
        }
    }
    println!("{} - это чемпион!", soccer_command);
}