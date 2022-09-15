# Registration call flow

5G registration sets up a GNB-CU UE context without a PDU session. 
Since no DRB is needed yet, no context in the DU or CU-UP needs to be created.
The GNB-CU establishes Rrc security on the SRB and then completes the operation.
See O-RAN.WG5.C.1, 6.1.1.2.

```mermaid
sequenceDiagram
  participant DU
  participant CU
  participant AMF
  AMF->>CU: Initial Context Setup Request
  Note over CU: Retrieve
  CU->>DU: Dl Rrc Message Transfer -- Rrc Security Mode Command
  DU->>CU: Ul Rrc Message Transfer -- Rrc Security Mode Complete
  Note over CU: Store
  CU->>AMF: Initial Context Setup Response
```

