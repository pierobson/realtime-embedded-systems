#include <Arduino_FreeRTOS.h>
#include <Wire.h>
#include <Adafruit_Sensor.h>
#include <Adafruit_BNO055.h>
#include <utility/imumaths.h>

/* Set the delay between fresh samples */
#define BNO055_SAMPLERATE_DELAY_MS (100)

#define RPI_ADDRESS 4

// Check I2C device address and correct line below (by default address is 0x29 or 0x28)
//                                   id, address
Adafruit_BNO055 bno = Adafruit_BNO055(-1, 0x28, &Wire);

volatile bool writing = false;
volatile bool reading = false;
double imu_data[] = {0.0, 0.0, 0.0 };

// define two tasks for Blink & AnalogRead
void TaskListen( void *pvParameters );
void TaskIMUReadWrite( void *pvParameters );

void sendIMUData();

// the setup function runs once when you press reset or power the board
void setup() {
  
  // initialize serial communication at 9600 bits per second:
  Serial.begin(115200);
  
  while (!Serial) delay(10);  // wait for serial port to open

  // Now set up two tasks to run independently.
  xTaskCreate(
    TaskListen
    ,  "Listen"   // A name just for humans
    ,  128  // This stack size can be checked & adjusted by reading the Stack Highwater
    ,  NULL
    ,  1  // Priority, with 3 (configMAX_PRIORITIES - 1) being the highest, and 0 being the lowest.
    ,  NULL );

  xTaskCreate(
    TaskIMUReadWrite
    ,  "IMUReadWrite"
    ,  128  // Stack size
    ,  NULL
    ,  2  // Priority
    ,  NULL );

  // Now the task scheduler, which takes over control of scheduling individual tasks, is automatically started.
}

void loop()
{
  // Empty. Things are done in Tasks.
}

/*--------------------------------------------------*/
/*---------------------- Tasks ---------------------*/
/*--------------------------------------------------*/

void TaskListen(void *pvParameters)  // This is a task.
{
  (void) pvParameters;

  byte read;

  for (;;) // A Task shall never return or exit.
  {
    read = Serial.read();
    if (read == '.')
    {
      sendIMUData();
    }
    else 
    {
      Serial.print(read);
    }
    vTaskDelay(10);  // one tick delay (15ms) in between reads for stability
  }
}

void TaskIMUReadWrite(void *pvParameters)  // This is a task.
{
  (void) pvParameters;

  Serial.println("In task");

  /* Initialise the sensor */
  if(!bno.begin())
  {
    /* There was a problem detecting the BNO055 ... check your connections */
    Serial.print("Ooops, no BNO055 detected ... Check your wiring or I2C ADDR!");
    while(1) { vTaskDelay(10); }
  }
  
  Serial.println("BNO inited.");

  bno.setExtCrystalUse(true);

  Serial.println("Starting wire");

  Wire.begin(RPI_ADDRESS);
  
  Serial.println("Wire started");

  Wire.onRequest(sendIMUData);

  Serial.println("onRequest set.");

  for (;;)
  {
    // Possible vector values can be:
    // - VECTOR_ACCELEROMETER - m/s^2
    // - VECTOR_MAGNETOMETER  - uT
    // - VECTOR_GYROSCOPE     - rad/s
    // - VECTOR_EULER         - degrees
    // - VECTOR_LINEARACCEL   - m/s^2
    // - VECTOR_GRAVITY       - m/s^2
    imu::Vector<3> euler = bno.getVector(Adafruit_BNO055::VECTOR_EULER);

    // Write the Yaw, Pitch, and Roll to Serial 

    double yaw = euler.x();
    double pitch = euler.y();
    double roll = euler.z();

    Serial.println("Waiting for read");
    while(reading) {}
    writing = true;

    Serial.println("Writing...");

    imu_data[0] = yaw;
    imu_data[1] = pitch;
    imu_data[2] = roll;
    writing = false;

    vTaskDelay(1);  // one tick delay (15ms) in between reads for stability
  }
}

void sendIMUData()
{
  while(writing){}
  reading = true;
  Wire.write((byte*)imu_data, sizeof(imu_data));
  Serial.print("Yaw: ");
  Serial.print(imu_data[0]);
  Serial.print("Pitch: ");
  Serial.print(imu_data[1]);
  Serial.print("Roll: ");
  Serial.print(imu_data[2]);
  Serial.print("\n");
  reading = false;
}
