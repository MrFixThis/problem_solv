import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.IntStream;

public class ProductSum {

    public static List<List<Integer>> productSum(List<Long> arr, long target) {
        final List<List<Integer>> comb = new ArrayList<>();
        final List<Integer> skiped_idxs = new ArrayList<>();
        IntStream.range(0, arr.size())
                .forEach(i -> {
                    final long curr_val = arr.get(i);
                    skiped_idxs.add(i);
                    backctrack(arr, target, curr_val, comb, skiped_idxs);
                    skiped_idxs.clear();
                });
        return comb;
    }

    private static void backctrack(
            List<Long> arr,
            long target,
            long curr_val,
            List<List<Integer>> comb,
            List<Integer> skiped_idxs) {

        final List<Integer> pos = new ArrayList<>(skiped_idxs);
        IntStream.range(0, arr.size())
                .forEach(i -> {
                    if (skiped_idxs.contains(i))
                        return;

                    long val = arr.get(i);
                    long sum = val + curr_val;

                    if (sum == target) {
                        pos.add(i);
                        comb.add(new ArrayList<>(pos));
                        pos.remove(pos.size() - 1);
                    } else if (sum < target) {
                        skiped_idxs.add(i);
                        backctrack(arr, target, sum, comb, skiped_idxs);
                        skiped_idxs.remove(pos.size() - 1);
                    }
                });
    }

    public static void main(String[] args) {
        final List<Long> vals = Arrays.asList(1L, 4L, 2L, 4L, 5L, 1L);
        final List<Long> arr = new ArrayList<>(vals);
        final long pres = 6;

        productSum(arr, pres).forEach(a -> {
            System.out.print("[ ");
            for (long val : a) {
                System.out.printf("%d ", val);
            }
            System.out.println("]");
        });
    }
}
