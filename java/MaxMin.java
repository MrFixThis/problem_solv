import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * MaxMin
 */
public class MaxMin {
    private static void maxMinSum(List<Long> arr) {
        Collections.sort(arr);

        final long[] memo = new long[2];
        for(int i = 0; i < 2; i++) {
            if(i == 1)
                Collections.reverse(arr);
            memo[i] = arr.stream().limit(4).mapToLong(Long::longValue).sum();
        }

        System.out.println(memo[0] + " " + memo[1]);
    }

    public static void main(String[] args) {
        List<Long> arr = new ArrayList<>();
        arr.add(1L);
        arr.add(2L);
        arr.add(3L);
        arr.add(4L);
        arr.add(5L);

        maxMinSum(arr);
    }
}
