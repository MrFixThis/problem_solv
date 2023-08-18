pub fn solve_queen<const N: usize>() -> Vec<Vec<String>> {
    let mut combs: Vec<Vec<String>> = Vec::new();
    let mut board = [[b'.'; N]; N];
    backtrack(&mut combs, &mut board, 0);
    combs
}

fn backtrack<const N: usize>(combs: &mut Vec<Vec<String>>, board: &mut [[u8; N]; N], row: usize) {
    if row == N {
        return combs.push(build_board(board));
    }

    (0..N).for_each(|col| {
        if is_valid_pos(board, row, col) {
            board[row][col] = b'Q';
            backtrack(combs, board, row + 1);
            board[row][col] = b'.';
        }
    });
}

#[inline]
fn build_board<const N: usize>(board: &[[u8; N]; N]) -> Vec<String> {
    board
        .iter()
        .map(|&v| String::from_utf8_lossy(&v).into_owned())
        .collect()
}

fn is_valid_pos<const N: usize>(board: &[[u8; N]; N], row: usize, col: usize) -> bool {
    for i in 0..row {
        if board[i][col] == b'Q' {
            return false;
        }
    }

    for i in 1..=row.min(col) {
        if board[row - i][col - i] == b'Q' {
            return false;
        }
    }

    for i in 1..=row.min(N - 1 - col) {
        if board[row - i][col + i] == b'Q' {
            return false;
        }
    }

    true
}
