# Roadmap

## Mission

Our long term mission is to reduce the cost of 5G RAN by providing high quality free componentry.

## Who Alsoran is for

-  5G Core developers who need a reference RAN to test against.
-  RAN developers who would benefit from ready made Rust components.
-  Developers of 5G testbeds.

## Releases

### Release 0.1 - NGAP and F1AP basics

- GNB-CU-CP prototype.  No DU, no userplane.
- Successful PDU session setup against free5GC and Open5GS driven from DU simulator.
- Quickstart and early design documentation.
- License text, copyright notices, contributing guide.

### Release 0.2 - userplane prototype

- Userplane prototype - DU simulator sets up a TUN interface per UE. 
- Linting scripts + release process.

### Release 0.3 - Kubernetes

- Dockerfiles and Helm charts
- Open source notices

### Release 0.4 - GNB-CU-CP scale out

- Session context stored separately.
- Interchangeable workers, each with TNLAs to AMF and DU.

### Release 0.5 - coordinator scale out and state replication

### Release 0.6 - user plane fault tolerance and replication

- Session context replicated and fault tolerant.
- Switchover of active sessions on failure of UP container.

### Release 0.7 - hardening

- Spec compliance document
- Connection management resilient to variations in startup ordering.
- Triangular redirection
- Avoid using unresponsive TNLAs.
- Clear error logging of unsupported function.

### Release 0.8 - performance

- Prometheus integration
- Control plane performance benchmark

### TBD

- OpenTracing
- Userspace SCTP
- AMF Set - connect to multiple AMFs, note backups, observe AMF selection rules
