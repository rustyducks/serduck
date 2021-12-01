# Serduck

Connect a serial port to an UDP socket

cargo run -- -s /dev/ttyUSB0 -b 115200 -u 0.0.0.0:3456

- **-s** : serial port
- **-b** : baudrate
- **-u** : addr:port. addr is the address to listen to. Use 0.0.0.0 to listen to anyone. 
- **-t** : NOT YET IMPLEMENTED! transport to use with the serial: ducklink or xbee.

Clients are discovered when they initiate a connection. They must initiate the connexion to receive messages. (They must send any paquet to the correct port).


