# Logging design thoughts

Instead of plumbing down the logger everywhere - e.g. to low level SCTP function - the alternative is to capture enough context in the Result / Err and then log higher up.

This is important given that those low level components do not know what log level to use because they don't know how important / recoverage their operation is.

However,
  
- a function like 'maintain connection' that runs as an independent task will need logging
- where we treat setsockopts as non fatal, we have to log since otherwise there will be no other way of noticing the failure.
