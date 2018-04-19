import java.util.Arrays;

public class File {

  public static void main(String[] args) {
    final int[] t = new int[] {
      123,
      456,
      67890
    };
    System.out.println(Arrays.toString(t));

    final long[] l = new long[] {
      1L,
      234L,
      567890L
    };
    System.out.println(Arrays.toString(l));
  }

}