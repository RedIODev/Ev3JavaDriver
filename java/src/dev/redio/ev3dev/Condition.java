package dev.redio.ev3dev;
/**
 * An interface representing a method that returns a boolean.
 * This interface is for example used as a callback to control conditional sleep methods.
 * @param X the exception thrown by the implementing method. 
 * @apiNote Set the generic parameter to RuntimeException and don't throw anything if the implementing method is exception free.
 * @see Motor#sleep
 * @author RedIODev
 */
@FunctionalInterface
public interface Condition<X extends Exception> {
    /**
     * Returns true if the condition is met.
     * @return whether the condition is met.
     * @throws X when the implementing method throws an exception.
     */
    boolean condition() throws X;
}
