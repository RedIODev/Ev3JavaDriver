package dev.redio.ev3dev;

@FunctionalInterface
public interface Condition<X extends Exception> {
    boolean condition() throws X;
}
