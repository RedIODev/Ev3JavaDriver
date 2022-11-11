package dev.redio.ev3dev.exceptions;

public class Ev3NotConnectedException extends Exception {
    private final String device;
    private final String port;

    public Ev3NotConnectedException(String device) {
        this(device, null);
    }

    public Ev3NotConnectedException(String device, String port) {
        super("'"+ device + "' not connected at port " + port + "!");
        this.device = device;
        this.port = port;
    }

    public String device() {
        return device;
    }

    public String port() {
        return port;
    }


}
