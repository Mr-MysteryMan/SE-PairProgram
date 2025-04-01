use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub dir: i32,
}

#[wasm_bindgen]
pub fn snake_BFS(src: Pos, dst: Pos, bar: Vec<Pos>, mut queue: Vec<Pos>, mut visited: Vec<Pos>) -> i32 {
    if dst.x == src.x && dst.y == src.y {
        return src.dir
    } else {
        visited.push(src);
        if !bar.iter().any(|p| p.x == src.x && p.y == src.y - 1) 
        && !visited.iter().any(|p| p.x == src.x && p.y == src.y - 1) {
            let near = Pos{x: src.x, y: src.y - 1, dir: 0};
            queue.push(near);
        } 
        if !bar.iter().any(|p| p.x == src.x + 1 && p.y == src.y) 
        && !visited.iter().any(|p| p.x == src.x + 1 && p.y == src.y) {
            let near = Pos{x: src.x + 1, y: src.y, dir: 1};
            queue.push(near);
        } 
        if !bar.iter().any(|p| p.x == src.x && p.y == src.y + 1) 
        && !visited.iter().any(|p| p.x == src.x && p.y == src.y + 1) {
            let near = Pos{x: src.x, y: src.y + 1, dir: 2};
            queue.push(near);
        } 
        if !bar.iter().any(|p| p.x == src.x - 1 && p.y == src.y) 
        && !visited.iter().any(|p| p.x == src.x - 1 && p.y == src.y) {
            let near = Pos{x: src.x - 1, y: src.y, dir: 3};
            queue.push(near);
        }
        if queue.len() > 0 {
            let new_src = queue.remove(0);
            let new_dir = snake_BFS(new_src, dst, bar, queue, visited);
            return new_dir
        } else {
            return -1
        }
    }
}

#[wasm_bindgen]
pub fn greedySnakeMoveBarriers(spos: &[i32], apos: &[i32], bpos: &[i32]) -> i32 {
    let src = Pos{x: apos[0], y: apos[1], dir: -1};
    let dst = Pos{x: spos[0], y: spos[1], dir: -1};
    let mut bar: Vec<Pos> = Vec::new();
    for i in 1..4 {
        let barrier = Pos{x: spos[2 * i], y: spos[2 * i + 1], dir: -1};
        bar.push(barrier);
    }
    for i in 0..12 {
        let barrier = Pos{x: bpos[2 * i], y: bpos[2 * i + 1], dir: -1};
        bar.push(barrier);
    }
    for i in 0..8 {
        let barrier = Pos{x: 0, y: i + 1, dir: -1};
        bar.push(barrier);
        let barrier = Pos{x: i + 1, y: 0, dir: -1};
        bar.push(barrier);
        let barrier = Pos{x: 9, y: i + 1, dir: -1};
        bar.push(barrier);
        let barrier = Pos{x: i + 1, y: 9, dir: -1};
        bar.push(barrier);
    }
    let queue: Vec<Pos> = Vec::new();
    let visited: Vec<Pos> = Vec::new();
    let result = snake_BFS(src, dst, bar, queue, visited);
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
