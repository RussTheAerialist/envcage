#include <Arduino.h>
#include <WiFi.h>
#include <WiFiUdp.h>
#include <SimpleDHT.h>
#include <NTPClient.h>
#include <PubSubClient.h>
#include "ssid.h"

#ifndef SSID_SET
const char* ssid = "";
const char* passwd = "";
#endif // SSID_SET

const char* broker = "test.mosquitto.org";
uint16_t brokerPort = 1883;
int dhtPin = 2; // TODO: Find the right pin
SimpleDHT11 sensor(dhtPin); // TODO: Configure this once I find my existing DHT
// TODO: Figure out how to get the MAC address or Unique ID for the device
float temperature = 0, humidity = 0;
WiFiUDP ntpUdp;
NTPClient ntpClient(ntpUdp);

WiFiClient mqttClient;
PubSubClient mqtt(mqttClient);
String macAddress;
String topic = "monitor/";

void setup_wifi() {
  WiFi.begin(ssid, passwd);
  // Check wi-fi is connected to wi-fi network
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.print(".");
  }
  Serial.println("");
  Serial.println("WiFi connected successfully");
  Serial.print("Got IP: ");
  Serial.println(WiFi.localIP());  //Show ESP32 IP on serial
  macAddress = WiFi.macAddress();
  Serial.println(macAddress); // Show Mac Address
  topic += macAddress;

}

void setup_ntp() {
  ntpClient.begin();
}

void mqtt_connect() {
  while(!mqtt.connected()) {
    Serial.print("MQTT Connection..");
    if (mqtt.connect("envcage-sensor")) {
      String status = topic + "/status";
      String message = "{ \"state\": \"online\", \"id\": \"";
      message += macAddress;
      message += "\" }";
      Serial.println("connected");
      mqtt.publish(status.c_str(), message.c_str());
    } else {
      Serial.print("failed, rc=");
      Serial.println(mqtt.state());
      delay(5000);
    }
  }
}

void setup_mqtt() {
  mqtt.setServer(broker, brokerPort);
  mqtt_connect();
}

void setup() {
  Serial.begin(11520);
  Serial.print("Trying to connect ");
  Serial.println(ssid);

  setup_wifi();
  setup_ntp();
  setup_mqtt();
}

void mqtt_send() {
  String reading = topic + "/reading";
  String message = "{ ";
  message += "\"id\": \"";
  message += macAddress;
  message += "\", \"created\": ";
  message += ntpClient.getEpochTime();
  message += "\", \"temperature\": ";
  message += (float)temperature;
  message += ", \"humidity\": ";
  message += (float)humidity;
  message += "}";

  mqtt.publish(reading.c_str(), message.c_str());
}

void loop() {
  unsigned long startTS = millis();
  ntpClient.update();
  int err = SimpleDHTErrSuccess;
  if ((err = sensor.read2(&temperature, &humidity, NULL)) != SimpleDHTErrSuccess) {
    Serial.print("Read DHT22 failed, err="); Serial.println(err);delay(2000);
    // return;
  }

  if (!mqtt.connected()) {
    mqtt_connect();
  }

  Serial.print(ntpClient.getFormattedTime());
  Serial.print(": T=");
  Serial.print((float)temperature);
  Serial.print(" *C, H=");
  Serial.print((float)humidity);
  Serial.println(" RH");

  mqtt_send();

  mqtt.loop();
  delay(60000 - (millis() - startTS)); // One recording per minute is probably enough
}