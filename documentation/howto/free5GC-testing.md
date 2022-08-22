# Demo against free5GC

The current (basic) function of Alsoran can be demonstrated against free5GC, using tcpdump to capture the NGAP and F1AP exchanges.  Alsoran includes a tool `gnbdu-sim` to drive it in this setup.  `gnbdu-sim` drives the F1 and RRC interfaces of the Alsoran CU, and knows how to provision a UE in the free5GC subscriber database.

The following instructions below were created for a WSL environment without intalling the free5GC kernel module.  They have not been comprehensively tested either against new versions of free5GC or in different environments - please raise a Github issue if you hit problems or have improvements.

## Get free5GC (one-off)
Follow the instructions at https://www.free5gc.org/installations/stage-3-free5gc-install/ to install mongodb and make free5GC.  Because we are not going to use the userplane, you can skip 'Setting up Networking', and the make install of the kernel module. 

## Alsoran demo
Open four terminals.  

In terminal 1, in the free5GC directory.
```
# Start MongoDB if not done already
sudo service mongodb start

# Start NFs
bin/nrf &
bin/udm &
bin/udr &
bin/ausf &
bin/pcf &
bin/amf &
```

In terminal 2, start capturing NGAP and F1AP over the loopback interface.
```
sudo tcpdump -w alsoran.pcap  -i lo port 38472 or port 38412
```

In terminal 3, in the alsoran directory, run Redis and the Alsoran GNB-CU.  On startup the GNB-CU will connect to the AMF and perform NG Setup.
```
redis-server &
cargo run --bin gnbcu
```

In terminal 4, in the alsoran directory, run `gnbdu-sim` tool.  This provisions a UE in MongoDB, connects as a DU, drives a UE registration procedure and then exits.
```
cargo run --bin gnbdu-sim
```

In terminal 2, hit Ctrl-C to finish the tcpdump.  You can now view `alsoran.pcap` in Wireshark.  The 'Malformed Packet' errors are a known problem.

To clean up,
- Ctrl-C in terminal 3 to shut down Alsoran
- `fg` and Ctrl-C in terminal 3 to shut down Redis, and `rm dump.rbd` to clean up its saved state
- in terminal 1, `kill $(jobs -p)` to terminate the free5GC network functions that are running in the background
- `sudo service mongodb stop` to terminate MongoDB.