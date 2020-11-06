class Twist {
    private static native String search(String token, String query);

    static {
        System.loadLibrary("twist");
        //System.loadLibrary("/home/pothix/repos/twist-rs/target/debug/libtwist_rs.so");
    }

    public static void main(String[] args) {
        String output = Twist.search(System.getenv("auth"), "pothix");
        System.out.println(output);
    }
}
