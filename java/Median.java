import java.util.Collections;
import java.util.List;

public class Median {
    public static int findMedian(List<Integer> arr) {
        Collections.sort(arr);

        int idx = 0;
        if((arr.size() & 1) == 0)
            idx = (arr.size() + 1) / 2;
        else
            idx = arr.size() / 2;

        return arr.get(idx);
    }
}
