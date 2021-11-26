# Non-UE associated signaling

From TS 38.413:
> Between one AMF and NG-RAN node pair:
> -	A single pair of stream identifiers shall be reserved over at least one SCTP association for the sole use of NGAP elementary  procedures that utilize non UE-associated signalling.

A comment on Github suggests that it is normal practice to use stream ID 0 for non-UE associated signaling.
https://github.com/free5gc/free5gc/issues/88#issuecomment-767446612

So we implement that.

# UE associated signaling

From TS 38.413:
> For a single UE-associated signalling, the NG-RAN node shall use one SCTP association and one SCTP stream, and the SCTP association/stream should not be changed during the communication of the UE-associated signalling until after current SCTP association is failed, or TNL binding update is performed as described in TS 23.502.

i.e. everyone uses the association and stream chosen by the GNB... unless it decides not to.

TS 38.472 has almost identical text for F1AP, and additionally clarifies in TS38.473:
> The F1AP UE TNLA binding is a binding between a F1AP UE association and a specific TNL association for a given UE. After the F1AP UE TNLA binding is created, the gNB-CU can update the UE TNLA binding by sending the F1AP message for the UE to the gNB-DU via a different TNLA. The gNB-DU shall update the F1AP UE TNLA binding with the new TNLA. 

> The gNB-DU Configuration Update procedure also allows the gNB-DU to inform the gNB-CU that the indicated TNLA(s) will be removed by the gNB-DU.
...perhaps to be used as a quiesce? 

F1AP specifies serialization of procedures for a UE.  From TS 38.473:
> Unless explicitly indicated in the procedure specification, at any instance in time one protocol endpoint shall have a maximum of one ongoing F1AP procedure related to a certain UE.
But the equivalent section in the NGAP document doesn't!

# Multiple associations
There are a maximum of 32.  See maxnoofTNLAssociations in NGAP ASN.1 - TS38.413.

## Between GNB-CU and GNB-DU
As described in TS38.401, Multiple TNLAs for F1-C:
- GNB-CU must allow multiple associations from GNB-DU.
- GNB-CU may order GNB-CU to set up multiple associations to it using gNB-CU Configuration Update. 
So when we run multiple GNB-CU workers, we send gNB-CU Configuration Update with one address for each one.
Implying that when a new GNB-CU worker starts up, an existing worker must find out about that and send an update.

## Between AMF and GNB-CU
Conversely, the AMF may order us to open multiple associations to it.  Here we have the reverse problem.  A single GNB-CU worker gets an AMF Configuration Update, but each worker should then open a connection to the AMF.

When a worker gets the AMF Configuration Update, it sets up N TNLAs and responds.  

Then maybe the other workers catch up by adding their own connections and sending RAN CONFIGURATION UPDATE.  But maybe AMFs won't cope with this.  

# Overload
The AMF may order the gNB to reduce the signalling load.  This needs to propagate to all workers. 

# Chosen design - Node controller
Considering the following designs to propagate information around the cluster
1.  Gossip protocol.  
2.  Kubernetes controller ConfigMaps.
3.  Additional node controller microservice.
...I went for the last of these.

The idea is that 
-  the node controller does all non-UE associated signaling and knows about all the TNLAs
-  the workers find the node controller using a Kubernetes service
-  the node controller needs some synchronization mechanism
   -  either a consistent store like etcd
   -  or needs to be a singleton pod
   -  or needs to be an active-standby.
  
Key example of why a synchronization mechanism is needed is that otherwise two node controller instances might simultaneously try to send NG Setup.






