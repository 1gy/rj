public class HelloWorld {

    private String message = "Hello, World!";

    private void sayHello() {
        System.out.println(message);
    }

    public static void main(String[] args) {
        new HelloWorld().sayHello();
    }
}
