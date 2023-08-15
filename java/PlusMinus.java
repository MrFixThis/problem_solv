import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class PlusMinus {
    private static void plus_sum(List<Integer> arr) {
        final int[] memo = {0, 0, 0};
        arr
            .stream()
            .forEach(x -> {
                if(x > 0)
                    ++memo[0];
                else if(x < 0)
                    ++memo[1];
                else
                    ++memo[2];
            });

        Arrays
            .stream(memo, 0, memo.length)
            .mapToDouble(x -> (double) x / arr.size())
            .forEach(x -> System.out.printf("%.6f\n", x));
    }

    public static void main(String[] args) {
        List<Integer> arr = new ArrayList<>();
        arr.add(1);
        arr.add(1);
        arr.add(0);
        arr.add(-1);
        arr.add(-1);

        plus_sum(arr);
    }
}
