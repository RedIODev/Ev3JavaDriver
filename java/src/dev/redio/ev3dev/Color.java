package dev.redio.ev3dev;
/**
 * A simple Color class representing Colors using the rgb spectrum. 
 * The different RGB values range from 0 (no color) to X (intense Color) for each R, G, B value respectively.
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
     * @return 0 for no red to X for bright red
     */
    public int red() {
        return red;
    }

    /**
     * Returns the green value of this Color.
     * @return 0 for no green to X for bright green
     */
    public int green() {
        return green;
    }

    /**
     * Returns the blue value of this Color.
     * @return 0 for no blue to X for bright blue
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
        final int prime = 31;
        int result = 1;
        result = prime * result + red;
        result = prime * result + green;
        result = prime * result + blue;
        return result;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (!(obj instanceof Color))
            return false;
        Color other =(Color)obj;
        return this.red == other.red &&
                this.green == other.green &&
                this.blue == other.blue;
    }



    

    
}
