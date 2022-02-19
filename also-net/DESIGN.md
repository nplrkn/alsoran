# Multitasking design

There is a task running for each of the following.
  
- Every SCTP listen.  This task accepts connections and spawns the assocation task.
- Every SCTP association.  This task receives packets and calls up to the procedure handler.  The procedure handler spawns a new transaction task for an incoming request.
- Every transaction.

## Shared state

These are Arc/Mutex collections, meaning that they can be cloned and shared between tasks, and any task can modify them.

- List of pending requests.  All outgoing requests over a given transport are held in a single list.
- List of connections (TNLAs).