#define UNIT_DURATION_MS 200
#define DOT_DURATION_MS UNIT_DURATION_MS
#define DASH_DURATION_MS  (UNIT_DURATION_MS * 3)
#define DEFAULT_SPACE_MS  UNIT_DURATION_MS
#define LETTER_SPACE_MS (UNIT_DURATION_MS * 3)
#define WORD_SPACE_MS (UNIT_DURATION_MS * 7)

#define SENTINEL '!'

enum Code {
  Dot,
  Dash
};

struct CodeAndLength {
  Code* code;
  uint8_t length;
};

const Code A[] = { Code::Dot, Code::Dash };                         // .-
const Code B[] = { Code::Dash, Code::Dot, Code::Dot, Code::Dot };   // -...
const Code C[] = { Code::Dash, Code::Dot, Code::Dash, Code::Dot };  // -.-.
const Code D[] = { Code::Dash, Code::Dot, Code::Dot };              // -..
const Code E[] = { Code::Dot };                                     // .
const Code F[] = { Code::Dot, Code::Dot, Code::Dash, Code::Dot };   // ..-.
const Code G[] = { Code::Dash, Code::Dash, Code::Dot };             // --.
const Code H[] = { Code::Dot, Code::Dot, Code::Dot, Code::Dot };    // ....
const Code I[] = { Code::Dot, Code::Dot };                          // ..
const Code J[] = { Code::Dot, Code::Dash, Code::Dash, Code::Dash }; // .---
const Code K[] = { Code::Dash, Code::Dot, Code::Dash };             // -.-
const Code L[] = { Code::Dot, Code::Dash, Code::Dot, Code::Dot };   // .-..
const Code M[] = { Code::Dash, Code::Dash };                        // --
const Code N[] = { Code::Dash, Code::Dot };                         // -.
const Code O[] = { Code::Dash, Code::Dash, Code::Dash };            // ---
const Code P[] = { Code::Dot, Code::Dash, Code::Dash, Code::Dot };  // .--.
const Code Q[] = { Code::Dash, Code::Dash, Code::Dot, Code::Dash }; // --.-
const Code R[] = { Code::Dot, Code::Dash, Code::Dot };              // .-.
const Code S[] = { Code::Dot, Code::Dot, Code::Dot };               // ...
const Code T[] = { Code::Dash };                                    // -
const Code U[] = { Code::Dot, Code::Dot, Code::Dash };              // ..-
const Code V[] = { Code::Dot, Code::Dot, Code::Dot, Code::Dash };   // ...- 
const Code W[] = { Code::Dot, Code::Dash, Code::Dash };             // .--
const Code X[] = { Code::Dash, Code::Dot, Code::Dot, Code::Dash };  // -..-
const Code Y[] = { Code::Dash, Code::Dot, Code::Dash, Code::Dash }; // -.--
const Code Z[] = { Code::Dash, Code::Dash, Code::Dot, Code::Dot };  // --..

const Code ONE[] = { Code::Dot, Code::Dash, Code::Dash, Code::Dash, Code::Dash };   // .----
const Code TWO[] = { Code::Dot, Code::Dot, Code::Dash, Code::Dash, Code::Dash };   // ..---
const Code THREE[] = { Code::Dot, Code::Dot, Code::Dot, Code::Dash, Code::Dash };   // ...--
const Code FOUR[] = { Code::Dot, Code::Dot, Code::Dot, Code::Dot, Code::Dash };   // ....-
const Code FIVE[] = { Code::Dot, Code::Dot, Code::Dot, Code::Dot, Code::Dot };   // .....
const Code SIX[] = { Code::Dash, Code::Dot, Code::Dot, Code::Dot, Code::Dot };   // -....
const Code SEVEN[] = { Code::Dash, Code::Dash, Code::Dot, Code::Dot, Code::Dot };   // --...
const Code EIGHT[] = { Code::Dash, Code::Dash, Code::Dash, Code::Dot, Code::Dot };   // ---..
const Code NINE[] = { Code::Dash, Code::Dash, Code::Dash, Code::Dash, Code::Dot };   // ----.
const Code ZERO[] = { Code::Dash, Code::Dash, Code::Dash, Code::Dash, Code::Dash }; // -----

Code *codes[] = { A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE };
uint8_t code_lengths[] = { 2, 4, 4, 3, 1, 4, 3, 4, 2, 4, 3, 4, 2, 2, 3, 4, 4, 3, 3, 1, 3, 4, 3, 4, 4, 4 };

uint16_t letter_index = 0;
uint16_t code_index = 0;
CodeAndLength current_cal { nullptr, 0 }; 

bool blinking = false;
String user_input = "";


void dot() {
    //Serial.println("DOT");
    digitalWrite(LED_BUILTIN, HIGH);
    delay(DOT_DURATION_MS); 
}

void dash() {
    //Serial.println("DASH");
    digitalWrite(LED_BUILTIN, HIGH);  
    delay(DASH_DURATION_MS);
}

void default_space() {
  //Serial.println("PAUSE");
  digitalWrite(LED_BUILTIN, LOW);
  delay(DEFAULT_SPACE_MS);
}

void letter_space() {
  //Serial.println("LETTER PAUSE");
  digitalWrite(LED_BUILTIN, LOW);
  delay(LETTER_SPACE_MS);
}

void word_space() {
  //Serial.println("WORD PAUSE");
  digitalWrite(LED_BUILTIN, LOW);
  delay(WORD_SPACE_MS);
}

CodeAndLength next_letter() {  
  CodeAndLength cal { nullptr, 0 };

  if (letter_index >= user_input.length())
    return cal;

  char c = user_input[letter_index];
  uint8_t letter = (uint8_t)c;

  // Serial.print("Next letter is: ");
  // Serial.println(c);

  // Check for letter
  if (letter > 96 && letter < 123)
  {
    uint8_t index = letter - 97;
    cal.code = codes[index];
    cal.length = code_lengths[index]; 
  }
  // Check for digit
  else if (letter > 47 && letter < 58)
  {
    cal.code = codes[letter - 22]; // + 26 - 48
    cal.length = 5; 
  }

  return cal;
}


void do_blink()
{
  if (letter_index >= user_input.length())
  {
    // loop back to start of string.
    letter_index = 0;
    code_index = 0;
    current_cal = next_letter();

    // Do a long pause.
    word_space();
    return;
  }

  // Somewhere in the middle of a letter.
  Code &code = current_cal.code[code_index];
  if (code == Code::Dot)
    dot();
  else
    dash();

  code_index += 1;
  
  if (code_index >= current_cal.length)
  {
    letter_index += 1;
    code_index = 0;
    current_cal = next_letter();
  
    if (current_cal.code == nullptr)
    {
      // Space between words.
      word_space();
    }
    else
    {
      // Space between letters.
      letter_space();
    }
  }
  else
  {
    // Regular pause.
    default_space();
  }
}

void setup() {
  // initialize digital pin LED_BUILTIN as an output.
  pinMode(LED_BUILTIN, OUTPUT);
  Serial.begin(9600);
}

// the loop function runs over and over again forever
void loop() {
  if (Serial.available() > 0)
  {
      String str = Serial.readString();

      // Check to reset.
      if (str.indexOf(SENTINEL) != -1)
      {
        // Found the sentinel.
        Serial.println("Received sentinel value.");

        user_input = "";
        blinking = false;
      }
      else
      {
        str.trim();
        str.toLowerCase();

        //Serial.print("I received: ");
        Serial.println(str);

        user_input = str;
        blinking = true;
        letter_index = 0;
        code_index = 0;
        current_cal = next_letter();
      }
  }

  if (blinking)
  {
    do_blink();
  }
}