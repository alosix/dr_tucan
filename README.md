Dr Tucan

This is a pretty simple app. It listens for the Dirt Rally(2) udp output and emits it over canbus on a pi in a motec like format.

Useful if you want a bling motec dash on your sim rig..

Or if you're cheap.. Using the race dash stuff here to display it as well.

https://www.waveshare.com/w/upload/2/29/RS485-CAN-HAT-user-manuakl-en.pdf

Add 
dtparam=spi=on
dtoverlay=mcp2515-can0,oscillator=8000000,interrupt=25,spimaxfrequency=1000000

to /boot/config.txt and reboot, should come up as can0 in ifconfig