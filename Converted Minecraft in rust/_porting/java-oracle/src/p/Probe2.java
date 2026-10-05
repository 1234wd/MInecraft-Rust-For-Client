package p;

public class Probe2 {
    public static void main(String[] a) {
        // Java: (float)(180.0 / Math.PI)   vs   180.0F / (float)Math.PI
        float viaDouble = (float) (180.0 / Math.PI);
        float viaFloat = 180.0F / (float) Math.PI;
        System.out.printf("(float)(180.0/Math.PI)   = 0x%08x  (%s)%n",
                Float.floatToRawIntBits(viaDouble), viaDouble);
        System.out.printf("180.0F/(float)Math.PI   = 0x%08x  (%s)%n",
                Float.floatToRawIntBits(viaFloat), viaFloat);
        System.out.printf("(float)Math.PI          = 0x%08x%n",
                Float.floatToRawIntBits((float) Math.PI));

        double arg = -1.0 / Math.sqrt(0.0 * 0.0 + 1.0 * 1.0 + 0.5 * 0.5);
        float asinF = (float) Math.asin(arg);
        System.out.printf("(float)Math.asin(arg)   = 0x%08x%n", Float.floatToRawIntBits(asinF));
        System.out.printf("asinF * viaFloat        = 0x%08x%n",
                Float.floatToRawIntBits(asinF * viaFloat));
        System.out.printf("asinF * viaDouble       = 0x%08x%n",
                Float.floatToRawIntBits(asinF * viaDouble));
        System.out.println();
        System.out.println("golden expects 0xc27dbd63");
        System.out.println();
        float deg2rad = (float)(Math.PI / 180.0);
        System.out.printf("Mth.DEG_TO_RAD = (float)(Math.PI/180.0) = 0x%08x  (%s)%n", Float.floatToRawIntBits(deg2rad), deg2rad);
        System.out.printf("Mth.RAD_TO_DEG = 180.0F/(float)Math.PI = 0x%08x  (%s)%n", Float.floatToRawIntBits(viaFloat), viaFloat);
    }
}