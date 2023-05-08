mod pr1;
mod msg_prev;
use crate::msg_prev::info::msg_point_task;
use crate::pr1::{
    one_dot_1::msg_number_one,
    one_dot_2::msg_numbers_2,
    one_dot_3::msg_number_3,
};

fn main() {
    msg_point_task(1.1);
    msg_number_one();
    msg_point_task(1.2);
    msg_numbers_2();
    msg_point_task(1.3);
    msg_number_3();
}
