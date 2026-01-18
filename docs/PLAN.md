When provisioning, the intent has changed or the network has drifted from the
current service model definition. The planning stage needs to reevaluate things.

Firstly, since intents are scoped by service, the service model need to be redefined.

This is a interesting from a function point of view. The service model is redefined,
and 'merges' itself with the old service model. Literally, like a reverse osmosis.
The requirement for a backend load balancer, merges with the load balancer which is
already deployed.

(Maybe boringly, this is just. Delete one of each current type... idk.)

This way only the diff of current state and new state needs decision logic.

Maybe we should call this something different. The service model is the model.
Maybe this is the service actualization or something. service graph... idk.

Basically, whether it's generated separately or grown from a small model, a
service graph is made.

Generating might make things a little simpler.
