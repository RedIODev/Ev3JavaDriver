package dev.redio;

import dev.redio.ev3dev.Motor;
import dev.redio.ev3dev.MotorPort;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public class Main {
    public static void main(String[] args) throws Ev3Exception, InterruptedException {
        try (var lm = new Motor(MotorPort.OUT_A)) {
            lm.setSpeedSetPoint(500);
            lm.runForever();
            Thread.sleep(2000);
            lm.stop();
            Thread.sleep(1000);
        }
    }
}
