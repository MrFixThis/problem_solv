public class BTree {
    private static class Node {
        int item;
        Node left;
        Node right;
    }

    public void preOrder(Node root) {
        if (root == null)
            return;

        System.out.printf("%s ", root.item);
        preOrder(root.left);
        preOrder(root.right);
    }

    public static void main(String[] args) {
        // rest of code...
    }
}
