package dev.redio.ev3dev.exceptions;

/**
 * Ev3InternalException
 */
public class Ev3InternalException extends Ev3Exception {
    public Ev3InternalException(String msg) {
        super("Internal Error: " + msg + "!");
    }
    
}