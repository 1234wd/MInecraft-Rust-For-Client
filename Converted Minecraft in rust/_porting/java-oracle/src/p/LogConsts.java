package p;

/** Prints the exact IEEE-754 bits of FdLibm's `e_log` constants, for the Rust port. */
public class LogConsts {
    public static void main(String[] a) {
        p("TWO54", 0x1p54);
        p("LN2_HI", 0x1.62e42feep-1);
        p("LN2_LO", 0x1.a39ef35793c76p-33);
        p("LG1", 0x1.5555555555593p-1);
        p("LG2", 0x1.999999997fa04p-2);
        p("LG3", 0x1.2492494229359p-2);
        p("LG4", 0x1.c71c51d8e78afp-3);
        p("LG5", 0x1.7466496cb03dep-3);
        p("LG6", 0x1.39a09d078c69fp-3);
        p("LG7", 0x1.2f112df3e5244p-3);
    }

    static void p(String name, double v) {
        System.out.printf("%-8s 0x%016x%n", name, Double.doubleToRawLongBits(v));
    }
}