# Notes on mobility

A UE is served by a cell of a DU.  The cell is known either by its globally unique ID, the NR-CGI, or by a cell index
which is scoped to the DU.

The GNB-CU learns all the cells served by a DU via the F1 Setup (and can be updated by DU configuration update).

When a UE establishes an RRC connection, its cell is first learned by the GNB-CU in the NR-CGI field of the initial 
RRC message transfer.

From then on, the GNB-CU receives measurement reports that allow it to determine if a cell handover should take place.

Cells handovers can be intra-DU, inter-DU, or inter-GNB, or inter-AMF.  The intra-gNB-CU mobility cases are covered in
TS 38.401, section 8.2.1.  For example, in the most simple case (8.2.1.2), the Gnb-CU simply sends a UE Context Modification. 