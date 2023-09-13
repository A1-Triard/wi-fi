![travis](https://travis-ci.org/A1-Triard/wi-fi.svg?branch=master)

Installation
============

```sh
# ./install
```

Prepare for use
===============

```sh
$ mkdir -p ~/wifi
$ cat > ~/wifi/my_wifi << "EOF"
> #!/usr/local/bin/wi-fi
> 
> network={
>     ssid="my_wifi"
>     psk="my password"
> }
> EOF
$ chmod +x ~/wifi/my_wifi
$ ln -s /usr/local/bin/wi-fi ~/wifi/disconnect
```

Using
=====

Connect
-------

```sh
$ ~/wifi/my_wifi
```

Disconnect
----------

```sh
$ ~/wifi/disconnect
```
