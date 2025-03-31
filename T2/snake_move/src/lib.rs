use wasm_bindgen::prelude::*;

#[wasm_bindgen]
struct Pos {
    pub x: i32,
    pub y: i32,
    pub dir: i32,
}

#[wasm_bindgen]
pub fn snake_BFS(src: &Pos, dst: &Pos, bar: &Vec<Pos>, queue: &Vec<Pos>) -> i32 {
    if dst.x == src.x && dst.y == src.y {
        return dir
    } else {
        if !bar.iter().any(|&p| p.x == src.x && p.y == src.y + 1) {
            let near = Pos{x: src.x, y: src.y + 1, dir: 0};
            queue.add(near);
        }
        let new_src = queue.pop();
        snake_BFS(new_src, dst, bar, queue);
    }
}

#[wasm_bindgen]
pub fn greedySnakeMoveBarriers(spos: &[i32], apos: &[i32], bpos: &[i32]) -> i32 {
    let x = [apos[0], spos[0], spos[2], spos[4], spos[6]];
    let y = [apos[1], spos[1], spos[3], spos[5], spos[7]];
    let bx = [bpos[0], bpos[2], bpos[4], bpos[6], bpos[8], bpos[10], bpos[12], bpos[14], bpos[16], bpos[18], bpos[20], bpos[22]];
    let by = [bpos[1], bpos[3], bpos[5], bpos[7], bpos[9], bpos[11], bpos[13], bpos[15], bpos[17], bpos[19], bpos[21], bpos[23]];
    let mut flag = [1, 1, 1, 1];
    let mut order = [0, 0, 0, 0];
    for i in 2..4 {
        if x[i] == x[1] {
            if y[i] == y[1] + 1 {
                flag[0] = 0;
            } else if y[i] == y[1] - 1 {
                flag[2] = 0;
            }
        } else if y[i] == y[1] {
            if x[i] == x[1] + 1 {
                flag[3] = 0;
            } else if x[i] == x[1] - 1 {
                flag[1] = 0;
            }
        }
    }
    for i in 0..11 {
        if (bx[i] == x[1]) && (by[i] == y[1] + 1) {
            flag[0] = 0;
        } else if (bx[i] == x[1] - 1) && (by[i] == y[1]) {
            flag[1] = 0;
        } else if (bx[i] == x[1]) && (by[i] == y[1] - 1) {
            flag[2] = 0;
        } else if (bx[i] == x[1] + 1) && (by[i] == y[1]) {
            flag[3] = 0;
        }
    }
    if x[1] > x[0] {
        order[0] = 1;
        order[3] = 3;
        if y[1] >= y[0] {
            order[1] = 2;
            order[2] = 0;
        } else {
            order[1] = 0;
            order[2] = 2;
        }
    } else if x[1] < x[0] {
        order[0] = 3;
        order[3] = 1;
        if y[1] >= y[0] {
            order[1] = 2;
            order[2] = 0;
        } else {
            order[1] = 0;
            order[2] = 2;
        }
    } else {
        order[1] = 3;
        order[2] = 1;
        if y[1] >= y[0] {
            order[0] = 2;
            order[3] = 0;
        } else {
            order[0] = 0;
            order[3] = 2;
        }
    }
    for i in 0..3 {
        if flag[order[i]] > 0 {
            return order[i] as i32
        }
    }
    return -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_test_when_left() {
        assert_eq!(greedy_snake_move(&[4,4,4,5,4,6,4,7], &[1,1]), 1);
        // assert_eq!(greedy_snake_move(&[4,4,4,5,4,6,4,7], &[1,1]), 3);
    }

    #[test]
    fn func_test_when_right() {
        assert_eq!(greedy_snake_move(&[4,4,4,5,4,6,4,7], &[8,1]), 3);
        // assert_eq!(greedy_snake_move(&[4,4,4,5,4,6,4,7], &[8,1]), 1);
    }

    #[test]
    fn func_test_when_up() {
        assert_eq!(greedy_snake_move(&[4,4,4,3,4,2,4,1], &[4,8]), 0);
        // assert_eq!(greedy_snake_move(&[4,4,4,3,4,2,4,1], &[4,8]), 2);
    }
    
    #[test]
    fn func_test_when_down() {
        assert_eq!(greedy_snake_move(&[4,4,4,5,4,6,4,7], &[4,1]), 2);
        // assert_eq!(greedy_snake_move(&[4,4,4,5,4,6,4,7], &[4,1]), 0);
    }
}
