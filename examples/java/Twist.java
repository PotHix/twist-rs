class Twist {
    private static native String search(String token, String query);

    static {
        System.loadLibrary("twist");
    }

    public static void main(String[] args) {
        String output = Twist.search(System.getenv("auth"), "pothix");
        System.out.println(output);
    }
}
