GTP TEID ID is a 32 bit value:

An Alsoran UP UE E1AP ID is a up to 23 bit value.  The least significant N bits indicate the worker ID that allocated the ID.

The GTP TEID ID is encoded to include the UE E1AP ID as follows:

least significant bit: 0 for uplink and 1 for downlink.
bits 1-23: UE E1AP ID
most significant byte : session/DRB index

The userplane data structure (ForwardingContext) is indexed by bits 0-24 of the GTP Teid ignore the session/DRB index.  
i.e. producing the entire downlink, or uplink information for a given UE.  This structure contains session info as substructures. 

A pool of UE E1AP IDs is allocated at start of day.

The combination of the above means that the CU-UP state can be stored in a simple array and the packet processing path does not need to hash TEIDs.