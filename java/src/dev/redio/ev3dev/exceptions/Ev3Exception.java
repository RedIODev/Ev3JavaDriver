package dev.redio.ev3dev.exceptions;

public abstract class Ev3Exception extends Exception {
    protected Ev3Exception(String msg) {
        super(msg);
    }
}
