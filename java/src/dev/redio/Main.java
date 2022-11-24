package dev.redio;

import dev.redio.ev3dev.ColorSensor;
import dev.redio.ev3dev.Motor;
import dev.redio.ev3dev.MotorPort;
import dev.redio.ev3dev.SensorPort;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public class Main {
    public static void main(String[] args) throws Ev3Exception, InterruptedException {
        try (var m = new Motor(MotorPort.OUT_A)) {
            var oldPos = m.getPosition();
            m.setSpeedSetPoint(50);
            m.setPositionSetpoint(100);
            m.runToRelativePosition();
            m.waitUntilNotMoving();
            var newPos = m.getPosition();
            System.out.println("OLD:" + oldPos + " NEW:" + newPos);
            // s.setRGBRaw();
            // m1.runForever();
            // m2.runForever();
            // while(true) {
            //     // if (s.getRGB().blackWhite() < 20) {
            //     //     m1.runToRelativePosition(100);
            //     // }
            //     // if (s.getRGB().blackWhite() < 20) {
            //     //     m2.runToRelativePosition(100);
            //     // }
            // }
        }
    }

    
}
