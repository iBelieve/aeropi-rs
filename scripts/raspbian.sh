#! /bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

SDCARD=$1

cd "$(dirname $SCRIPT_DIR)"

if [ -z $SDCARD ]; then
    echo "ERROR: Expecteded path to sdcard /dev/"
    exit -1
fi

if [ ! -f $SDCARD ]; then
    echo "ERROR: Device not found: $SDCARD"
    exit -1
fi

if [ ! -f data/wpa_supplicant.conf ];  then
    echo "ERROR: Copy data/wpa_supplicant.example.conf to data/wpa_supplicant.conf and edit as necessary"
    exit -1
fi

unmount() {
    for DEV in $(ls $SDCARD?); do
        if mount | grep $DEV > /dev/null; then
            sudo umount $DEV
        fi
    done
}

download() {
    if ! compgen -G "data/*-stretch-lite.img" > /dev/null; then
        ZIP=$MOUNT-stretch-lite.zip

        echoing "Downloading latest raspbian stretch lite"

        wget -c https://downloads.raspberrypi.org/raspbian_lite_latest -O $ZIP
        unzip $ZIP
        rm $ZIP
    fi

    export IMG=$(ls data/*-stretch-lite.img)
}

flash() {
    echo "Flashing raspbian to $SDCARD"
    sudo dd bs=4M if=$IMG of=$SDCARD status=progress conv=fsync
    sudo sync
}

customize() {
    echo "Customizing raspbian install"

    MOUNT=data/raspbian

    if [ ! -d $MOUNT ]; then mkdir $MOUNT; fi
    sudo mount ${SDCARD}2 $MOUNT
    sudo mount ${SDCARD}1 $MOUNT/boot

    sudo touch $MOUNT/boot/ssh
    echo aeropi | sudo tee $MOUNT/etc/hostname > /dev/null
    sudo cp data/wpa_supplicant.conf $MOUNT/boot

    if [ -f ~/.ssh/id_rsa.pub ]; then
        echo "Copying ~/.ssh/id_rsa.pub to authorized_keys"
        cat ~/.ssh/id_rsa.pub | sudo tee -a $MOUNT/home/pi/.ssh/authorized_keys > /dev/null
    fi

    echo 'Set user password for pi@aeropi:'
    PASS=$(python -c 'import crypt,getpass; print(crypt.crypt(getpass.getpass(), crypt.mksalt(crypt.METHOD_SHA512)))')

    sudo sed -i -e  "s,^pi:[^:]*:,pi:$PASS:," data/raspbian/etc/shadow

    sudo umount $SDCARD?
    sudo sync
}


unmount
download
flash
customize
