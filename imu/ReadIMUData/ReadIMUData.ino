#include <Arduino_FreeRTOS.h>
#include <SoftwareWire.h>
#include <Wire.h>
#include <Adafruit_Sensor.h>
#include <Adafruit_BNO055.h>
#include <utility/imumaths.h>

/* Set the delay between fresh samples */
#define BNO055_SAMPLERATE_DELAY_MS (100)

#define SLAVE_ADDRESS 12

SoftwareWire imuWire = SoftwareWire(14, 15);

// Check I2C device address and correct line below (by default address is 0x29 or 0x28)
//                                   id, address
Adafruit_BNO055 bno = Adafruit_BNO055(-1, 0x28, &imuWire);

// define two tasks for Blink & AnalogRead
void TaskBlink(void *pvParameters);
void TaskIMUReadWrite(void *pvParameters);

void sendIMUData();
void IMUDataToSerial();

// the setup function runs once when you press reset or power the board
void setup() {

  // initialize serial communication at 9600 bits per second:
  Serial.begin(115200);

  while (!Serial) delay(10);  // wait for serial port to open

  // Now set up two tasks to run independently.
  xTaskCreate(
    TaskBlink, "Blink"  // A name just for humans
    ,
    128  // This stack size can be checked & adjusted by reading the Stack Highwater
    ,
    NULL, 1  // Priority, with 3 (configMAX_PRIORITIES - 1) being the highest, and 0 being the lowest.
    ,
    NULL);

  xTaskCreate(
    TaskIMUReadWrite, "IMUReadWrite", 128  // Stack size
    ,
    NULL, 2  // Priority
    ,
    NULL);

  // Now the task scheduler, which takes over control of scheduling individual tasks, is automatically started.
}

void loop() {
  // Empty. Things are done in Tasks.
}

/*--------------------------------------------------*/
/*---------------------- Tasks ---------------------*/
/*--------------------------------------------------*/

volatile double imu_data[3] = { 1.0, 2.0, 3.0 };

void TaskBlink(void *pvParameters)  // This is a task.
{
  (void)pvParameters;

  /*
  Blink
  Turns on an LED on for one second, then off for one second, repeatedly.

  Most Arduinos have an on-board LED you can control. On the UNO, LEONARDO, MEGA, and ZERO 
  it is attached to digital pin 13, on MKR1000 on pin 6. LED_BUILTIN takes care 
  of use the correct LED pin whatever is the board used.
  
  The MICRO does not have a LED_BUILTIN available. For the MICRO board please substitute
  the LED_BUILTIN definition with either LED_BUILTIN_RX or LED_BUILTIN_TX.
  e.g. pinMode(LED_BUILTIN_RX, OUTPUT); etc.
  
  If you want to know what pin the on-board LED is connected to on your Arduino model, check
  the Technical Specs of your board  at https://www.arduino.cc/en/Main/Products
  
  This example code is in the public domain.

  modified 8 May 2014
  by Scott Fitzgerald
  
  modified 2 Sep 2016
  by Arturo Guadalupi
*/

  // initialize digital LED_BUILTIN on pin 13 as an output.
  pinMode(LED_BUILTIN, OUTPUT);

  for (;;)  // A Task shall never return or exit.
  {
    digitalWrite(LED_BUILTIN, HIGH);        // turn the LED on (HIGH is the voltage level)
    vTaskDelay(1000 / portTICK_PERIOD_MS);  // wait for one second
    digitalWrite(LED_BUILTIN, LOW);         // turn the LED off by making the voltage LOW
    vTaskDelay(1000 / portTICK_PERIOD_MS);  // wait for one second
  }
}

void TaskIMUReadWrite(void *pvParameters)  // This is a task.
{
  (void)pvParameters;

  Serial.println("In task");

  /* Initialize the sensor */
  if (!bno.begin()) {
    /* There was a problem detecting the BNO055 ... check your connections */
    Serial.print("Ooops, no BNO055 detected ... Check your wiring or I2C ADDR!");
    while (1) { vTaskDelay(10); }
  }

  Serial.println("BNO inited.");

  bno.setExtCrystalUse(true);

  // Initialize Wire
  Serial.println("Starting wire");

  Wire.begin(SLAVE_ADDRESS);
  Wire.onRequest(sendIMUData);

  uint16_t count = 0;

  for (;;) {
    // Possible vector values can be:
    // - VECTOR_ACCELEROMETER - m/s^2
    // - VECTOR_MAGNETOMETER  - uT
    // - VECTOR_GYROSCOPE     - rad/s
    // - VECTOR_EULER         - degrees
    // - VECTOR_LINEARACCEL   - m/s^2
    // - VECTOR_GRAVITY       - m/s^2
    imu::Vector<3> euler = bno.getVector(Adafruit_BNO055::VECTOR_EULER);

    // Write the Yaw, Pitch, and Roll to I2C (and Serial)

    imu_data[0] = euler.x();
    imu_data[1] = euler.y();
    imu_data[2] = euler.z();

    count += 1;

    if (count > 500)
    {
      IMUDataToSerial();
      count = 0;
    }

    vTaskDelay(1);  // one tick delay (15ms) in between reads for stability
  }
}

void IMUDataToSerial()
{
    Serial.print("Yaw: ");
    Serial.print(imu_data[0]);
    Serial.print(" Pitch: ");
    Serial.print(imu_data[1]);
    Serial.print(" Roll: ");
    Serial.println(imu_data[2]);
}

void sendIMUData() {
  Wire.write((uint8_t *)imu_data, sizeof(imu_data));
  Serial.print("Wrote: ");
  IMUDataToSerial();
}