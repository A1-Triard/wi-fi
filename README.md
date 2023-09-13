FreeBSD
-------

1. Create `wlan0` interface using `/etc/rc.conf` as described in the Handbook:
 
   ```sh
   # sysrc wlans_iwn0="wlan0"
   ```

2. Compile and install `wi-fi` binary:

   ```sh
   $ cd freebsd
   # ./install
   ```

3. Create `wi-fi` directory in user `$HOME`

4. Create executable config file in `$HOME/wi-fi` using `freebsd/example_config` as template.

5. Create `disconnect` executable:

   ```sh
   $ ln -s /usr/local/bin/wi-fi $HOME/wi-fi/disconnect
   ```

6. Restart network:

   ```sh
   # service netif restart
   ```

7. To connect or disconnect, run appropriate file in `$HOME/wi-fi` directory.

Linux
-----

1. Install `wi-fi` script:

   ```sh
   $ cd linux
   # ./install
   ```

2. Create `wi-fi` directory in user `$HOME`

2. Create executable config file in `$HOME/wi-fi` using `linux/example_config` as template.

3. Create `disconnect` executable:

   ```sh
   $ ln -s /usr/bin/wi-fi $HOME/wi-fi/disconnect
   ```

4. Add user to the `network` group.

5. Reboot.

6. To connect or disconnect, run appropriate file in `$HOME/wi-fi` directory.
