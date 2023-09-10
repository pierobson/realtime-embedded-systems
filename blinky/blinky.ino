bool blinking = false;

void setup() {
  // initialize digital pin LED_BUILTIN as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
}

// the loop function runs over and over again forever
void loop() {

  if (blinking) 
  {
    digitalWrite(LED_BUILTIN, HIGH);  // turn the LED on (HIGH is the voltage level)
    delay(1000);                      // wait for a second
    digitalWrite(LED_BUILTIN, LOW);   // turn the LED off by making the voltage LOW
    delay(1000);                      // wait for a second
  }
  else
  {
    if (Serial.available() > 0)
    {
      String str = Serial.readString();
      str.trim();

      Serial.print("I received: ");
      Serial.println(str);

      blinking = true;
    }
  }
}
