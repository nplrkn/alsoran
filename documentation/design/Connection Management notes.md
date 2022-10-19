# Connection Management

## Primer on AMF discovery

The starting point is the AMF Set.  Either we get it from the UE (GUAMI, S-TMSI) or from configuration/policy, or from a previous message from the AMF indicating what to do in the event of failure.  (ref 23.501, 6.3.5).

We then use NAPTR to find AMFs in the AMF Set as described in TS 29.303, 7.2.  To quote:

  The S-NAPTR procedure outputs a list of host names (AMFs) each with a service, protocol, port and a list of IPv4 and IPv6 addresses.

The GNB should connect to all AMFs in the AMF Set.  That means finding an address that works, setting up the first TNLA, doing an NG Setup over it, and then letting the AMF instruct it to set up more TNLAs if it so desires.  

(Does Kubernetes support NAPTR?  We could set up the same structure in local configuration as a stop gap.)

## Non-UE associated signaling

From TS 38.413:

> Between one AMF and NG-RAN node pair ... a single pair of stream identifiers shall be reserved over at least one SCTP association for the sole use of NGAP elementary  procedures that utilize non UE-associated signalling.

A comment on Github suggests that it is normal practice to use stream ID 0 for non-UE associated signaling: [https://github.com/free5gc/free5gc/issues/88#issuecomment-767446612]

## UE associated signaling

From TS 38.413:
> For a single UE-associated signalling, the NG-RAN node shall use one SCTP association and one SCTP stream, and the SCTP association/stream should not be changed during the communication of the UE-associated signalling until after current SCTP association is failed, or TNL binding update is performed as described in TS 23.502.

i.e. everyone uses the association / stream chosen in the first place by the GNB... until they don't.

TS 38.472 has almost identical text for F1AP, and additionally clarifies in TS38.473:
> The F1AP UE TNLA binding is a binding between a F1AP UE association and a specific TNL association for a given UE. After the F1AP UE TNLA binding is created, the gNB-CU can update the UE TNLA binding by sending the F1AP message for the UE to the gNB-DU via a different TNLA. The gNB-DU shall update the F1AP UE TNLA binding with the new TNLA.
The gNB-DU Configuration Update procedure also allows the gNB-DU to inform the gNB-CU that the indicated TNLA(s) will be removed by the gNB-DU.

F1AP specifies serialization of procedures for a UE.  From TS 38.473:
> Unless explicitly indicated in the procedure specification, at any instance in time one protocol endpoint shall have a maximum of one ongoing F1AP procedure related to a certain UE.

## Multiple associations
### Between GNB-CU and GNB-DU

As described in TS38.401, Multiple TNLAs for F1-C:

- GNB-CU must allow multiple associations from GNB-DU.
- GNB-CU may order GNB-CU to set up multiple associations to it using gNB-CU Configuration Update.

So when we run multiple GNB-CU workers, we send gNB-CU Configuration Update with one address for each one.
Implying that when a new GNB-CU worker starts up, an existing worker must find out about that and send an update.

### Between AMF and GNB-CU

Conversely, the AMF may order us to open multiple associations to it.  Here we have the reverse problem.  A single GNB-CU worker gets an AMF Configuration Update, but each worker should then open a connection to the AMF.  When a worker gets the AMF Configuration Update, it sets up N TNLAs and responds.  

The NG-RAN node is also allow to add endpoints.  So the other workers catch up by adding their own connections and sending RAN CONFIGURATION UPDATE.

> When the configuration with multiple SCTP endpoints per NG-RAN node is supported and the NG-RAN node wants to add additional SCTP endpoints, the RAN configuration update procedure shall be the first NGAP procedure triggered on an additional TNLA of an already setup NG-C interface instance after the TNL association has become operational, and the AMF shall associate the TNLA to the NG-C interface instance using the included Global RAN node ID.

There are a maximum of 32.  See maxnoofTNLAssociations in NGAP ASN.1 - TS38.413.

## AMF support for multiple associations

free5GC AMF identifies GNB / creates RAN context by connection.  Each of our workers would manifest as a separate RAN.  So it is never going to do triangular redirection.  And there is no point having a node controller because there is no coordination required.

Open5GS AMF identifies the gNB by address - see amf_gnb_find_by_addr().  Presumably this means it can't do triangular redirection either.  Since both NG Setup and ngap_handle_ran_configuration_update() call amf_gnb_set_gnb_id() the GNB IDs will overwrite each other for the purposes of handover.

If neither of the open source AMFs can cope with multiple parallel connections from same gNB, possibly some of the commercial ones don't either.  It depends on their support for TS 38.412 multi associations.

## Overload

The AMF may order the gNB to reduce the signalling load.  This needs to propagate to all workers.

## Coordinator design

To allow workers to coordinate, we could provide a coordinator microservice.  The idea is that

- the coordinator controls or does non-UE associated signaling and knows about all the TNLAs
- the workers find the coordinator using a Kubernetes service
- the coordinator synchronizes exchanges across multiple workers
  - either a consistent store like etcd
  - or needs to be a singleton pod (potentially per instance of the API) with single-threaded business logic
  - or needs to be an active-standby.
  
Key example of why a synchronization mechanism is needed is that otherwise two node controller instances might simultaneously try to send NG Setup.

## Coordinator state

The coordinator needs to know
-  what workers, associations, peers and AMF endpoints exist
-  what the initialization state of each association is.

In a stateless model, it would have to learn all of this from worker nodes.

### Procedure trigger callback

Consider a design where the worker on startup was informed whether the AMF has been contacted and the NG interface instance is up yet.  The worker can now autonomously decide whether to send NG Setup or RAN Configuration update.

The problem with this idea is that, when two workers come up simultaneously, they will both learn that the interface instance is down and thus both send NG Setup.  More elaborate synchronization is needed.

Worker instances provide a procedure trigger callback.  

When the coordinator learns of a new connection to the AMF, it uses the procedure trigger callback to trigger an NG Setup or RAN configuration update.  Either a single task must be used, or a consistent store.

### Sequencing of connection establishment 

Can new workers be set up in such a way as to minimize the situation where UE associated messages can't be passed through?

Since the UE initiates the connection, this suggests setting up the NGAP interface before allowing a new DU to finish initializing its connection.

However, if this same policy is applied when adding the second worker, there is the danger that it will receive a triangular redirected response from the AMF and be unable to pass it back to a DU. 


### UE state retention

When we tell AMF that UE state has been retained, this is on behalf of the entire GNB, not just the CU.

According to the description of F1 Setup, this procedure always clears out state.  
> This procedure also re-initialises the F1AP UE-related contexts (if any) and erases all related signalling connections in the two nodes like a Reset procedure would do. 

So, when all CU workers die, we necessarily lose all F1 TNLAs, hence our F1 interface instance, hence all F1 state.

The above shows that in the Alsoran design, ues-retained should only be set to true on NG Setup if all NGAP TNLAs are lost but workers, state and DU connections remain.


### Coordinator startup and restart
```mermaid
sequenceDiagram
  participant C
  participant W1
  participant W2
  participant W3
  participant AMF
  W1->>C: Refresh (conneciton api server, F1, E1, no TNLAs)
  C->>W1: Refresh 204
  C->>W1: NG Setup (ran node ID)
  W1->>AMF: NG Setup (ues-retained = false)
  AMF->>W1: NG Setup response
  note over W1: updates local coordination state - TNLA up to EP 1
  W1->>C: NG Setup response
  note over W1: stores Ue state in Redis
  note over C: restarts 
  W1->>C: Refresh (callback server, F1, E1, TNLA up to EP 1)
  C->>W1: Refresh 204
  note over W1: restarts as W2 - UE state still in Redis
  W2->>C: Refresh (callback server, F1, E1, no TNLAs)
  C->>W2: Refresh 204
  W3->>C: Refresh (callback server, F1, E1, no TNLAs)
  note over C: Still busy initializing W2 - will come back to W3 later
  C->>W3: Refresh 204
  C->>W1: Add F1 endpoint
  note over C: timeout, W1 dead, all TNLAs to AMF dead
  C->>W2: NG Setup (ran node id)
  note over W2: Redis state exists related to this node ID
  W2->>AMF: NG Setup (ues-retained = true)
  AMF->>W2: NG Setup (ues-retained = false)
  note over W2: AMF doesn't support retention - clear Redis state, F1 reset, E1 reset
  W2->>C: NG Setup
```

### Multiple TNLA endpoints from AMF - orig design

```mermaid
sequenceDiagram
  participant C
  participant W1
  participant W2
  participant AMF
  participant AMFb
  participant AMFc
  W1->>C: F1AP port, no AMFs connected
  W2->>C: F1AP port, no AMFs connected
  C->>W1: AMF address, you may setup 
  C->>W2: refresh in 5s
  W1->>AMF: connect
  W1->>AMF: NG Setup
  AMF->>W1: Ack
  W1->>C: F1AP port, AMF connected
  C->>W1: refresh in 30s
  AMF->>W1: AMF Configuration Update (2 new ports)
  W1->>AMFb: connect
  W1->>AMFc: connect
  AMF->>W1: AMF Configuration Update ok
  W1->>C: F1AP port, AMF a, b, c connected
  C->>W1: refresh in 30s
  W2->>C: F1AP port, no AMFs connected
  C->>W2: AMF addresses a, b, c, add yourself
  W2->>AMF: connect
  W2->>AMF: RAN configuration update
  AMF->>W2: ack
  W2->>AMFb: connect
  W2->>AMFb: RAN configuration update
  AMFb->>W2: ack
  W2->>AMFc: connect
  W2->>AMFc: RAN configuration update
  AMFc->>W2: ack
  W2->>C: F1AP port, AMF a, b, c connected
  C->>W1: ok, refresh in 30s

```

### Where we need to connect to multiple AMFs

This is from 29.303.

The AMFs available within an AMF Set should be provisioned within NAPTR records in the DNS, under the AMF Set FQDN (as defined in clause 28.3.2.7 of 3GPP TS 23.003 [4]), with the Service Parameters "x-3gpp-amf:x-n2".
The 5G-AN may discover the AMFs available within an AMF Set by:
-	constructing the AMF Set FQDN, as defined in clause 28.3.2.7 of 3GPP TS 23.003 [4], identifying the AMF Set of the AMFs to be discovered; and
-	initiating an S-NAPTR procedure, with the Application-Unique String set to that AMF Set FQDN, and with the "Service Parameters" set to "x-3gpp-amf:x-n2".

When connecting to an AMF it may provide a backup AMF name per GUAMI in its served GUAMI list.

