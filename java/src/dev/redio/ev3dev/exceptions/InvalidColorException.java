package dev.redio.ev3dev.exceptions;

public class InvalidColorException extends IllegalArgumentException {
    public InvalidColorException(int red, int green, int blue) {
        super("The values for red:" + red + ", green:" + green + ", blue:" + blue + " define not a valid color.");
    }
}
