# Single process GNB-CU

# NEXT UP
- RRC connection release
- UE context release
- Testing of Session/context releases on different worker
- Paging

# TECH DEBT
## CU-UP and O-RAN O-DU interop
- With O-RAN-SC ODU
- multiple TNLAs on CU-UP?
- >1 CU-UP?
- avoid need for recompile of ODU by enabling O1 (but we need to recompile it anyway to set ratio = 20)
- document a method that other people could use to test ODU
- don't set up SRB + 2 DRBs if all we need is one session = one DRB

## SCALE OUT / MULTIPLE TNLA
- Allow AMF to specify 2nd endpoint - ask worker 1
- Allow AMF to specify 2nd endpoint - ask worker 2
- Load balance, stickiness and switchover between TNLAs to AMF
- Allow DU / UP to set up multiple connections to same worker
- Restart and catchup of coordinator
- Failure and retry to set up / join NG / F1 / E1
- Switchover of UE on dead worker - RAN initiated
- Switchover of UE on dead worker - AMF initiated
- Connection API operations should be idempotent
- Stickiness and switchover between TNLAs to DU / UP
- AMF not started at point workers start
- Both workers die - reset
- All NGAP TNLAs drop - don't reset, ues-retained = true. 
- All F1AP TNLAs drop - reset. 
- All E1AP TNLAs drop - ?

## CONNECTION MANAGEMENT
- Regression test for tearing down requests when a connection dies
- Retry connection to AMF if connection refused.  (e.g. just run GNB-CU-CP on its own)
- Fix hang on Ctrl-C when AMF connect doesn't complete
- two worker enablement of DU interactions (share DU configuration between workers - see [documentation/design/State - DU.md])

## FUNCTION
- Proper graceful shutdown (waiting for / sending responses to pending requests)
- Make values in NG Setup configurable rather than hard coded (Tac, Plmn Id, slices, etc)
- Generate RRC transaction IDs properly
- Don't hang indefinitely waiting for response (e.g. NG Setup response)
- Don't allow unlimited pending requests
- Handle -ve response to InitialContextSetupRequest with bad RAN UE ID

## MAINTAINABILITY + DIAGNOSTICS
- standardize handler->workflow result handling and logging (who logs message, who forms failure) across CU-CP and CU-UP
- Rather than saying "WARN Unsupported UlDcchMessage C1(RrcReconfigurationComplete" we should report that the messsage does not match a transaction.
- Remove slog from workflow module and use log methods on Workflow instead
- Errors are too easy to miss - log_ue_error()? to optionally warn! on failure
  - e.g. "Inital access procedure failed - Connection refused (os error 111)" at debug
- Ue logging level should be settable to allow warnings to show up.  UE context should appear in logs / be stored in Logger.
- Cleaner RRC interface in trait Gnbcu
- Enforce Rust docs (see .cargo/config commented out compiler option)

## TESTS
- Failure to retrieve UE for each kind of request
- Efficient monolithic GNB-DU + GNB-CU can be built without a F1AP Stack or TransportProvider
- Efficient monolithic GNB-CU-CP + GNB-CU-UP can be built without an E1AP Stack or TransportProvider

## ASN.1 GENERATOR
- fixed size octet string should be [] not Vec?
- inlining of lists to avoid newtypes of vecs (e.g. PDU-Session-Resource-Activity-List)
- ENUMERATED{True} OPTIONAL (as seen in RRC) should appear in Rust as a bool
- Cope with extension marker being set
- Get rid of todo!() in top_pdu.rs and replace with a log
- Implement Rrc setuprelease
- Move to latest asn1-codecs crate version
- Deduplicate inline definitions in RRC autogeneration
- Generate procedures for Rrc and make F1AP a RequestProvider.
- Move to latest version of specs
- Fix clippy
## REDIS
- Don't create 1 Redis connection per access
- Live redis test returns ok even after "# Failed listening on port 23491 (TCP), aborting."
- Redis live test should not create Redis dump.rdb
## SCTP
- sock_opt.rs doesn't need to be a separate file
- Remodel SCTP API to look like the one in the webrtc-sctp crate?
## FREE5GC DEMO
- free5GC demo can register 2 (N?) UEs
- GNB-CU-UP executable can be started in demo and performs E1 Setup with GNB-CU-CP

# MEDIUM TERM
- Selection and stickiness of SCTP streams
- Dockerfiles and Helm charts
- Distributed timers and failure path cleanup mechanism

# DONE
- PDU session deletion
- Retry connection to AMF
- Single process GNB-CU with configurable MCC / MNC
- SDAP / PDCP should be added/stripped
- local-ip arg for mock-5gc and amf-ip arg for gnb-cu-cp
- mock_amf becomes mock_5gc
- dashmap
- E1 Setup
- CU-UP passes through userplane packets
- ASN.1 generator makes use of NonEmpty
- state.md flow "Eventually the AMF furnishes the GNB" wrongly shows DU context being created
- Refactoring of pdu_session_resource_setup.rs
- Add PduSessionId to xxap common
- Improved case conversion of DLUPTNLInformation-ToBeSetup-List and similar
- TransportLayerAddress has TryFrom<&str>
- Form PDUSessionResourceSetupResponse correctly
- Avoid missing NAS message Wireshark error on RrcReconfiguration
- Find a way to get O-DU PHY stub to send ReconfigurationComplete in the right order (recompile with `ratio = 20`)
- Common XXAP structures in Asn.1 generator (e.g. Snssai, GtpTunnel) to allow easy transfer between NGAP, F1AP, E1AP
- Use proper 32 bit bitstrings for TransportAddress in E1AP messages (avoids Wireshark decode issue) 
- Support for BearerContextModificationRequest in CU-UP
- Pass CellGroupConfig from DU to UE in Rrc Reconfiguration Request
- Use proper NAS messages from AMF-SIM to improve Wireshark trace of ODU interop
- Form CUtoDURRCInformation correctly in UeContextSetupRequest
- Supply DRBs-to-be-Setup-List on UeContextSetupRequest
- ASN.1 generator copes with choice IE extensions
- Store cell (NR-CGI) on UE context and pass on UeContextSetupRequest
- Tear down requests when a connection dies (e.g. if gnb-cu-up aborts while handling a request)
- Add GNB-CU-UP
- DL-CCCH-Message should not be PDCP encapsulated
- 'Worker startup failure' (e.g. when specifying wrong --local-ip) should be fatal
- RRC is UPER not APER
- Call TnlaEventHandler serially for a given association allowing message ordering control by upper layers (...meaning that intermittent reordering in scripted tests can be avoided)
- Tolerate missing mandatory IE TransactionId on InitialULRrcMessageTransfer for ORAN ODU interop
- Respond to GNBDUConfigurationUpdate
- Include CellsToBeActivated on F1SetupResponse.
- Fix intermittent failures in mock connection checking 
- F1 Setup with O-RAN ODU
- ASN.1 decoder should cope protocol extensions being present.
- Allow local-ip to be passed in on command line
- Triangular redirection 
- Get rid of revision number
- Go public
- Fix TNLA established event so that it is always processed before first packet 
- Add readmes + review as they appear on Github
- Rename gnbcu to gnb-cu-cp
- Simplify and add revision number to connection API
- UP / DU connections in either order
- Coordinator ensures time gap between attempts to add workers 
- 2nd worker receives UP / DU connection and adds 1st worker
- Test one UE through each worker
- Assoc should be in pool by the point that the connect call returns
- 1st worker initializes NG interface and 2nd worker joins in
- 1st worker receives UP connection and adds 2nd worker
- 1st worker receives DU connection and adds 2nd worker
- Timing bug causing cu_can_connect_to_amf() to sometimes hang when logging disabled.  No repro
- Log interleaving when RUST_LOG=debug and multiple tests run in parallel - noop - just do RUST_TEST_THREADS=1
- Two workers up
- Use IP address instead of ports to distinguish NGAP, E1 and F1 endpoints
- Standalone single worker that runs built-in coordinator
- Parallel registration of two UEs
- Get build working on Github
- Parallel tests
- Don't run live redis test by default
- PDU session resource setup - code review and tidy
- Break procedures into small functions
- NG Setup is in workflows but F1 and E1 Setup aren't
- Ints should be Copy rather than Clone (e.g. AmfUeNgapId)
- Registration sequence should not set up UE context in DU or invovle Rrc Reconfiguration
- SRB set to 0 or 1 and checking of that in Mock DU
- Add comments at top of files
- GNB-CU-CP serves E1 and supports E1 Setup
- Clean up Redis even if live redis test fails
- E1AP generation
- Update Amf Mock to support multiple Ues
- Registration of multiple UEs in FV
- Tidy code e.g. in procedures, GnbcuOps
- GNBDU-SIM should produce a clear error message if mongo is not started
- In demo, GNBDU SIM should not warn 'connection failed - will retry' on shutdown
- Get live test and demo working
- Store AMF UE Ngap Id on first downlink NAS transport
- Uplink and downlink NAS transfer should be in the procedures module
- Fix int coding bug
- Redis live test
- Single async fn / task that sends the Rrc Setup and waits for Rrc Setup Complete
- Fix failing test
- Code review (incl commented out code and TODOs) - gnbcu
- Redo roadmap
- In demo, gnbdu-sim has inconsistent message logging
- At end of demo, gnbcu panicked on Ctrl-C with 'not yet implemented'.  AMF was Ctrl-C'd first.  AMF logs indicate it is sending AMF Status Indication.  Crash is in ngap_gnb.rs.  Add an FV for this.
- Remove the weird TNLA 3, TNLA 53 IDs
- Three lines for each log sucks
- In demo, gnbcu logs strangely end with ">> UlRrcMessageTransfer".  Where's the uplink message?
- In demo, gnbcu logs saying UlRrcMessageTransfer should include (nas) 
- In demo, inconsistent debug log "Got Downlink Nas Transport - send RRC to UE via DU"
- In demo, downgrade log Accepted connection from Some(127.0.0.1:60313), since we already have an INFO log for that
- In demo, consider merging 'Accepted SCTP connection from 127.0.0.1:60313' and 'NGAP TNLA 3 established' into single INFO log
- In demo, inconsistent debug log "InitialUeMessage >"
- Write up reliable demo instructions starting with download of free5GC
- Add quickstart instructions and notes on current status to top readme
- Bump deps
- Move to consistent message logging
- Regression test integration covering same ground as live test with free5GC
- Pretty Ue initial context and move into separate modules
- Fix malformed CellGroupConfig issue seen in Wireshark in F1AP UEContextSetupResponse -- DUtoCURRCInformation, cellGroupConfig shown as <MISSING>
- Build go code using cargo build.
- Fix code that does match match
- Registration (23.502, figure 4.2.2.2-1) wrapped in UE Initial Access procedure (38.401, figure 8.1-1) working against free5GC as seen in ngap_setup.pcap.
- Go program that processes NAS information
- Encapsulate RRC in PDCP PDU
- gnbdu-sim
- RRC autogeneration
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
- explore HA + use of SCTP.