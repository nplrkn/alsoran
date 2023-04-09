# DU state

Alsoran CU-CP stores the DU configuration received on the F1 Setup.  For example, this includes the information about the cells
that the DU is configured to serve.

Since F1 Setup is sent to a single worker, there needs to be a way to distribute this information to the other workers.  The options are either to store it in Redis, or to have the coordinator distribute it via the coordination and connection APIs.  The former is simpler.

Given that the DU configuration changes rarely, it would be a shame from a performance point of view to get it from the database for every request, so instead we read it once and then store it.  Later, we will work out a method for updating it. 