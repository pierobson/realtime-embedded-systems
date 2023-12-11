from smbus2 import SMBus, i2c_msg
import time
import struct

ARDUINO_ADDRESS = 12

def read(bus):
        data = bus.read_i2c_block_data(ARDUINO_ADDRESS, 0, 12)
        yaw = struct.unpack('f', bytearray(data[0:4]))[0]
        pitch = struct.unpack('f', bytearray(data[4:8]))[0]
        roll = struct.unpack('f', bytearray(data[8:12]))[0]

        print("Yaw: " + str(yaw) + " Pitch: " + str(pitch) + " Roll: " + str(roll))

with SMBus(1) as bus:
        while(True):
                read(bus)
                time.sleep(5)
