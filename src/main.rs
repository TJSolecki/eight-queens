use std::collections::HashSet;

fn print_board(board: &Vec<Vec<u8>>) {
    for row in 0..8 {
        println!("{:?}", board[row]);
    }
}

fn has_conflict(queen_positions: &Vec<(usize, usize)>, new_row: usize, new_col: usize) -> bool {
    for (queen_row, queen_col) in queen_positions.iter() {
        if *queen_row == new_row {
            return true;
        }
        if *queen_col == new_col {
            return true;
        }
        if new_col.abs_diff(*queen_col) == new_row.abs_diff(*queen_row) {
            return true;
        }
    }
    return false;
}

fn solve_8_queens(
    board: &mut Vec<Vec<u8>>,
    queen_positions: &mut Vec<(usize, usize)>,
    used_rows: &mut HashSet<usize>,
    used_cols: &mut HashSet<usize>,
) -> bool {
    if queen_positions.len() >= 8 {
        print_board(board);
        return true;
    }

    for row in 0..8 {
        if used_rows.contains(&row) {
            continue;
        }

        for col in 0..8 {
            if used_cols.contains(&col) {
                continue;
            }

            if !has_conflict(queen_positions, row, col) {
                board[row][col] = 1;
                queen_positions.push((row, col));
                used_rows.insert(row);
                used_cols.insert(col);
                let solved = solve_8_queens(board, queen_positions, used_rows, used_cols);
                if solved {
                    return true;
                }
            }
        }
    }

    let (row_to_remove, col_to_remove) = queen_positions.pop().unwrap();
    board[row_to_remove][col_to_remove] = 0;
    used_rows.remove(&row_to_remove);
    used_cols.remove(&col_to_remove);
    return false;
}

fn main() {
    let mut board: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let mut used_rows = HashSet::<usize>::new();
    let mut used_cols = HashSet::<usize>::new();
    let mut queen_positions: Vec<(usize, usize)> = vec![];
    solve_8_queens(
        &mut board,
        &mut queen_positions,
        &mut used_rows,
        &mut used_cols,
    );
}
