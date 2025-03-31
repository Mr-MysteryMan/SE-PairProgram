mod lib;
use lib::greedySnakeMoveBarriers;


fn main() {
    greedySnakeMoveBarriers(
        &[1,4,1,3,1,2,1,1], 
        &[1,7],                              
        &[2,7,2,6,3,7,3,6,4,7,4,6,5,7,5,6,1,6,6,6,7,6,8,6]
    );
}