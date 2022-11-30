package dev.redio.ev3dev;

import java.util.Objects;

import dev.redio.ev3dev.exceptions.InvalidColorException;

/**
 * A simple Color class representing Colors using the rgb spectrum. 
 * The different RGB values range from 0 (no color) to 255 (intense Color) for each R, G, B value respectively.
 * @implNote the RGB values can vary depending on lighting conditions.
 * @author RedIODev
 */
public final class Color implements Comparable<Color> {
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
        if (red < 0 || green < 0 || blue < 0 || red > 255 || green > 255 || blue > 255)
            throw new InvalidColorException(red, green, blue);
        this.red = red;
        this.green = green;
        this.blue = blue;
    }

    /**
     * Returns the red value of this Color.
     * @return 0 for no red to 255 for bright red
     */
    public int red() {
        return red;
    }

    /**
     * Returns the green value of this Color.
     * @return 0 for no green to 255 for bright green
     */
    public int green() {
        return green;
    }

    /**
     * Returns the blue value of this Color.
     * @return 0 for no blue to 255 for bright blue
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

    @Override
    public int compareTo(Color o) {

        var hsb1 = java.awt.Color.RGBtoHSB(red, green, blue, null);
        var h1 = hsb1[0];
        var s1 = hsb1[1];
        var b1 = hsb1[2];
        var hsb2 = java.awt.Color.RGBtoHSB(o.red, o.green, o.blue, null);
        var h2 = hsb2[0];
        var s2 = hsb2[1];
        var b2 = hsb2[2];
        if (h1 != h2)
            return Math.round(h1 - h2);
        if (s1 != s2)
            return Math.round(s1 - s2);
        return Math.round(b1 -b2);
    }





    

    
}
