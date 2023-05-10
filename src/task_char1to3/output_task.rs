// Задачи из книги 1400 задач по программированию c 1.1 по 1.17
pub fn msg_numbers_1() {
    // Вывести на одной строке числа 31, 18 и 79 с одним про-
    // белом между ними. Текст '31 18 79' не использовать.
    println!("{} {} {}\n", 31, 18, 79);
}

pub fn msg_numbers_2() {
    //Вывести на одной строке числа 47, 52 и 150 с двумя про-
    // белами между ними. Текст '47 52 150' не использовать.
    println!("{} {} {}\n", 47, 52, 150);
}

pub fn msg_numbers_3() {
    //Вывести на экран числа 50 и 10 одно под другим.
    println!("{}\n{}", 50, 10);
}

pub fn msg_numbers_4() {
    // Вывести на экран числа 5, 10 и 21 одно под другим.
    println!("{}\n{}\n{}\n", 5, 10, 21);
}

pub fn msg_numbers_5() {
    // Получить на экране следующее:
    // 1
    // 2
    println!("{}\n\n{}", 1, 2);
}

pub fn msg_numbers_6() {
    //Число π примерно равно 3,1415926. Вывести на экран это
    // число с тремя цифрами в дробной части. Текст '3.142' не исполь-
    // зовать.
    println!("{:.3}", std::f32::consts::PI);
}

pub fn msg_numbers_7() {
    // Число e (основание натурального логарифма) приблизи-
    // тельно равно 2,71828. Вывести на экран это число с точностью
    // до десятых. Текст '2.7' не использовать.
    println!("{:.1}", std::f32::consts::E);
}

pub fn output_numbers_1() {
    //Составить программу вывода на экран числа, вводимого
    // с клавиатуры. Выводимому числу должно предшествовать со-
    // общение «Вы ввели число».
    println!("Введите число");
    let mut input_n = String::new();
    std::io::stdin().read_line(&mut input_n).ok().expect("Ошибка ввода");
    let num: i32 = input_n.trim().parse().unwrap();
    println!("Вы ввели число: {}", num);
}

pub fn output_numbers_2() {
    //Составить программу вывода на экран числа, вводимого
    // с клавиатуры. После выводимого числа должно следовать сообще-
    // ние «– вот какое число Вы ввели».
    println!("Введите число");
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).ok().expect("Ошибка ввода");
    let convert_num: i32 = num.trim().parse().unwrap();
    println!("{} – вот какое число Вы ввели", convert_num);
}

pub fn output_name() {
    // Составить программу, которая запрашивает имя человека
    // и повторяет его на экране
    println!("Введите ваше имя:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).ok().expect("Ошибка ввода");
    println!("{}", name);
}

pub fn output_soccer_command() {
    //Составить программу, которая запрашивает название
    // футбольной команды и повторяет его на экране со словами «–
    // это чемпион!»
    println!("Введите название футбольной команды:");
    let mut soccer_command = String::new();
    std::io::stdin().read_line(&mut soccer_command).ok().expect("Ошибка ввода");
    println!("{} - это чемпион!", soccer_command);
}

pub fn output_hello_name() {
    // Напишите программу, в которую вводится имя человека
    // и выводится на экран приветствие в виде слова «Привет», после
    // которого должна стоять запятая, введенное имя и восклицатель-
    // ный знак. После запятой должен стоять пробел, а перед воскли-
    // цательным знаком пробела быть не должно
    println!("Введите ваше имя: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).ok().expect("Ошибка ввода");
    println!("Привет, {}!", name);
}

pub fn next_number_lats() {
    //Напишите программу, в которую вводится целое число,
    // после чего на экран выводится следующее и предыдущее целое
    // число. Например, при вводе числа 15 на экран должно быть вы-
    // ведено:
    // Следующее за числом 15 число – 16.
    // Для числа 15 предыдущее число – 14.
    println!("Введите число:");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).ok().expect("Ошибка ввода");
    let num: i32 = number.trim().parse().unwrap();
    println!("Следующее за числом {} число – {}", num, num+1);
    println!("Для числа {} предыдущее число – {}.", num, num-1);
}

pub fn output_line_number_3(){
    //Составить программу вывода на экран в одну строку трех
    // любых чисел, вводимых с клавиатуры, с двумя пробелами между
    // ними.
    let mut first_num = String::new();
    let mut second_num = String::new();
    let mut three_num = String::new();
    println!("Введите первое число:");
    std::io::stdin().read_line(&mut first_num).ok().expect("Ошибка ввода");
    println!("Введите второе число:");
    std::io::stdin().read_line(&mut second_num).ok().expect("Ошибка ввода");
    println!("Введите третие число:");
    std::io::stdin().read_line(&mut three_num).ok().expect("Ошибка ввода");
    let fs_num: i32 = first_num.trim().parse().unwrap();
    let sd_num: i32 = second_num.trim().parse().unwrap();
    let th_num: i32 = three_num.trim().parse().unwrap();
    println!("Вы ввели следующие числа: {} {} {}", fs_num, sd_num, th_num);

}

pub fn output_line_number_4(){
    //Составить программу вывода на экран в одну строку че-
    // тырех любых чисел, вводимых с клавиатуры, с одним пробелом
    // между ними.
    let mut first_num = String::new();
    let mut second_num = String::new();
    let mut three_num = String::new();
    let mut four_num = String::new();
    println!("Введите первое число:");
    std::io::stdin().read_line(&mut first_num).ok().expect("Ошибка ввода");
    println!("Введите второе число:");
    std::io::stdin().read_line(&mut second_num).ok().expect("Ошибка ввода");
    println!("Введите третие число:");
    std::io::stdin().read_line(&mut three_num).ok().expect("Ошибка ввода");
    println!("Введите четвертое число:");
    std::io::stdin().read_line(&mut four_num).ok().expect("Ошибка ввода");
    let fs_num: i32 = first_num.trim().parse().unwrap();
    let sd_num: i32 = second_num.trim().parse().unwrap();
    let th_num: i32 = three_num.trim().parse().unwrap();
    let fr_num: i32 = four_num.trim().parse().unwrap();
    println!("Вы ввели следующие числа: {} {} {} {}", fs_num, sd_num, th_num, fr_num);
}

pub fn output_choice(){
    let mut x_str = String::new();
    let mut y_str = String::new();
    let mut t_str = String::new();
    let mut v_str = String::new();
    println!("Введите число x:");
    std::io::stdin().read_line(&mut x_str).ok().expect("Ошибка ввода");
    println!("Введите число y");
    std::io::stdin().read_line(&mut y_str).ok().expect("Ошибка ввода");
    println!("Введите число t");
    std::io::stdin().read_line(&mut t_str).ok().expect("Ошибка ввода");
    println!("Введите число v");
    std::io::stdin().read_line(&mut v_str).ok().expect("Ошибка ввода");
    let x: i32 = x_str.trim().parse().unwrap();
    let y: i32 = y_str.trim().parse().unwrap();
    let t: i32 = t_str.trim().parse().unwrap();
    let v: i32 = v_str.trim().parse().unwrap();
    println!("a) 5 10      б) 100 {}     в) {} 25", t, x);
    println!("   7 см         1949 {}       {} {}", v, x, y);
}
