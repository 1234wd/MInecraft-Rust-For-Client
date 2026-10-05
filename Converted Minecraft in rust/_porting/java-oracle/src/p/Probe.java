package p;

public class Probe {
    public static void main(String[] a) {
        double x = 0.0, y = 1.0, z = 0.5;
        double lenSq = x * x + y * y + z * z;
        double len = Math.sqrt(lenSq);
        double arg = -y / len;
        System.out.printf("lenSq bits = 0x%016x%n", Double.doubleToRawLongBits(lenSq));
        System.out.printf("len   bits = 0x%016x  (%s)%n", Double.doubleToRawLongBits(len), len);
        System.out.printf("arg   bits = 0x%016x  (%s)%n", Double.doubleToRawLongBits(arg), arg);
        double r = Math.asin(arg);
        System.out.printf("asin  bits = 0x%016x  (%s)%n", Double.doubleToRawLongBits(r), r);
        System.out.printf("Strict asin bits = 0x%016x%n",
                Double.doubleToRawLongBits(StrictMath.asin(arg)));
        float f = (float) r * ((float) (180.0 / Math.PI));
        System.out.printf("f32 RESULT = 0x%08x  (%s)%n", Float.floatToRawIntBits(f), f);
        System.out.printf("RAD_TO_DEG as f32 bits = 0x%08x%n",
                Float.floatToRawIntBits((float) (180.0 / Math.PI)));
    }
}