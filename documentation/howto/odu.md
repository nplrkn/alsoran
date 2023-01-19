
```sh
~/projects/o-du-l2/build/odu$ make odu MACHINE=BIT64
~/projects/o-du-l2/build/odu$ make cu_stub NODE=TEST_STUB MACHINE=BIT64
~/projects/o-du-l2/build/odu$ make ric_stub NODE=TEST_STUB MACHINE=BIT64

# Set up the addresses.  These match the ones configured in o-du-l2/bin/odu/config/startup_config.xml
sudo ip addr add 192.168.130.80/32 dev eth0 label eth0:RIC_STUB
sudo ip addr add 192.168.130.81/32 dev eth0 label eth0:ODU
sudo ip addr add 192.168.130.82/32 dev eth0 label eth0:CU_STUB

# optional set up tcpdump
sudo tcpdump -w alsoran.pcap -i lo port 38472 or port 38412

# in terminal 1, projects/alsoran/gnb-cu-cp
cargo run -- --local-ip=192.168.130.82

# in terminal 2, projects/o-du-l2
bin/ric_stub/ric_stub

# in terminal 3, projects/o-du-l2
sudo bin/odu/odu

```