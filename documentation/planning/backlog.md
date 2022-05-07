# Backlog


## TO DO
- Autogeneration of procedures etc
- Add error messages to ASN.1 library + get it upstreamed
- TODOs
- Design sequencing of interface startup with multiple workers in Connection Management.md
- F1 connection management with 2nd worker
- test, with a single AMF and DU, all variants of connection ordering and 2nd worker addition
- make tests time out rather than hang
- regression test connection failures and retries
- pass logger in context
- regression testing of maintain and retry scenarios

## DONE
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
- see if two alsorans can connect to one AMF - yes.  though dubious we are sending two duplicate setups.
  2021-12-10T09:09:50+01:00 [INFO][AMF][NGAP] [AMF] SCTP Accept from: 127.0.0.1/172.24.71.65:36674
  2021-12-10T09:09:50+01:00 [INFO][AMF][NGAP] Create a new NG connection for: 127.0.0.1/172.24.71.65:36674
  2021-12-10T09:09:50+01:00 [INFO][AMF][NGAP][127.0.0.1/172.24.71.65:36674] Handle NG Setup request
  2021-12-10T09:09:50+01:00 [INFO][AMF][NGAP][127.0.0.1/172.24.71.65:36674] Send NG-Setup response
  2021-12-10T09:09:54+01:00 [INFO][AMF][NGAP] [AMF] SCTP Accept from: 127.0.0.1/172.24.71.65:60194
  2021-12-10T09:09:54+01:00 [INFO][AMF][NGAP] Create a new NG connection for: 127.0.0.1/172.24.71.65:60194
  2021-12-10T09:09:54+01:00 [INFO][AMF][NGAP][127.0.0.1/172.24.71.65:60194] Handle NG Setup request
  2021-12-10T09:09:54+01:00 [INFO][AMF][NGAP][127.0.0.1/172.24.71.65:60194] Send NG-Setup response
- [started] documentation of features and level of testing
- socket OS errors getting hidden by anyhow
- Maintain + Retry - i.e. test plan is
  
  1. start up Alsoran first then Free5GC [pass]
  2. start up free5G first then Alsoran [pass]
  3. kill and restart Free5GC [fail - doesn't notice]

- Successful NG Setup with Free5G.
- get rid of haphazard error conversion - see TODOs
- clean up sctp_association.rs
- Clippy
- send a fake ng setup and test the sctp receive function
  - by running wireshark and getting byte dump
    - wireshark Copy as hex stream, then hex::decode()
         00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140
- Properly close sockets when wrapper structs are dropped
- Stay up until Ctrl-C
- SCTP connect
- test against a free5GC
- add logging
- do two exchanges concurrently in UT
- most important area to explore is HA + use of SCTP.  Key point is that two instances of a single CU must share the same UE context, and a CU cannot assume it will receive a UE associated response on the same association as the one it sent the request on.
