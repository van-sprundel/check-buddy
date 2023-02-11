const MIN: i32 = -1000;
const MAX: i32 = 1000;
const MAX_DEPTH: usize = 3;

fn main() {
    let values = &mut vec![3, 5, 6, 9, 1, 2, 0, -1];
    println!("Optimal value is {}", minimax(0, 0, true, values, MIN, MAX))
}

fn minimax(
    depth: usize,
    node_index: usize,
    maximizing_player: bool,
    values: &mut Vec<i32>,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if depth == MAX_DEPTH {
        return values[node_index];
    }

    return if maximizing_player {
        let mut best = MIN;
        for i in 0..2 {
            let value = minimax(depth + 1, node_index * 2 + i, false, values, alpha, beta);
            best = std::cmp::max(best, value);
            alpha = std::cmp::max(alpha, best);

            if beta <= alpha {
                break;
            }
        }
        best
    } else {
        let mut best = MAX;
        for i in 0..2 {
            let value = minimax(depth + 1, node_index * 2 + i, true, values, alpha, beta);
            best = std::cmp::min(best, value);
            beta = std::cmp::min(beta, best);

            if beta <= alpha {
                break;
            }
        }
        best
    };
}
