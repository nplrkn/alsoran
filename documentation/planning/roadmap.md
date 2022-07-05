# Roadmap

## Mission

Reduce the cost of 5G RAN by providing high quality free componentry.

## Releases

### Release 0.1 - NGAP and F1AP basics

- GNB-CU-CP prototype.  No DU, no userplane.
- Successful registration against free5GC.
- Quickstart and early design documentation.
- License text, copyright notices (?), contributing guide.

### Release 0.2 - reference userplane

- Session setup (23.502, figure 4.3.2.2.1-1).
- Userplane prototype - DU simulator sets up a TUN interface per UE. 
- Linting scripts + release process.

### Release 0.3 - Kubernetes

- Dockerfiles and Helm charts
- Open source notices

### Release 0.4 - GNB-CU-CP scale out

- Interchangeable workers, each with TNLAs to AMF and DU.
- Session context in datastore (preferably encrypted).
- Distributed timers. 

### Release 0.5 - triangular redirection

- Cope with AMF responding to UE associated request on a different TNLA binding.

### Release 0.6 - connection termination handling

- Connection management resilient to variations in startup ordering.
- Avoid using unresponsive TNLAs.
- Quiesce
- SCTP backpressure

### Release 0.7 - performance

- Prometheus integration
- Control plane performance benchmark
- Overload work shedding
- Tuning
- Balance load across TNLAs.

### Release 0.8 - documentation

- Spec compliance document
- Quality status (features -> maturity + test coverage) 
- Dimensioning guidance
- Code coverage
- Troubleshooting + diagnostics

### TBD

- Privacy (redact personal information from diagnostics)
- Message encryption
- UP session high availability.
- OpenTracing
- Userspace SCTP
- AMF Set - connect to multiple AMFs + observe AMF selection rules
- MOCN
