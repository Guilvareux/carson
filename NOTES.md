# Notes

This project considers the goal of 'intent' in intent-based networking to be concerned with the
service model. With a sufficiently expressive service model, all future decision-making about
the lifecycle of a network service should largely be automatic. The intent handler concept that
will be demonstrated by this project will create service models in three stages. Firstly,
the service model is defined with invariants, features which must always be true. The intent
model provides additional requirements based on the customers business needs. With both,
our intent reasoner should be able to manage the lifecycle of network services most of the
time without human intervention.


## Lifecycle stages

Service models themselves have day 0, day 1 and day 2 requirements.
- Day 0 requirements are fundamental features of the hardware they run on, these are encoded in the model
- Day 1 requirements are requirements that the intent may provide. I.e. a certain amount of redundancy
or certain security guarantees these can be made during provisioning without knowledge of the network
state.
- Day 2 requirements are based on network conditions and demand. These cannot be predicted without
an understanding of the current network state, and possibly without actually deploying the service.
It's not trivial with software to know fundamentally how much capacity a given service may be able
to serve for example.

### Day 0: What do we know immediately

#### From the service model
- We know what the picture of the service looks like: what connects to what, what the role
of each function is (load balancer/backend/firewall etc.)
- We can have some idea of what needs to scale (replicate backends until load balancer is saturated, replicate
frontend until database is saturated and then add another whole service etc.)
- What the hardware requirements of the host are. For example HEVC video encoding
-

#### From the intent
- Security guarantees (no data travelling through Russia?)
- Redundancy (Add a ratio of 1:3 idle services to active services in case of outage)
- Connectivity (Ensure two backhaul networks minimum incase of cable failure)
- Others...

> TL;DR What do we know without considering scaling yet?

### Day 1: What will the network tell us
- What needs to scale up
- Do we need any extra deployments?
- Do we need to scale databases
- Do we need more CDNs?

> TL;DR Do we need more of what we've already defined.

### Day 2: What can we learn from patterns
- Anomaly Detection

> TL;DR How can we avoid over-provisioning

## Intent

Intent is split into two camps

High-level attributes (Guides Day 1 + 2)
- Provide customers with 4K Video!
- Provide customers with 100ms response time!

Low-level attributes (Guides Day 0)
- Redundancy (Especially important for video)
- Security
- Deployment Locations
- Service Model refinements


## Stages of execution

> The intent handler lifecycle

- Deployment
- Monitor
- Analyse (Intent validation)
- Plan
- Execute

### Deployment

#### Definition

> Service model exposes requirements through the knowledge graph.

- Hosts are nodes with features i.e. cpu requirement, free mem requirement
- Hardware requirements for the host
- Placement i.e. `w:Q4`

#### Intent Actualisation

>Policies are added or relaxed based on Low-level intent the goal of this stage is
>to create runtime rules that offload all decisions to runtime requirements.

- Geo restrictions (Don't pass data through Russia)
- Monitoring (we care that services are measured in UK but not France)

This manifests as more attachments to the 'Host' node, which essentially must match. Examples:
- 'not in country w:Q4'
- 'setup monitoring node at a host, at the edge which is in country x'

#### Context

> Policies manifest into actual runtime decisions

- How much CPU does the load balancer need before the whole service scales
- How much do backends scale before a new loadbalancer+backends service is needed

#### Advanced Context

> What can running this service for some time tell us

- When should we scale up services
- When can we scale down services
- How can we achieve 99.999% the cheapest way!

### Monitor

All rust, simply query the network at intervals to provide the analysis step

### Analyse

The analysis loop uses validation intents to process what's happening.
We might also add some monitoring of resource...

Links:
Max bandwith,
total Throughput etc.

Hosts:
CPU usage,
mem etc.

VMs/Containers:
CPU quota usage etc.
### Plan
So what the functions should look like?
- Search for Host
- Mitosis of a specific component
- Absorption of a specific component (in the background, pick one to kill... and possibly migrate to better spot).

- Additional Wacky Ideas
    - Spacial Functions... Instead of trying to calculate everything with traditional methods. Use the space in graphs to our benefit!
        - Ignore hops and bandwith costs, make a target on the graph and first look at vnfs in the bullseye.
        - When making decisions, manipulate the graph to make space help with decision making. i.e. repel all machines from the decision area with low spare resource! idk...
        - Calculate the average area of a service. Since all services are different this could probably just be used to track service "defragmentation"...
    - We also need graph gravity... This is actually two ideas
        - We want to place things based on gravity
        - We want to slowly migrate individual vnfs based on gravity (not via migration, but by natural selection).

There should be a 'requirement' ontology, which mirrors the network ontology.
Except it specifically advertises a need ONLY. Then, in the stage AFTER
planning, the need should be resolved.

Intent create:
Start with service model.
Intent are the abstract things we want out of the service model.
THEN the predicates for solving that problem.
