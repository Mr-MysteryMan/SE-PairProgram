mod lib;
use lib::greedy_snake_step;


fn main() {
    greedy_snake_step(
        5,
        &[1,4,1,3,1,2,1,1], 
        1,
        &[5,4,5,3,5,2,5,1],
        5,
        &[2,1,1,5,2,4,3,5,5,5],                              
        10
    );
}