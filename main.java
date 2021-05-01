public class main {
    public static int counter = 0;

    public static int foo(int n) {
        //int a, b;
        if (n > 0) {
            counter += 1;
            foo(n - 1);
        }
        return 0;
    }

    public static void main(String[] args) {
        //System.out.println("Hello World!");
        //int ITER = 1000000;
        //int ITER = 20000;
        int ITER = 30000;
        try {
            foo(ITER);
        } catch(Throwable e) {
            //System.out.println(e.toString() + "\n ITER" + ITER);
            System.out.println(e.toString());
            System.out.println("counter " + counter);
        }
    }
}
