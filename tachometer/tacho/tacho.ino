/*
 * Optical Tachometer
 *
 * Uses an IR LED and IR phototransistor to implement an optical tachometer.
 * The IR LED is connected to pin 13 and ran continually. A status LED is connected
 * to pin 12. Pin 2 (interrupt 0) is connected across the IR detector.
 *
 * 
 */

#define PHOTODIODE 2
#define STATUSPIN 12
#define IRLED 13           

#define MOTOR_ENABLE 5
#define MOTOR_DIRECTION_A 3
#define MOTOR_DIRECTION_B 4

volatile int rpmcount;
volatile int status;

unsigned int rpm;

unsigned long timeold;

 void rpm_fun()
 {
   //Each rotation, this interrupt function is run twice, so take that into consideration for 
   //calculating RPM
   //Update count
    rpmcount++;
      
   //Toggle status LED   
   if (status == LOW) {
     status = HIGH;
   } else {
     status = LOW;
   }
   digitalWrite(STATUSPIN, status);
 }

void setup()
 {
   Serial.begin(9600);
   //Interrupt 0 is digital pin 2, so that is where the IR detector is connected
   //Triggers on FALLING (change from HIGH to LOW)
   attachInterrupt(digitalPinToInterrupt(PHOTODIODE), rpm_fun, FALLING);
   
   //Turn on IR LED
   pinMode(IRLED, OUTPUT); 
   digitalWrite(IRLED, HIGH);
   
   //Use statusPin to flash along with interrupts
   pinMode(STATUSPIN, OUTPUT);

    // Enable the pins for the motor
    pinMode(MOTOR_ENABLE,OUTPUT);
    pinMode(MOTOR_DIRECTION_A,OUTPUT);
    pinMode(MOTOR_DIRECTION_B,OUTPUT);

    // Set the spinning direction.
    digitalWrite(MOTOR_DIRECTION_A, HIGH);
    digitalWrite(MOTOR_DIRECTION_B, LOW);

    // Set the motor speed.
    analogWrite(MOTOR_ENABLE, 128);

   rpmcount = 0;
   rpm = 0;
   timeold = 0;
   status = LOW;
 }

 unsigned int speed = 0;

 void loop()
 {
   //Update RPM every second
   delay(1000);
   //Don't process interrupts during calculations
   detachInterrupt(digitalPinToInterrupt(PHOTODIODE));
   //Note that this would be 60*1000/(millis() - timeold)*rpmcount if the interrupt
   //happened once per revolution instead of twice. Other multiples could be used
   //for multi-bladed propellers or fans
   rpm = (unsigned int)((float)20000/(float)(millis() - timeold)*(float)rpmcount);
   timeold = millis();
   rpmcount = 0;
   
   //Write it out to serial port
   Serial.println(rpm,DEC);
   
   //Restart the interrupt processing
   attachInterrupt(digitalPinToInterrupt(PHOTODIODE), rpm_fun, FALLING);

   analogWrite(MOTOR_ENABLE, speed+100);
   speed = (speed + 2) % 155;
  }



