# Triangular redirection

## Spec references
From TS23.502, 

> When a UE connects to the 5GC via a 5G-AN node  ... the AMF may decide to use the TNL association selected by the 5G-AN or the AMF may modify the NGAP UE-TNLA-binding by triangular redirection.  
>
> (4.2.7.2.1)

and

> At any time the AMF may decide to re-bind the NGAP UE association to a new TNL association:
> -	by sending a UE-specific NGAP message on a new TNL association (triangular redirection), if:
>   -	AMF responds to the 5G-AN node initiated NGAP message (i.e. triangular redirection) as described in clauses 4.2.7.2.1, 4.2.7.2.2 and 4.2.7.2.3; or
>   -	AMF initiated UE-specific NGAP message needs to be sent to 5G-AN node;
>
> (4.2.7.2.4)

## Any worker can handle any request
Because Alsoran workers do not store UE state, it does not matter which TNLA or worker a request from the AMF comes in on - the
worker will always retrieve the UE state from the data store.

## Are responses an issue?  (Answer: no)
However, Alsoran workers _do_ assume that a response comes into the same worker as the one that sent a request.  Therefore the clause about doing a triangular redirection on a response needs careful inspection: "AMF responds to the 5G-AN node initiated NGAP message (i.e. triangular redirection) as described in clauses 4.2.7.2.1, ...".

What clause 4.2.7.2.1 describes is specifically the NGAP Initial UE Message, the final step of the Initial Access Procedure.  The AMF typically 'responds' by sending a Downlink NAS Transport.  See [state.md](state.md) for these flows.  

At the NGAP layer, Initial UE message and Downlink NAS transport are not a request/response, but two indications.  The concept of a response relates to the NAS layer.  Alsoran workers close their transaction task and commit the UE state at the point of sending Initial Ue Message, as with any other indication.  So, this case of triangular redirection is in fact no different from the other   described, and an Alsoran worker can happily handle this 'response' arriving on a different TNLA without any special logic. 

## Updating TNLA bindings
Currently, Alsoran is limited to a single TNLA per worker, and uses SCTP stream 0 for all UEs.  This means there is no point storing UE TNLA bindings.  When one of these limitations is lifted, it will have to store the TNLA binding as part of the UE state and follow the AMF's lead if the AMF wants to change them, by updating this state. 