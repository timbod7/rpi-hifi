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

reboot 


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

Note confusion between controlling from ios device: ie airplay vs spotify connect. There are 2 different
system services:
 
```
journalctl -u shairport-sync.service
journalctl -u raspotify.service
journalctl -u pulseaudio.service
```

The librespot command can take an `--onevent` argument that calls a script whenever the spotify
track changes - ie making it possible to update a display.
