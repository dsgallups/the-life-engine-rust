2025-11-9
- [ ] Make the thing move

I can say
1) one network controls all the cells
2) each cell has a network that communicate across junctions


Let's go with the first approach.
One network controls all the cells.

And we can say that
each cell has a "network template."

i.e. it has several input nodes and several output nodes.

I'm confused.

So a cell, let's say a launcher. how does it interface/replicate?
It has a set number of inputs and outputs, definitely.
The input nodes can be mapped properly

but the output node...like, if the cell grows a new launcher,
do we say that output node is unconnected? maybe we make sure that output node is connected.

let's say the cell replicates. the output node must exist yeah?

what if the cell replicates and loses this node. Then those inputs and outputs are also removed.









Things for modeling

- Every organism has a brain
- Every organism can move
- Every organism can eat
- Every organism can see
- Food spawns at a certain rate
- Spores spawn at a certain rate
- Organism cells become food + spores when killed

## Cell types
- Launcher: Can launch spores that will kill
- Collagen: connector cells, cheap
- Brain: The center of the organism
- Information: arbitrary data storage

Cells die when their brain is hit


- We should enforce that all input nodes are part of the map,
- or at least that input nodes cannot be pruned.
