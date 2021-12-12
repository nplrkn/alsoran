# Interop testing
We use free5GC.  Set up using the instructions at https://www.free5gc.org/installations/stage-3-free5gc-install/.

The kernel module, and hence UPF, don't work on WSL.  However, it is still possible to get quite far through session setup.

## Wireshark
Capture NGAP to ngap.cap.
```
sudo tcpdump -i lo port 38412 -w ngap.cap
```

## Test setup
```
cd free5GC
# Start mongodb
sudo service mongodb start
# Start NRF and AMF
bin/nrf &
bin/amf
```
Note the following log line from the AMF

```
2021-11-28T12:45:16+01:00 [INFO][AMF][NGAP] Listen on 127.0.0.1:38412
```

Run alsoran.  It automatically connects to port 38412 on localhost.
```
cargo run
```