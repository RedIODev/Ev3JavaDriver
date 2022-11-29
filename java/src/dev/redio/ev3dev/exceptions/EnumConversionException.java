package dev.redio.ev3dev.exceptions;


public class EnumConversionException extends IndexOutOfBoundsException {

    @Deprecated
    public EnumConversionException(String msg) {
        super(msg);
    }

    public EnumConversionException(int index) {
        super("Enum index out of bounds: " + index);
    }
}