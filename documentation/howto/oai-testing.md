# Running the proxy without Alsoran
Instructions from https://github.com/EpiSci/oai-lte-5g-multi-ue-proxy.
To set up extra address:
  sudo ip addr add 127.0.0.2 dev lo:
This looks to be a better config file to use:
  gnb.sa.band78.106prb.l2sim.conf
To sniff the packets to/from the proxy:
  sudo tcpdump -w proxy.pcap -i lo port 50601 or port 50600

Hits https://github.com/EpiSci/oai-lte-5g-multi-ue-proxy/issues/30.

# Running DU against Alsoran
As usual:
  sudo tcpdump -w alsoran.pcap -i lo port 38472 or port 38412 or port 38462 or port 38462 or port 2152

Run Alsoran:
  ~/projects/alsoran/gnb-cu$ cargo run -- --mcc 222 --mnc 01

Run nr-softmodem:
  sudo -E ./ran_build/build/nr-softmodem -O ../ci-scripts/conf_files/alsoran/gnb-du.sa.band78.106prb.nfapi.conf --nfapi VNF --sa --emulate-l1



The key part of the config file is the MACRLCs structure which must use f1 as the northbound interface and nfapi as the southbound interface.  There is no readymade config file that has this combination.

MACRLCs = (
  {
    num_cc           = 1;
    tr_n_preference  = "f1";
    local_n_if_name = "eth0";
    local_n_address = "127.0.0.1";
    remote_n_address = "127.0.0.1";
    local_n_portc   = 51111;
    local_n_portd   = 2153;
    remote_n_portc  = 38472;
    remote_n_portd  = 2153;

    tr_s_preference = "nfapi";
    local_s_if_name  = "lo:";
    remote_s_address = "127.0.0.1"; // pnf addr [!]
    local_s_address  = "127.0.0.2"; // vnf addr
    local_s_portc    = 50601; // vnf p5 port
    remote_s_portc   = 50600; // pnf p5 port [!]
    local_s_portd    = 50611; // vnf p7 port [!]
    remote_s_portd   = 50610; // pnf p7 port [!]

    pusch_TargetSNRx10          = 200;
    pucch_TargetSNRx10          = 200;
    ulsch_max_frame_inactivity = 1;
  }
);
