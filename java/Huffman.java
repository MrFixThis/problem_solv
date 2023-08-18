import java.util.ArrayDeque;
import java.util.Deque;

public class Huffman {
    private static class Node {
        int frequency;
        char data;
        Node left;
        Node right;

        public Node(int frequency, char data, Node left, Node right) {
            this.frequency = frequency;
            this.data = data;
            this.left = left;
            this.right = right;
        }
    }

    public static void decode(String s, Node root) {
        final StringBuilder txt = new StringBuilder();
        final Deque<Character> codes = new ArrayDeque<>();
        for (int i = 0; i < s.length(); i++)
            codes.addLast(s.charAt(i));

        while (!codes.isEmpty()) {
            traverse(codes, txt, root);
        }

        System.out.println(txt.toString());
    }

    private static void traverse(
            Deque<Character> codes,
            StringBuilder txt,
            Node root) {
        if (root == null) {
            return;
        }

        if (root.data != '\0') {
            txt.append(root.data);
            return;
        }

        if (codes.removeFirst().equals('0')) {
            traverse(codes, txt, root.left);
        } else {
            traverse(codes, txt, root.right);
        }
    }

    public static void main(String[] args) {
        final String codes = "1001011";
        final Node root = new Node(
                5,
                '\0',
                new Node(
                        2,
                        '\0',
                        new Node(1, 'B', null, null),
                        new Node(1, 'C', null, null)),
                new Node(3, 'A', null, null));

        Huffman.decode(codes, root);
    }
}
