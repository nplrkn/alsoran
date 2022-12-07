# Alsoran

Alsoran is a Rust implementation of the gNodeB Centralized Unit (gNB-CU) of the 5G Radio Access Network (RAN).

This is the component that manages the radio access of 5G User Equipment (UEs).  It connects UEs to a 5G Core. 

In the control plane, the gNB-CU communicates with  
- the 5G Core Access Management and Mobility function (AMF) using the NGAP protocol
- the gNodeB Distributed Unit (gNB-DU) using the F1AP protocol
- User Equipment (UEs) using the RRC protocol, encapsulated in F1AP
- other gNodeB using the Xn-C interface.

The gNB-CU control and user plane (gNB-CU-CP and gNB-CU-UP) are interconnected by the E1 interface.  However, Alsoran CU has no userplane, yet. 

## What is different about Alsoran?

It's written in Rust and it has a "scale-out single hop" design.

"Scale-out" means that it has multiple interchangeable stateless worker processes.  Any request can be processed by any worker and no worker is a single point of failure.  A Coordinator process coordinates the interface management exchanges of the workers when the topology changes.  The motivation is scalability and fault tolerance.

"Single hop" means that, in the mainline case, a message is processed by a single worker (rather than chained through multiple microservices or load balancers).  Consequently each Alsoran CU-CP worker has to have its own SCTP connections to the AMF, the DU and the CU-UP.  The motivation is execution speed and system simplicity.

Rust is an obviously attractive choice of language for new O-RAN development.  The main barrier to entry is the SCTP and ASN.1 based protocols.  This project attempts to prove that this barrier is surmountable.

## Current support

- UE registration demo against free5GC.
- Scale out of GNB-CU workers using multiple TNLAs.
- Ue state in Redis datastore.
- Session setup (TS 23.502, figure 4.3.2.2.1-1).
- Procedures: NG Setup, RAN Configuration Update, F1 Setup, E1 Setup, Initial Access, Uplink NAS, Downlink NAS, Initial Context Setup, Pdu Session Resource Setup, AMF Status Indication, GNB CU Configuration Update, GNB CU CP Configuration Update.
- Connection management stack 
- ASN.1 libraries for NGAP, E1AP, F1AP and RRC.
- Rust ASN.1 autogenerator (written in Python).

Generally only the success cases are covered, and there are a lot of 'To Dos'.

## Building and running integration tests

The build relies on `lld` to reduce linker memory needs.  You will either need to install it (`sudo apt install lld` or similar), or edit .cargo/config to remove the `target.x86_64-unknown-linux-gnu` config and revert to plain `cc` linking.

`cargo test` runs the integration test suite, minus the live Redis test.  

To run the live Redis test, `cargo test live_redis -- --ignored`.  For this to pass, you need to have `redis-server` in your path.  Get Redis here: https://redis.io/docs/getting-started/.

## A quick tour

The following test shows the Alsoran CU-CP carrying out UE registration and session establishment.
```
RUST_LOG=info cargo test successful_pdu_session_setup --test pdu_session -- --nocapture
```

This test shows two workers starting up, and the Coordinator instructing the workers how to initialize their NGAP, E1AP and F1AP interfaces.
```
RUST_LOG=info cargo test two_workers_base --test two_workers -- --nocapture
```

You can packet capture during these tests by running the following in parallel. 
```
sudo tcpdump -w alsoran.pcap -i lo port 38472 or port 38412 or port 38462
```
...then Ctrl-C at the end of the test and open alsoran.pcap in Wireshark.

To run the live registration against free5GC takes a bit more setup - see the [demo instructions](documentation/howto/free5GC-testing.md).

Finally you might want to browse the design notes in documentation/design, which give an idea of the design thinking that has gone into Alsoran so far.

## Contributing

So far, Alsoran has been developed by a single person, with a few hours a week to spare.  If you want to make it more useful for your own project, please consider contributing.  Start by creating a Github issue or discusion to propose the change you want to make.

The [backlog](documentation/backlog.md) shows the main items being worked on and also tracks areas of tech debt. 

The instructions for regenerating the two OpenAPI interfaces are in [OpenAPI generation](documentation/howto/OpenAPI%20generation.md).

## 3GPP and O-RAN specifications

Alsoran protocol handling and workflow logic is based on the following specifications.  

-  3GPP TS23.501 - System architecture for the 5G System
-  3GPP TS23.502 - Procedures for the 5G System
-  3GPP TS38.300 - NR and NG-RAN Overall Description
-  3GPP TS38.323 - Packet Data Convergence Protocol (PDCP) Specification
-  3GPP TS38.331 - Radio Resource Control (RRC) protocol specification
-  3GPP TS38.401 - NG-RAN; Architecture description 
-  3GPP TS38.412 - NG signalling transport 
-  3GPP TS38.413 - NG Application Protocol (NGAP)
-  3GPP TS38.462 - E1 signalling transport
-  3GPP TS38.463 - E1 Application Protocol (E1AP)
-  3GPP TS38.472 - F1 signalling transport
-  3GPP TS38.473 - F1 Application Protocol (F1AP)
-  O-RAN.WG5.C.1 - NR C-plane profile
