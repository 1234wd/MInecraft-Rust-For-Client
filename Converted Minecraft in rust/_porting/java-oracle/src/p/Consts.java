package p;

/** Prints the exact IEEE-754 bits of every FdLibm constant, as Rust `f64::from_bits` args. */
public class Consts {
    public static void main(String[] a) {
        // asin
        p("PIO2_HI", 0x1.921fb54442d18p0);
        p("PIO2_LO", 0x1.1a62633145c07p-54);
        p("PIO4_HI", 0x1.921fb54442d18p-1);
        p("PS0", 0x1.5555555555555p-3);
        p("PS1", -0x1.4d61203eb6f7dp-2);
        p("PS2", 0x1.9c1550e884455p-3);
        p("PS3", -0x1.48228b5688f3bp-5);
        p("PS4", 0x1.9efe07501b288p-11);
        p("PS5", 0x1.23de10dfdf709p-15);
        p("QS1", -0x1.33a271c8a2d4bp1);
        p("QS2", 0x1.02ae59c598ac8p1);
        p("QS3", -0x1.6066c1b8d0159p-1);
        p("QS4", 0x1.3b8c5b12e9282p-4);

        // atan
        p("ATANHI_0", 0x1.dac670561bb4fp-2);
        p("ATANHI_1", 0x1.921fb54442d18p-1);
        p("ATANHI_2", 0x1.f730bd281f69bp-1);
        p("ATANHI_3", 0x1.921fb54442d18p0);
        p("ATANLO_0", 0x1.a2b7f222f65e2p-56);
        p("ATANLO_1", 0x1.1a62633145c07p-55);
        p("ATANLO_2", 0x1.007887af0cbbdp-56);
        p("ATANLO_3", 0x1.1a62633145c07p-54);
        p("AT_0", 0x1.555555555550dp-2);
        p("AT_1", -0x1.999999998ebc4p-3);
        p("AT_2", 0x1.24924920083ffp-3);
        p("AT_3", -0x1.c71c6fe231671p-4);
        p("AT_4", 0x1.745cdc54c206ep-4);
        p("AT_5", -0x1.3b0f2af749a6dp-4);
        p("AT_6", 0x1.10d66a0d03d51p-4);
        p("AT_7", -0x1.dde2d52defd9ap-5);
        p("AT_8", 0x1.97b4b24760debp-5);
        p("AT_9", -0x1.2b4442c6a6c2fp-5);
        p("AT_10", 0x1.0ad3ae322da11p-6);

        // atan2
        p("ATAN2_TINY", 1.0e-300);
        p("PI_O_4", 0x1.921fb54442d18p-1);
        p("PI_O_2", 0x1.921fb54442d18p0);
        p("PI_LO", 0x1.1a62633145c07p-53);
        p("HUGE", 1.0e300);
    }

    static void p(String name, double v) {
        System.out.printf("%-12s 0x%016x  // %s%n",
                name, Double.doubleToRawLongBits(v), Double.toString(v));
    }
}