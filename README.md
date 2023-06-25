# Alsoran

Alsoran is a Rust implementation of the gNodeB Centralized Unit (gNB-CU) of the 5G Radio Access Network (RAN).

The gNodeB is the component that manages the radio access of 5G User Equipment (UEs) and connects them to a 5G Core. 

This project is currently a proof of concept and not yet a fully functional gNB-CU.

## Current support
- UE registration demo against free5GC.
- PDU session setup (TS 23.502, figure 4.3.2.2.1-1).
- Relay of userplane packets between GNB-DU and UPF.
- Separate CU-CP and CU-UP communicating using E1.
- Scale-out of GNB-CU-CP workers using multiple TNLAs.
- ASN.1 libraries for NGAP, E1AP, F1AP and RRC.
- Rust ASN.1 autogenerator (written in Python).
- UE state in Redis datastore.
- [Triangular redirection](documentation/design/triangular-redirection.md)
- SCTP connection management stack 

Generally only the success cases are covered, and there are a lot of 'To Dos'.

### Procedure support
#### CU-CP
-  NG Setup
-  RAN Configuration Update
-  F1 Setup
-  E1 Setup
-  Initial Access
-  Uplink NAS
-  Downlink NAS
-  Initial Context Setup
-  Pdu Session Resource Setup
-  AMF Status Indication
-  GNB CU Configuration Update
-  GNB DU Configuration Update
-  GNB CU CP Configuration Update.
#### CU-UP
-  E1 Setup
-  Bearer Context Setup
-  Bearer Context Modification
-  GNB CU CP Configuration Update
-  Downlink user data
-  Uplink user data

## What's different about Alsoran?

It's written in Rust and it has a "scale-out single hop" design.

"Scale-out" means that it has multiple interchangeable stateless worker processes.  A request can be processed by any worker and no worker is a single point of failure.  A Coordinator process coordinates the interface management exchanges of the workers when the topology changes.  The motivation is scalability and fault tolerance.

"Single hop" means that, in the mainline case, a message is processed by a single worker (rather than chained through multiple microservices or load balancers).  Each Alsoran CU-CP worker has its own SCTP connection to the AMF, the DU and the CU-UP.  The motivation is speed and system simplicity.

Rust is an attractive choice of language for new O-RAN development.  The main barrier to entry is the ASN.1 and SCTP based protocols.  This project attempts to prove that this barrier is surmountable.


## Building and running integration tests

The build relies on `lld` to reduce linker memory needs.  You will either need to install LLD (`sudo apt install lld` or similar), or edit .cargo/config to remove the `target.x86_64-unknown-linux-gnu` config, which reverts to plain `cc` linking.

`cargo test` runs the integration test suite, minus the live Redis test.  

To run the live Redis test, `cargo test live_redis -- --ignored`.  For this to pass, you need to have `redis-server` in your path.  Get Redis here: https://redis.io/docs/getting-started/.

## A quick tour

The following test shows the Alsoran CU-CP and CU-UP carrying out UE registration, session establishment and userplane forwarding.
```
RUST_LOG=info cargo test successful_pdu_session_setup --test pdu_session -- --nocapture
```

This test shows two workers starting up, and the Coordinator instructing the workers how to initialize their NGAP, E1AP and F1AP interfaces.
```
RUST_LOG=info cargo test two_workers_base --test two_workers -- --nocapture
```

You can packet capture during these tests by running the following in parallel. 
```
sudo tcpdump -w alsoran.pcap -i lo port 38472 or port 38412 or port 38462 or port 38462 or port 2152
```
...then Ctrl-C at the end of the test and open alsoran.pcap in Wireshark.

To run the live registration against free5GC takes a bit more setup - see the [demo instructions](documentation/howto/free5GC-testing.md).

Finally you might want to browse the design notes in documentation/design, which give an idea of the design thinking that has gone into Alsoran so far.

## Contributing

If you would like to contribute, start by creating a Github issue or discusion to propose the change you want to make.

The [backlog](documentation/backlog.md) shows the main items being worked on and also tracks areas of tech debt. 

The instructions for regenerating the two OpenAPI interfaces are in [OpenAPI generation](documentation/howto/OpenAPI%20generation.md).

## 3GPP and O-RAN specifications

Alsoran protocol handling and workflow logic is based on the following specifications.  

-  3GPP TS23.501 - System architecture for the 5G System
-  3GPP TS23.502 - Procedures for the 5G System
-  3GPP TS29.281 - General Packet Radio System (GPRS) Tunnelling Protocol User Plane (GTPv1-U)
-  3GPP TS37.324 - Service Data Adaptation Protocol (SDAP) specification
-  3GPP TS38.300 - NR and NG-RAN Overall Description
-  3GPP TS38.323 - Packet Data Convergence Protocol (PDCP) Specification
-  3GPP TS38.331 - Radio Resource Control (RRC) protocol specification
-  3GPP TS38.401 - NG-RAN; Architecture description 
-  3GPP TS38.412 - NG signalling transport 
-  3GPP TS38.413 - NG Application Protocol (NGAP)
-  3GPP TS38.414 - NG data transport
-  3GPP TS38.415 - PDU Session User Plane Protocol
-  3GPP TS38.462 - E1 signalling transport
-  3GPP TS38.463 - E1 Application Protocol (E1AP)
-  3GPP TS38.472 - F1 signalling transport
-  3GPP TS38.473 - F1 Application Protocol (F1AP)
-  3GPP TS38.474 - F1 data transport
-  O-RAN.WG5.C.1 - NR C-plane profile
