mod task_char1to3;

use std::io;
mod msg_prev;
use crate::msg_prev::info::msg_point_task;
use crate::task_char1to3::output_task;

fn task_output() {
    println!("Введите решенную задачу которую хотите увидеть или нажмите ctrl+c чтобы завершить работу программы?");
    let mut choice_task = String::new();
    io::stdin().read_line(&mut choice_task).ok().expect("Ошибка ввода");
    let ch: usize = choice_task.trim().parse().unwrap();
    match ch {
        1 => {
            msg_point_task(1.1);
            output_task::msg_numbers_1();
        }
        2 => {
            msg_point_task(1.2);
            output_task::msg_numbers_2();
        }
        3 => {
            msg_point_task(1.3);
            output_task::msg_numbers_3()
        }
        4 => {
            msg_point_task(1.4);
            output_task::msg_numbers_4()
        }
        5 => {
            msg_point_task(1.5);
            output_task::msg_numbers_5();
        }
        6 => {
            msg_point_task(1.6);
            output_task::msg_numbers_6();
        }
        7 => {
            msg_point_task(1.7);
            output_task::msg_numbers_7();
        }
        8 => {
            msg_point_task(1.8);
            output_task::output_numbers_1();
        }
        9 => {
            msg_point_task(1.9);
            output_task::output_numbers_2();
        }
        10 => {
            msg_point_task(1.10);
            output_task::output_name();
        }
        11 => {
            msg_point_task(1.11);
            output_task::output_soccer_command();
        }
        12 =>{
            msg_point_task(1.12);
            output_task::output_hello_name();
        }
        13 => {
            msg_point_task(1.13);
            output_task::next_number_lats();
        }
        14 => {
            msg_point_task(1.14);
            output_task::output_line_number_3();
        }
        15 => {
            msg_point_task(1.15);
            output_task::output_line_number_4();
        }
        16 => {
            msg_point_task(1.16);
            output_task::output_choice();
        }
        _ => println!("Не понятный ввод"),
    }
}

fn main() {
    task_output();
}
