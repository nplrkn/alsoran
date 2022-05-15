# Backlog

This is focused on the next release (0.1).  For longer term items, put them in the roadmap or put a TODO in the code.

## TO DO
- Requests should be processed in parallel in separate tasks.
- Registration (23.502, figure 4.2.2.2-1) wrapped in UE Initial Access procedure (38.401, figure 8.1-1) working against free5GC as seen in ngap_setup.pcap.
- Session setup (23.502, figure 4.3.2.2.1-1).  (Requires N2 PDU Session Request/Response.)
- Review TODOs and remove commented out code

## MAYBE
- Enforce Rust docs (see .cargo/config commented out compiler option)
- Remodel SCTP API to follow the one in the webrtc-sctp crate.

## DONE
- Multi worker code (including coordinator) moved to a 'prototype' version leaving remaining code super simple
- NG Setup working again against free5GC using the instructions in [the free5gc howto](../howto/free5GC-testing.md).
- Add error messages to ASN.1 library + get it upstreamed
- GNB-CU code moved to a subdirectory (leaving code shared with DU or AMF at top level).
- Autogeneration of procedures etc
- Fix Address already in use warning seen in tests.  Refactor so that the listen() fails more obviously. 
- Test should fail if the F1AP SCTP bind fails 
- 'Worker confirms successul TNLA initialization' is output before output of AMF sending setup response
- send request instead of send pdu
- Test script as DU can send Setup and get response 
- Test script as DU can connect to CU
- Create callback client on coordinator
- Drive NG Setup from callback server
- Drive RAN configuration update from coordinator
- Start callback server on worker
- Develop callback in node control API.
- test the sctp receive function
- sort out TODOs and unwraps()
- commonize TNLA pool
- try out the Hampi library to decode the response at least?
- start fake amf
- get info / trace levels right
- get tests passing cleanly
- remove bindgen test cruft
- on startup, the worker should connect to node controller, get AMF address, connect
- fix typo refresh
- factor out logging and signal handling to common library
- treat that as a separate package within the same workspace for now
- create a basic POST API (later we add callback server)
- ok so we need to run OpenAPI generator
- Implement the node controller / investigate OpenAPI
- see if two alsorans can connect to one AMF
- documentation of features and level of testing
- socket OS errors getting hidden by anyhow
- Maintain + Retry 
- Successful NG Setup with Free5G.
- get rid of haphazard error conversion - see TODOs
- clean up sctp_association.rs
- Clippy
- send a fake ng setup and test the sctp receive function
- Properly close sockets when wrapper structs are dropped
- Stay up until Ctrl-C
- SCTP connect
- test against a free5GC
- add logging
- do two exchanges concurrently in UT
- explore is HA + use of SCTP.