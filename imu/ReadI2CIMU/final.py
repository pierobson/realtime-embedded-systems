from smbus2 import SMBus, i2c_msg
from gps import *
import time
import struct
import threading

ARDUINO_ADDRESS = 12

def read_gps():
        conn = gps(mode=WATCH_ENABLE)

        while True:
                data = conn.next()
                if data['class'] == 'TPV':
                        if hasattr(data, 'lat') and hasattr(data, 'lon'):
                                print("Lat: " + str(data["lat"]) + " Long: " + str(data["lon"]))
                                time.sleep(1)


def read_imu():
        with SMBus(1) as bus:
                while True:
                        try:
                                data = bus.read_i2c_block_data(ARDUINO_ADDRESS, 0, 12)
                                yaw = struct.unpack('f', bytearray(data[0:4]))[0]
                                pitch = struct.unpack('f', bytearray(data[4:8]))[0]
                                roll = struct.unpack('f', bytearray(data[8:12]))[0]
                                print("Yaw: " + str(yaw) + " Pitch: " + str(pitch) + " Roll: " + str(roll))
                                time.sleep(1)
                        except:
                                print("Failed to read from IMU")
                                time.sleep(0.1)

# Setup the threads
imu_thread = threading.Thread(target=read_imu)
gps_thread = threading.Thread(target=read_gps)

imu_thread.start()
gps_thread.start()