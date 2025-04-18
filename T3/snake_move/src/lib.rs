use wasm_bindgen::prelude::*;
use std::cmp;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub dir: i32,
    pub dis: i32,
}

#[wasm_bindgen]
pub fn add_barriers(n: i32, spos: &[i32], snum: i32, ospos: &[i32], 
    anum: i32, apos: &[i32]) -> Vec<Pos> {
    let mut bar: Vec<Pos> = Vec::new();
    for i in 1..4 {
        let barrier = Pos{x: spos[(2 * i) as usize], y: spos[(2 * i + 1) as usize], dir: -1, dis: -1};
        bar.push(barrier);
    }
    for i in 0..snum {
        let barrier = Pos{x: ospos[(8 * i) as usize], y: ospos[(8 * i + 1) as usize], dir: -1, dis: -1};
        bar.push(barrier);
        let barrier = Pos{x: ospos[(8 * i + 2) as usize], y: ospos[(8 * i + 3) as usize], dir: -1, dis: -1};
        bar.push(barrier);
        let barrier = Pos{x: ospos[(8 * i + 4) as usize], y: ospos[(8 * i + 5) as usize], dir: -1, dis: -1};
        bar.push(barrier);
        let barrier = Pos{x: ospos[(8 * i + 6) as usize], y: ospos[(8 * i + 7) as usize], dir: -1, dis: -1};
        bar.push(barrier);
    }
    for i in 0..n {
        let barrier = Pos{x: 0, y: i + 1, dir: -1, dis: -1};
        bar.push(barrier);
        let barrier = Pos{x: i + 1, y: 0, dir: -1, dis: -1};
        bar.push(barrier);
        let barrier = Pos{x: n + 1, y: i + 1, dir: -1, dis: -1};
        bar.push(barrier);
        let barrier = Pos{x: i + 1, y: n + 1, dir: -1, dis: -1};
        bar.push(barrier);
    }
    return bar
}

#[wasm_bindgen]
pub fn snake_BFS(src: Pos, dst: Pos, bar: Vec<Pos>, 
    mut queue: Vec<Pos>, mut visited: Vec<Pos>) -> Pos {
    if dst.x == src.x && dst.y == src.y {
        return src
    } else {
        //println!("{} {}", src.x, src.y);
        if !bar.iter().any(|p| p.x == src.x && p.y == src.y - 1) 
        && !visited.iter().any(|p| p.x == src.x && p.y == src.y - 1) {
            //println!("src: {} {} next: {} {}", src.x, src.y, src.x, src.y - 1);
            let near = Pos{x: src.x, y: src.y - 1, dir: 0, dis: src.dis + 1};
            queue.push(near);
            visited.push(near);
        } 
        if !bar.iter().any(|p| p.x == src.x + 1 && p.y == src.y) 
        && !visited.iter().any(|p| p.x == src.x + 1 && p.y == src.y) {
            //println!("src: {} {} next: {} {}", src.x, src.y, src.x + 1, src.y);
            let near = Pos{x: src.x + 1, y: src.y, dir: 1, dis: src.dis + 1};
            queue.push(near);
            visited.push(near);
        } 
        if !bar.iter().any(|p| p.x == src.x && p.y == src.y + 1) 
        && !visited.iter().any(|p| p.x == src.x && p.y == src.y + 1) {
            //println!("src: {} {} next: {} {} ", src.x, src.y, src.x, src.y + 1);
            let near = Pos{x: src.x, y: src.y + 1, dir: 2, dis: src.dis + 1};
            queue.push(near);
            visited.push(near);
        } 
        if !bar.iter().any(|p| p.x == src.x - 1 && p.y == src.y) 
        && !visited.iter().any(|p| p.x == src.x - 1 && p.y == src.y) {
            //println!("src: {} {} next: {} {} ", src.x, src.y, src.x - 1, src.y);
            let near = Pos{x: src.x - 1, y: src.y, dir: 3, dis: src.dis + 1};
            queue.push(near);
            visited.push(near);
        }
        if queue.len() > 0 {
            let new_src = queue.remove(0);
            let exp = snake_BFS(new_src, dst, bar, queue, visited);
            return exp
        } else {
            return Pos{x: 0, y: 0, dir: -1, dis: -1}
        }
    }
}

#[wasm_bindgen]
pub fn dir_nobar(src: Pos, bar: Vec<Pos>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if !bar.iter().any(|p| p.x == src.x && p.y == src.y + 1) {
        result.push(0);
    } 
    if !bar.iter().any(|p| p.x == src.x - 1 && p.y == src.y) {
        result.push(1);
    }
    if !bar.iter().any(|p| p.x == src.x && p.y == src.y - 1)  {
        result.push(2);
    } 
    if !bar.iter().any(|p| p.x == src.x + 1 && p.y == src.y)  {
        result.push(3);
    } 
    return result
}

#[wasm_bindgen]
pub fn dir_nosnake(src: Pos, snum: i32, snakes: Vec<Pos>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    if snum == 0 {
        result.push(0);
        result.push(1);
        result.push(2);
        result.push(3);
    } else {
        for i in 0..snum {
            let snake = *(&snakes[i as usize]);
            if !(snake.x == src.x + 1 && snake.y == src.y + 1) 
            && !(snake.x == src.x - 1 && snake.y == src.y + 1) 
            && !(snake.x == src.x && snake.y == src.y + 2) {
                result.push(0);
            }
            if !(snake.x == src.x - 1 && snake.y == src.y - 1) 
            && !(snake.x == src.x - 1 && snake.y == src.y + 1) 
            && !(snake.x == src.x - 2 && snake.y == src.y) {
                result.push(1);
            }
            if !(snake.x == src.x - 1 && snake.y == src.y - 1) 
            && !(snake.x == src.x + 1 && snake.y == src.y - 1) 
            && !(snake.x == src.x && snake.y == src.y - 2) {
                result.push(2);
            }
            if !(snake.x == src.x + 1 && snake.y == src.y + 1) 
            && !(snake.x == src.x + 1 && snake.y == src.y - 1) 
            && !(snake.x == src.x + 2 && snake.y == src.y) {
                result.push(3);
            }
        }
    }
    return result
}

#[wasm_bindgen]
pub fn cango(src: Pos, snum: i32, bar: Vec<Pos>, snakes: Vec<Pos>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let nobar = dir_nobar(src, bar);
    let nosnake = dir_nosnake(src, snum, snakes);
    for i in 0..4 {
        if nobar.iter().any(|&p| p == i) 
        && nosnake.iter().any(|&p| p == i) {
            result.push(i);
        } 
    }
    return result
}

#[wasm_bindgen]
pub fn react_to_nodir(dir: i32, can_dir: Vec<i32>) -> i32 {
    if dir == -1 {
        if can_dir.len() > 0 {
            for i in 0..4 {
                if can_dir.iter().any(|&p| p == i) {
                    return i
                }
            }
        }
        return 0;
    }
    return dir
}

#[wasm_bindgen]
pub fn greedy_snake_step(n: i32, spos: &[i32], snum: i32, ospos: &[i32], 
    anum: i32, apos: &[i32], round: i32) -> i32 {
    let mut bar = add_barriers(n, spos, snum, ospos, anum, apos);
    let mut queue: Vec<Pos> = Vec::new();
    let mut visited: Vec<Pos> = Vec::new();
    let mut scores: Vec<f32> = Vec::new();
    let mut actions: Vec<i32> = Vec::new();
    for i in 0..anum {
        let src = Pos{x: apos[(2 * i) as usize], y: apos[(2 * i + 1) as usize], dir: -1, dis: -1};
        let my_dst = Pos{x: spos[0], y: spos[1], dir: -1, dis: -1};
        let my_exp = snake_BFS(src, my_dst, bar.clone(), queue.clone(), visited.clone());
        queue.clear();
        visited.clear();
        actions.push(my_exp.dir);
        let my_score = my_exp.dis;
        let mut min_dis = 20;
        for j in 0..snum {
            let j_dst = Pos{x: ospos[(8 * j) as usize], y: ospos[(8 * j + 1) as usize], dir: -1, dis: -1};
            let j_exp = snake_BFS(src, j_dst, bar.clone(), queue.clone(), visited.clone());
            queue.clear();
            visited.clear();
            let m = min_dis.min(j_exp.dis);
            min_dis = m;
        }
        let score =  (20 - my_score) as f32 / 20.0 * 5.0 + (20 - min_dis) as f32 / 20.0 * 5.0;
        scores.push(score);
    }
    let mut result = -1;
    let mut max_score = -20.0;
    let my_pos = Pos{x: spos[0], y: spos[1], dir: -1, dis: -1};
    let mut snakes: Vec<Pos> = Vec::new();
    for i in 0..snum {
        let snake = Pos{x: ospos[(8 * i) as usize], y: ospos[(8 * i + 1) as usize], dir: -1, dis: -1};
        snakes.push(snake);
    }
    let can_dir = cango(my_pos, snum, bar.clone(), snakes);
    for i in 0..anum {
        let score_i = scores[i as usize];
        let action_i = actions[i as usize];
        if max_score < score_i && can_dir.iter().any(|&p| p == action_i) {
            max_score = score_i;
            result = action_i;
        }
    }
    let nobar = dir_nobar(my_pos, bar.clone());
    result = react_to_nodir(result, nobar);
    return result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_test_p3() {
        assert_eq!(greedySnakeMoveBarriers(
            &[1,4,1,3,1,2,1,1],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            3
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[2,4,1,4,1,3,1,2],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            3
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[3,4,2,4,1,4,1,3],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            2
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[3,3,3,4,2,4,1,4],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            2
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[3,2,2,4,1,4,1,3],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            3
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[4,2,3,2,2,4,1,4],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            3
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[5,2,4,2,3,2,2,4],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            3
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[6,2,5,2,4,2,3,2],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            0
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[6,3,6,2,5,2,4,2],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            0
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[6,4,6,3,6,2,5,2],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            0
        );
        assert_eq!(greedySnakeMoveBarriers(
            &[6,5,6,4,6,3,6,2],
            &[5,5],                              
            &[2,7,2,6,3,7,3,6,4,6,5,6,6,6,7,6,4,5,4,4,4,3,5,4]), 
            1
        );
    }

    #[test]
    fn func_test_p4() {
        assert_eq!(greedySnakeMoveBarriers(
            &[1,4,1,3,1,2,1,1], 
            &[1,7],                              
            &[2,7,2,6,3,7,3,6,4,7,4,6,5,7,5,6,1,6,6,6,7,6,8,6]), 
            -1
        );
    }
}
