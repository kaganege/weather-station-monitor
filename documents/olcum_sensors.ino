char databuffer[35];
double temp;

void getBuffer() // Get weather status data
{
  int index;
  for (index = 0; index < 35; index++)
  {
    if (Serial.available())
    {
      databuffer[index] = Serial.read();
      if (databuffer[0] != 'c')
      {
        index = -1;
      }
    }
    else
    {
      index--;
    }
  }
}

int transCharToInt(char *_buffer, int _start, int _stop) // char to int
{
  int _index;
  int result = 0;
  int num = _stop - _start + 1;
  int _temp[num];
  for (_index = _start; _index <= _stop; _index++)
  {
    _temp[_index - _start] = _buffer[_index] - '0';
    result = 10 * result + _temp[_index - _start];
  }
  return result;
}

int RuzgarYonu() // Wind Direction
{
  return transCharToInt(databuffer, 1, 3);
}

float OrtRuzgarHizi() // air Speed (1 minute)
{
  temp = 0.44704 * transCharToInt(databuffer, 5, 7);
  return temp;
}

float MaksRuzgarHizi() // Max air speed (5 minutes)
{
  temp = 0.44704 * transCharToInt(databuffer, 9, 11);
  return temp;
}

float Sicaklik() // Temperature ("C")
{
  temp = (transCharToInt(databuffer, 13, 15) - 32.00) * 5.00 / 9.00;
  return temp;
}

float YagmurOrt() // Rainfall (1 hour)
{
  temp = transCharToInt(databuffer, 17, 19) * 25.40 * 0.01;
  return temp;
}

float RainfallOneDay() // Rainfall (24 hours)
{
  temp = transCharToInt(databuffer, 21, 23) * 25.40 * 0.01;
  return temp;
}

int Nem() // Humidity
{
  return transCharToInt(databuffer, 25, 26);
}

float Basinc() // Barometric Pressure
{
  temp = transCharToInt(databuffer, 28, 32);
  return temp / 10.00;
}

void setup()
{
  Serial.begin(115200);
}

void loop()
{
  getBuffer(); // Begin!
  Serial.print("Rüzgar Yonu: ");
  if (RuzgarYonu() == 0)
  {
    Serial.println("Kuzey");
    Serial.print(RuzgarYonu());
  }
  else if (RuzgarYonu() == 45)
  {
    Serial.println("Kuzey Dogu");
  }
  else if (RuzgarYonu() == 90)
  {
    Serial.println("Dogu");
  }
  else if (RuzgarYonu() == 135)
  {
    Serial.println("Guney Dogu");
  }
  else if (RuzgarYonu() == 180)
  {
    Serial.println("Guney");
  }
  else if (RuzgarYonu() == 225)
  {
    Serial.println("Guney Batı");
  }
  else if (RuzgarYonu() == 270)
  {
    Serial.println("Batı");
  }
  else if (RuzgarYonu() == 315)
  {
    Serial.println("Kuzey Batı");
  }
  Serial.println("  ");
  Serial.print("OrtRuzgarHizi (Bir Dakika): ");
  Serial.print(OrtRuzgarHizi());
  Serial.println("m/s  ");
  Serial.print("Max Ruzgar Hizi : (Five Minutes): ");
  Serial.print(MaksRuzgarHizi());
  Serial.println("m/s");
  Serial.print("Rain Fall (One Hour): ");
  Serial.print(YagmurOrt());
  Serial.println("mm  ");
  Serial.print("Rain Fall (24 Hour): ");
  Serial.print(RainfallOneDay());
  Serial.println("mm");
  Serial.print(" Sicaklik: ");
  Serial.print(Sicaklik());
  Serial.println("C  ");
  Serial.print("Nem: ");
  Serial.print(Nem());
  Serial.println("%  ");
  Serial.print("Basinc: ");
  Serial.print(Basinc());
  Serial.println("hPa");
  Serial.println("");
  Serial.println("");
}
