# Backlog

Other ideas while we wait in the hope that https://github.com/gabhijit/hampi gets an encode function.


- [in progress] socket OS errors getting hidden by anyhow 
- regression testing of maintain and retry
- documentation of features and level of testing
- test the sctp receive function
- get info / trace levels right
- See if the aper codec used in work is open source.
- Implement the node controller / investigate OpenAPI
- Be test driven and get CI going


------

- [done] Maintain + Retry - i.e. test plan is
1. start up Alsoran first then Free5GC [pass]
2. start up free5G first then Alsoran [pass]
3. kill and restart Free5GC [fail - doesn't notice]
-  [done] Successful NG Setup with Free5G.
-  [done] get rid of haphazard error conversion - see TODOs
-  [done] clean up sctp_association.rs
-  [done] Clippy
-  [done] send a fake ng setup and test the sctp receive function
   -  by running wireshark and getting byte dump
      -  wireshark Copy as hex stream, then hex::decode()
         00150035000004001b00080002f83910000102005240090300667265653567630066001000000000010002f839000010080102030015400140
- [done] Properly close sockets when wrapper structs are dropped
-  [done] Stay up until Ctrl-C
-  [done] SCTP connect

-  it will be hard to test probably without simulated UE / NAS - and this will have to be wrapped in DU
   -  ok so NAS.py for the NAS?
-  non UE associated and UE associated signaling
-  add error handling / back pressure handling

-  [done] test against a 5G core
   -  install free5GC - based on https://www.free5gc.org/installations/stage-3-free5gc-install/
   -  seems highly unlikely to work on WSL but would be awesome if it did - at least registration and the first bits of PDU session
   -  THIS LOOKS GOOD ENOUGH FOR NOW!
-  [done] reinstate a callback model that lets the transport provider be aware of ue or non ue associated signaling
   -  idea is to type parameterize receive method not the whole trait
-  [done] add logging
-  [done] do two exchanges concurrently in UT


Build in chronological order with refactoring.

-  Step 1
   -  Start up and establish a connection with the AMF and perform the NG Setup exchange.
      *  Ability to connect / send / receive in async code with stream ID
         *  Got stream ID in sync code.
         *  Really the Rust crates are not great.
         *  Create the async API we want.  Start using Async<>.
         *  Check out impl Stream for https://docs.rs/async-net/1.6.1/async_net/struct.Incoming.html
            * Can we use this to model each association as a Stream of incoming (messages, stream id)?
      -  Mock AMF
-  Step 2
   -  Wait for and receive connections from DUs.



Most important area to explore is HA + use of SCTP multi-homing.

Node manager.    
  State
    -  AMF configuration
    -  DU configuration
    -  List of workers (via Kubernetes API)
  Supported procedures
    -  Send NG SETUP to AMF
    -  Receive AMF configuration -> set up new connections
    -  Receive F1 SETUP from DU
    -  Send RAN configuration to DU -> ask for new connections






Dev approach
-  implement an obvious procedure or two with fake messages
-  create SCTP containers to represent the AMF and CU
-  get procedure load balancing between in a sensible way

Two instances of a CU must share the same UE context.  

See 6.4 of TS38.401:
   An NG-RAN node UE context is a block of information in an NG-RAN node associated to one UE. The block of information contains the necessary information required to maintain the NG-RAN services towards the active UE. An NG-RAN node UE context is established when the transition to RRC CONNECTED for a UE is completed.

So once a UE is RRC CONNECTED, any CU instance must be able to cope with it.  That said, the AMF will steer all UE traffic over the same SCTP stream if it can.

TS 38.413:
  NG-RAN node and AMF shall support a configuration with a single SCTP association per NG-RAN node/AMF pair. 

  Within the set of SCTP associations established between one AMF and NG-RAN node pair, the AMF may request the NG-RAN node to restrict the usage of SCTP association for certain types of NG-C signalling. 

  Selection of the SCTP association by the NG-RAN node and the AMF is specified in TS 23.501 [3] and TS 23.502 [4]. The NG-RAN node shall establish the SCTP association.

  Between one AMF and NG-RAN node pair:
-	A single pair of stream identifiers shall be reserved over at least one SCTP association for the sole use of NGAP elementary procedures that utilize non UE-associated signalling.
-	At least one pair of stream identifiers over one or several SCTP associations shall be reserved for the sole use of NGAP elementary procedures that utilize UE-associated signallings. However, a few pairs (i.e. more than one) should be reserved.
-	For a single UE-associated signalling, the NG-RAN node shall use one SCTP association and one SCTP stream, and the SCTP association/stream should not be changed during the communication of the UE-associated signalling until after current SCTP association is failed, or TNL binding update is performed as described in TS 23.502 [3].


i.e. the last point says that the stream is sticky to UE, but gets reassigned after failure of the SCTP association.

23.502:
The AMF supplies the 5G-AN node with information about
a)	the AMF Name and the GUAMI(s) configured on that AMF Name;
b)	the set of TNL associations to be established between the NG-RAN node and the AMF;
c)	weight factor associated with each of the TNL association within the AMF; and
d)	weight factor for each AMF Name within the AMF Set; and
e)	(optional) for each GUAMI(s) configured on that AMF the corresponding backup AMF Name.

The 5G-AN node selects an AMF as defined in clause 6.3.5 of TS 23.501.

In NG Setup - 8.7 of TS 38.413:

If the UE Retention Information IE set to “ues-retained“ is included in the NG SETUP REQUEST message, the AMF may accept the proposal to retain the existing UE related contexts and signalling connections by including the UE Retention Information IE set to “ues-retained“ in the NG SETUP RESPONSE message.

RAN node identified by GlobalRANNodeID.

While a UE is in CM-Connected state the 5G-AN node shall maintain the same NGAP UE-TNLA-binding (i.e. use the same TNL association and same NGAP association for the UE) unless explicitly changed or released by the AMF.

An AMF shall be able to update the NGAP UE-TNLA-binding (i.e. change the TNL association for the UE) in CM-CONNECTED state at any time. The NGAP UE-TNLA-binding can also be updated when a UE-specific NGAP message initiated by AMF is received via a new TNL association.

An AMF shall be able to update the NGAP UE-TNLA-binding (i.e. change the TNL association for the UE) in response to an N2 message received from the 5G-AN by triangular redirection (e.g. by responding to the 5G-AN node using a different TNL association).

An AMF shall be able to command the 5G-AN node to release the NGAP UE-TNLA-binding for a UE in CM-CONNECTED state while maintaining N3 (user-plane connectivity) for the UE at any time.


