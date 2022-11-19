# Alsoran

Alsoran is a Rust implementation of the gNodeB Centralized Unit (gNB-CU) of the 5G Radio Access Network (RAN).

This is the component that helps 5G endpoints (UEs) manage their radio connections and request network access.  It connects UEs to the 5G Core that is responsible for the local radio coverage where the UE finds itself. 

In the control plane, the gNB-CU communicates with  
- the 5G Core Access Management and Mobility function (AMF) using the NGAP protocol
- the gNodeB Distributed Unit (gNB-DU) using the F1AP protocol
- User Equipment (UEs) using the RRC protocol, encapsulated in F1AP
- other gNodeB using the Xn-C interface.

The gNB-CU control and user plane (gNB-CU-CP and gNB-CU-UP) are interconnected by the E1 interface.  However, Alsoran CU has no userplane, yet. 

## Current status

-  Alsoran can perform some basic procedures: NG Setup, F1 Setup, and registration of a single UE.

-  The supported procedures can be demonstrated against open source 5G core Free5GC.  

-  There is functional but incomplete SCTP connection management and ASN.1 libraries for NGAP, F1AP and RRC.

-  The ASN.1 generator is also included in the project.  Functional but incomplete, most notably in the area of extension fields (forward compatibility).  This is in Python, and is rather messy.

-  Storage of UE Context in Redis datastore.

## Integration tests and Redis

`cargo test` runs the integration test suite, minus the live Redis test.  

When running an individual test it is recommended to enable info level tracing.  For example,

```
RUST_LOG=info cargo test two_ues_register_sequentially --test two_ues -- --nocapture
```

To reduce linker memory needs, `lld` is used as the linker.  You will either need to install lld (`sudo apt install lld` or similar), or edit .cargo/config to remove the `target.x86_64-unknown-linux-gnu` config to revert to plain `cc` linking. 

The live Redis test is ignored by default.  To run it, `cargo test live_redis -- --ignored`.  For this to pass, you need to have `redis-server` in your path.  Get Redis here: https://redis.io/docs/getting-started/.

## Up next

-  Scale out of control plane.

## Contributing

So far, Alsoran has been developed by a single person, with only a few hours a week to spare, so progress is slow.  If you want to speed it up or make it more useful for your own project, please consider contributing!  Start by creating a Github issue to propose the change you want to make.

## 3GPP specifications

The key 3GPP specifications are as follows.

-  TS23.501 - System architecture for the 5G System
-  TS23.502 - Procedures for the 5G System
-  TS38.300 - NR and NG-RAN Overall Description
-  TS38.323 - Packet Data Convergence Protocol (PDCP) Specification
-  TS38.331 - Radio Resource Control (RRC) protocol specification
-  TS38.401 - NG-RAN; Architecture description 
-  TS38.412 - NG signalling transport 
-  TS38.413 - NG Application Protocol (NGAP)
-  TS38.472 - F1 signalling transport
-  TS38.473 - F1 Application Protocol (F1AP)
