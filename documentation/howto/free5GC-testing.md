# Interop testing
We use free5GC.  Set up using the instructions at https://www.free5gc.org/installations/stage-3-free5gc-install/.

The kernel module, and hence UPF, don't work on WSL.  However, it is still possible to get quite far through session setup.

## Start free5GC
```
cd free5GC
# Start mongodb
sudo service mongodb start
# Start NFs
bin/nrf &
bin/udm &
bin/udr &
bin/ausf &
bin/pcf &
bin/amf
```
Note the following log line from the AMF

```
2021-11-28T12:45:16+01:00 [INFO][AMF][NGAP] Listen on 127.0.0.1:38412
```

## Wireshark
Capture NGAP and F1AP.
```
sudo tcpdump -w alsoran.pcap  -i lo port 38472 or port 38412
```

## Alsoran

Run the alsoran GNB-CU.  It automatically connects to port 38412 on localhost.
```
cd gnbcu
cargo run

# In a separate terminal
cd gnbdu-sim
cargo run
```