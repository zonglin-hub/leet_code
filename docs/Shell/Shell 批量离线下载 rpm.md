```sh
#!/bin/bash

do=`pwd`

SETNAME=(
    vim gcc pam zlib net-tools glibc fontconfig fontconfig-devel freetype freetype-devel wget bzip2 gnutls dos2unix nano
    wireshark vsftpd cairo-devel libjpeg-turbo-devel libjpeg-devel libpng-devel libtool uuid-devel expect uuid lrzsz
    freerdp-devel pango-devel libssh2-devel libtelnet-devel libvncserver-devel libwebsockets-devel
    pulseaudio-libs-devel openssl-devel libvorbis-devel libwebp-devel keepalived ntp nmap
    ansible git svn zip unzip ffmpeg ffmpeg-devel make autoconf pcre-devel ipvsadm psmisc ncurses-devel
    pciutils xorg-x11-font-utils libXfont libtiff gstreamer samba samba-client samba-swat
)

download() {
    for((i=0;i<${#SETNAME[*]};i++)); do
        pwd=$do/${SETNAME[i]}
        mkdir -p $pwd    # c创建下载 rpm 对应的目录
        yum install --downloadonly --downloaddir=$pwd "${SETNAME[i]}"    
    done
}

case "$1" in
  download)
        download
        ;;
      *)
            echo "download" >&2
            exit 1
    esac
    exit 0

    
```
