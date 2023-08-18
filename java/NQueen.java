import java.util.ArrayList;
import java.util.List;

public class NQueen {
    public static List<List<String>> solveQueen(int n) {
        final List<List<String>> combs = new ArrayList<>();
        final char[][] board = new char[n][n];

        // Board intialization
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                board[i][j] = '.';
            }
        }

        NQueen.backtrack(combs, board, 0);
        return combs;
    }

    private static void backtrack(
            List<List<String>> combs,
            char[][] board,
            int row) {

        if (row == board.length) {
            combs.add(NQueen.bulid_board(board));
            return;
        }

        for (int col = 0; col < board.length; col++) {
            if (NQueen.is_valid_pos(board, row, col)) {
                board[row][col] = 'Q';
                NQueen.backtrack(combs, board, row + 1);
                board[row][col] = '.';
            }
        }
    }

    private static List<String> bulid_board(char[][] board) {
        final List<String> sol = new ArrayList<>();
        for (char[] v : board) {
            sol.add(String.valueOf(v, 0, board.length));
        }
        return sol;
    }

    private static boolean is_valid_pos(char[][] board, int row, int col) {
        // Vertical
        for (int i = 0; i < row; i++)
            if (board[i][col] == 'Q')
                return false;

        // Left diagonal (\)
        for (int i = 1; i <= Math.min(row, col); i++)
            if (board[row - i][col - i] == 'Q')
                return false;

        // Right diagonal (/)
        for (int i = 1; i <= Math.min(row, board.length - 1 - col); i++)
            if (board[row - i][col + i] == 'Q')
                return false;

        return true;
    }

    public static void main(String[] args) {
        NQueen.solveQueen(4).forEach(x -> {
            x.forEach(System.out::println);
            System.out.println();
        });
    }
}
