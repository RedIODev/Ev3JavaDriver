package dev.redio.ev3dev;

import java.util.Objects;

/**
 * A simple Color class representing Colors using the rgb spectrum. 
 * The different RGB values range from 0 (no color) to 1020 (intense Color) for each R, G, B value respectively.
 * @implNote the RGB values can vary depending on lighting conditions.
 * @author RedIODev
 */
public final class Color {
    private final int red;
    private final int green;
    private final int blue;
    
    /**
     * Creates a new Color object by specifying it's rgb value.
     * @param red the red value
     * @param green the blue value
     * @param blue the green value
     */
    public Color(int red, int green, int blue) {
        this.red = red;
        this.green = green;
        this.blue = blue;
    }

    /**
     * Returns the red value of this Color.
     * @return 0 for no red to 1020 for bright red
     */
    public int red() {
        return red;
    }

    /**
     * Returns the green value of this Color.
     * @return 0 for no green to 1020 for bright green
     */
    public int green() {
        return green;
    }

    /**
     * Returns the blue value of this Color.
     * @return 0 for no blue to 1020 for bright blue
     */
    public int blue() {
        return blue;
    }

    /**
     * The String representation of the Color object with the following format:
     * <pre>
     * "Color[red=238, green=130, blue=238]"
     * </pre>
     * @return the String representation of this Color
     */
    @Override
    public String toString() {
        StringBuilder builder = new StringBuilder("Color[red=");
        builder.append(red);
        builder.append(", green=");
        builder.append(green);
        builder.append(", blue=");
        builder.append(blue);
        builder.append("]");
        return builder.toString();
    }

    @Override
    public int hashCode() {
        return Objects.hash(red, green, blue);
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        Color other = (Color) obj;
        return red == other.red && green == other.green && blue == other.blue;
    }





    

    
}
