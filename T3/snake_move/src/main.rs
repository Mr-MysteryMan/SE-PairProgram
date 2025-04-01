mod lib;
use lib::greedy_snake_step;


fn main() {
    let dir = greedy_snake_step(
        8,
        &[4,4,3,4,2,4,2,3], 
        1,
        &[1,5,1,4,1,3,1,2,6,4,6,3,6,2,6,1,7,8,7,7,7,6,7,5],
        5,
        &[2,2,3,2,4,1,5,3,5,4],                              
        10
    );
    println!("dir: {}", dir);
}