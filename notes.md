# Basic setup

## burn sd card image
## create ssh and wpa_supplicant.conf files in /boot


cat >/media/timd/bootfs/userconf <<'EOF'
app:$6$rUGoix/jBEinAHLb$Qlhe/Rz8F1A1l6xYj1IXr52DF310oW52nYfXInOeyrEEg8/jMUxPlI3hYr9c.ThJD2jtis9C2vwXKRT3OCnYf1
EOF

network={
 scan_ssid=1
 ssid="dkz"
 psk="darkchocolate"
}
EOF

touch /media/timd/bootfs/ssh

## first setup

connect ethernet cable, boot and login as app@raspberrypi.local

use raspi-config to
 - change hostname to hifi
 - configure wifi

##

In /boot/config.txt

```
#dtparam=audio=on
dtparam=i2s=on
dtoverlay = vc4-kms-v3d,noaudio
```

and also increase i2c speed

```
dtparam=i2c_arm=on,i2c_arm_baudrate=400000
```

reboot 


## Login

ssh app@hifi.local
(password in keepass)


## show audio devices

```
aplay -l
```



Then install

https://github.com/nicokaiser/rpi-audio-receiver


Adjust spotify sound level via pulse audio

```
pactl -- set-sink-volume 1 175%
```

Also want volume to not be max, so change

```
LIBRESPOT_INITIAL_VOLUME="60"
```

in /etc/raspotify/conf

Note confusion between controlling from ios device: ie airplay vs spotify connect. There are 2 different
system services:
 
```
journalctl -u shairport-sync.service
journalctl -u raspotify.service
journalctl -u pulseaudio.service
```

The librespot command can take an `--onevent` argument that calls a script whenever the spotify
track changes - ie making it possible to update a display.


## SSD1306 display

### Setup i2c bus

Enable i2c bus via `raspi-config`

Then install `sudo apt-get install i2c-tools`

and dump devices:

```
$ sudo i2cdetect  -y 1

     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:                         -- -- -- -- -- -- -- -- 
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
40: -- -- -- -- -- -- -- -- -- -- -- -- UU -- -- -- 
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
70: -- -- -- -- -- -- -- --  
```

### Connect display:

```
OLED Pin     Fn      GPIO Pin
-----------------------------
1            VCC     1
2            GND     6
3            SCL     5
4            SDA     3
```

After wiring:

```
app@hifi:~ $ sudo i2cdetect  -y 1
     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:                         -- -- -- -- -- -- -- -- 
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
30: -- -- -- -- -- -- -- -- -- -- -- -- 3c -- -- -- 
40: -- -- -- -- -- -- -- -- -- -- -- -- UU -- -- -- 
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
70: -- -- -- -- -- -- -- --  
```

## Cross compiling rust for rpi target from linux

For rpi 4, target is `armv7-unknown-linux-gnueabihf`

```
cd hifi-display
cargo install cross
cross build --target armv7-unknown-linux-gnueabihf --release
```

## Setting up display as a system service

On the rpi as root

```
cat >/etc/systemd/system/hifidisplay.service <<EOF
[Unit]
Description=Hifi i2c display driver
After=pulseaudio.service
StartLimitIntervalSec=0
[Service]
Type=simple
Restart=always
RestartSec=1
User=app
ExecStart=/home/app/hifi-display

[Install]
WantedBy=multi-user.target
EOF
```

then

```
systemctl start hifidisplay.service
systemctl enable hifidisplay.service
```
