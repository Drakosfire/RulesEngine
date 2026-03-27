# Game Engine

- [Game Engine](#game-engine)
  - [Definitions](#definitions)
  - [Further Definition Context](#further-definition-context)
  - [Spec](#spec)
  - [Solution](#solution)
    - [`GameEngine` CLI Reference](#gameengine-cli-reference)
    - [`GameEngine` CLI Usage Details](#gameengine-cli-usage-details)
      - [`init`](#init)
      - [`apply_transform` (most common use case)](#apply_transform-most-common-use-case)
      - [`create_transform`](#create_transform)
      - [`apply_next_transform`](#apply_next_transform)
      - [`apply_all_transforms`](#apply_all_transforms)
      - [`test_transform`](#test_transform)
    - [`GameEngine` CLI Implementation Details](#gameengine-cli-implementation-details)
      - [`$ GameEngine create_transform "example"` details](#-gameengine-create_transform-example-details)
    - [Data Shapes](#data-shapes)
      - [Ruleset Data Shape](#ruleset-data-shape)
      - [Campaign Data Shape](#campaign-data-shape)
      - [Game State Shape](#game-state-shape)
    - [DSLs](#dsls)
      - [Transformation DSL (english \& programmatic)](#transformation-dsl-english--programmatic)
      - [Ruleset Node DSL](#ruleset-node-dsl)
      - [Ruleset Edge DSL](#ruleset-edge-dsl)
      - [Campaign Nodes](#campaign-nodes)
      - [Campaign Edges](#campaign-edges)
      - [Ruleset Constraint DSL](#ruleset-constraint-dsl)
        - [Constraint Operators](#constraint-operators)
        - [Example Constraint Operator Composition](#example-constraint-operator-composition)
      - [Game State Patch DSL](#game-state-patch-dsl)


## Definitions
1. A "Graph" is a collection of concepts and relationships between them
2. A "Node" is a concept in a graph
3. An "Edge" is a relationship between two Nodes
4. An "Element" is a Node or Edge
5. A "Node Type" is a label for a category of Nodes.
6. An "Edge Type" is a label for a category of Edges. Examples include: synonym, hierarchy, containment, superset, condition, attribute, equality, inequality.
7. A "Source" is an Edge's origin Node
8. A "Sink" is an Edge's terminus Node
9. A "RPG" is a subset of all conceptual games called "Role Playing Games".
10. A "Rulebook" is a text (book, blog, napkin-back, etc.) that defines or modifies how to play a RPG.
11. A "Ruleset" is a directed acyclical Graph (DAG) projection of one or more rulebooks, containing references back to the originals.
12. A "Ruleset Node" is a single Rulebook concept (e.g. Creature, Combat, Rest).
13. A "Ruleset Edge" is a typed Edge between Ruleset Nodes (e.g. Creatures may wear Equipment, Equipment is a superset of Sword, Container can contain Node)
14. A "Ruleset Element" is a Rule Node or Rule Edge.
15. A "Rule" is a composition of one or more Ruleset Elements.
16. A "Constraint" is a Rule that includes Edge Types like subsets and inequalities. It exists for the purpose of validating other Elements. e.g. "A rides B" might only be valid when: "A is Creature && B is Creature && A size <= B size".
17. "A Time Schema" refers to a to a cluster of time-semantic Rules (e.g. speed, distance, forward, back, groupings, orderings, duration).
18. "A Space Schema" refers to a cluster of space-semantic Rules (e.g. proximity, groupings, distance, direction, orderings, length, width)
19. "A Spacetime Schema" refers to one pairing of a space schema and a time schema, and any relationships specific to it. (e.g. velocity)
20. An "Instance" is a concrete version of an abstract thing. e.g. Node:Creature -> Bob. e.g. Edge: A rides B -> Bob rides Bob's Horse.
21. "A Game" refers to a Ruleset paired with its Instances.
22. "A Game Element" references a single Ruleset Element or Instance in a Game.
23. "A Campaign" refers to all Game Elements
24. "A SpaceTime" is a SpaceTime Schema Instance, including all points in its time and space.
25. "A Time Point" refers to an infinitely small slice of a SpaceTime's time paired with all of its space
26. "A Space Point" refers to an infinitely small area of a SpaceTime's space paired with all of its time
27. "SpaceTime Time" is the collection of all Time Points in a SpaceTime
28. "SpaceTime Space" is the collection of all Space Points in a SpaceTime
29. "A Space Point" refers to an infinitely small slice of SpaceTime Space paired with all of SpaceTime Time
30. "A SpaceTime Point" is a Space Point paired with a Time Point
31. A "SpaceTime Clock" is a discrete numerical value that corresponds to a SpaceTime's Time Points. It enables SpaceTime Elements to change.
32. "The Game Clock" is a monotonically increasing discrete numerical value that provides a stable comparison for SpaceTime Clocks
33. A "Game State" is a snapshot of a Game at a single Game Clock value
34. A "Transformation" is a change to one or more Game Elements.

## Further Definition Context
1. Rulesets come from rulebooks and similiar sources
2. Many rulesets exist.
3. Concepts, meanings, and relationships often differ between rulesets.
4. Concepts, meanings, and relationships may be explicit or implicit.
5. Rulsets may include continuous concepts like time and space, and discrete concepts like integers.
6. Discrete concepts and relationships may be ordered (e.g. ordinal, sequential) or unordered (e.g. categorical).
7. Discrete concepts and relationships may be grouped in mutually exclusive and inclusive ways. (e.g. Space: A state is a distinct entity. "The midwest" is a spatially contiguous group of states. "Square states" is a non-contiguous group.) (e.g. Time: "span" is an ordered contiguous group. "the good years" is an unordered non-contiguous group.).
8. Rulesets may define other relationships, like representations (map represents country), synonyms (big is like large), subsets (human is a subset of race) and attributes (bob is race human, or bob is large)
9. A ruleset may have one or more Space Schemas, Time Schemas, or SpaceTime Schemas.
10. Instantiated concepts may be named differently from abstract concepts, but require some reference to the abstract concept in order to apply the rules correctly.

## Spec
1. We want to play a campaign where:
    1. A game master (GM) controls the Game and facilitates all Transformations
    2. A Game Engine handles all in-progress changes within a Game
    3. A Game Engine uses Constraints to ensure the game state is always valid, either by preventing transformations that would cause invalid relationships, or undoing transformations that caused invalid relationships.
    4. The GM can easily override the Ruleset while playing.
    5. The GM can easily CRUD Transformations while playing.
    6. The GM can easily trace rule provenance back to its rulebook origin while playing.
    7. The GM interacts with the game via a Command Line CLI
    8. The GM can easily suspend a game by doing nothing.
2. Non-functional Requirements
    1. Testability - to prevent bugs
    2. Reliability
    3. Speed - so we can play it sooner rather than later
    4. Flexibility - Transformations and Constraints, whether from rules, the GM, or the world itself, should be agnostic to any particular game system's language.
    5. Composability - for flexibility. Each transformation or constraint should be maximally composable to ensure they can express any ruleset and be programmatically enforced and executed.

## Solution

### `GameEngine` CLI Reference

- `$ GameEngine init path_to_rules path_to_campaign path_to_state` initializes a Game State object in a file at 'path_to_state', using the passed rules and campaign files
- `$ GameEngine apply_transform "example"` [most common case] creates transforms appropriate for the example and applies them to the game state. Shorthand for 'create_transform "example" && apply_all_transforms'.
- `$ GameEngine create_transform "example"` Creates all transforms appropriate for the 'example' scenario, prepends them to 'game_state.unresolved_transforms', and saves the game state file. Nothing else.
- `$ GameEngine apply_next_transform` pops and applies the last transform in the 'unresolved_transforms' queue to the game state.
- `$ GameEngine apply_all_transforms` repeatedly calls 'apply_next_transform' until the 'unresolved_transforms' queue is empty.
- `$ GameEngine test_transform "example"` for internal_testing. Identical to 'create_transform', but returns transforms as text instead of modifying 'unresolved_transforms'.

See "`GameEngine` CLI Usage Details" for more.

### `GameEngine` CLI Usage Details

All game transforms are queued into and resolved from the `{unresolved_transforms:[]}` queue on the Game State object.


#### `init`
`$ GameEngine init path_to_rules path_to_campaign path_to_game_state`
Initializes a file at path_to_game_state. It contains a Game State object with appropriate rules from path_to_rules, campaign data from path_to_campaign, and emtpy queues of unresolved_transforms and game_state_patches.

Returns a useful error when rules or campaign files are absent, either are malformed, or path_to_state exists.

#### `apply_transform` (most common use case)

`$ GameEngine apply_transform "example"`

Shorthand for `GameEngine queue_transform "example" && GameEngine apply_all_transforms`

#### `create_transform`
`$ GameEngine create_transform "example"`
Creates all transforms appropriate for the example scenario, prepends them to `game_state.unresolved_transforms`, and saves the game state file.

Returns a useful error when the passed scenario does not match a valid Transformation DSL format.

Example scenarios include:
- **"6 seconds pass"** Only transforms game time and any other active timers in `game_state.campaign_edges`.
- **"Bob casts fireball on Bob"** In this case, creates transforms appropriate for bob to cast fireball at a point centered on himself. Created transforms are automatically expanded with related transforms like targeting and resolution paths, as well as any time advancements appropriate for the transform, game state, and ruleset (like consuming Bob's action, or a "Tick" of 6 seconds for a combat turn.)
- **"override ...example..."** Creates an override transform for existing ruleset_nodes and ruleset_edges. Unlike other transforms, override patches don't change campaign_edges. Instead they go to ruleset_nodes_overrides or ruleset_edges_overrides for rule lookups.

#### `apply_next_transform`
`$ GameEngine apply_next_transform`

Validates the last item in `game_state.unresolved_transforms` against the rules, pops it off the queue, creates a patch for it, applies the patch to the game state, queues the patch into game_state_patches, and saves the game state file.

#### `apply_all_transforms`
`$ GameEngine apply_all_transforms`
Calls apply_next_transform until the unresolved_transforms queue is empty.



#### `test_transform`
`$ GameEngine test_transform "example"`
\[for internal testing] Identical to "create_transform", but returns successful results as text. Does not modify 'unresolved_transforms' or save the game state file.

### `GameEngine` CLI Implementation Details

#### `$ GameEngine create_transform "example"` details

TBD on what steps the English DSL must go through to become its final representation, but the result will be an adjacency list, and may contain tuples like this:

"Bob hurls a fireball at the Orc" becomes...
```js
{
  unresolved_transforms:[ // as tuples
  // id     source    type        sink
    ['fb',  'bob',    'casts',    'fireball'],
    ['t',   'fb',     'targets',  'tile2'],
  ]
}
```

Problems in the transformation include:
1. Fireball requires a map-point-center to target, not a creature.
2. Any synonyms in source, type, and sink must be resolved to ruleset language. For example, "hurls" isn't a term in ruleset_nodes, nor is it defined as a synonym in ruleset_edges, but the game engine needs a way to resolve the term "hurls" to "casts". Options might include hard-interrupting the GM to ask them to add an override, or less-interruptive resolutions like suggesting some synonyms and asking for approval, or asking if the GM meant a term from the ruleset_nodes, like "casts".
3. All edges necessary to relate the concepts to appropriate campaign_edges, campaign_nodes, ruleset_nodes, and ruleset_edges must be created.
4. All edges appropriate to specify and resolve time and spatial resolution must be created.



### Data Shapes
TODO restate which problems these solve to match the updated definitions and problems

#### Ruleset Data Shape
```txt
{
  ruleset_nodes:[], // List of concepts in the ruleset (e.g. Creature, PC, Multiverse, Plane, Map, TimeMode, TimeIncrement)
  ruleset_edges:[], // Adjacency list of all ruleset relationships. Includes dynamic relationships (e.g. Creature moves to Tile, Creature mounts Creature) and static relations (e.g. Map contains Tile. Creature is superset of Character. Walks is synonym for "Moves to".).
  rulset_constraints:[], // Adjacency list of Constraints expressed with the Ruleset Constraint DSL.
}
```

#### Campaign Data Shape
```txt
{
  ruleset_nodes_overrides:[], // Unordered list of gm-created rules that reference the ruleset_nodes they override.
  ruleset_edges_overrides:[], // Unordered list of gm-created rules that reference the ruleset_edges they override.
  campaign_state_nodes:[], // Unordered list of campaign-specific nodes that reference the underlying ruleset nodes e.g. `{name:Bob, reference:PC}`
  campaign_state_edges:[], // campaign-specific instances of edges between the nodes. e.g. a tuple like [Tile2, contains, firewall] if < 5 properties are necessary. Otherwise a record like `{source:Tile2, type:contains, sink:firewall}`. Avoid adding properties since they add complexity.
}
```

#### Game State Shape
Enables state saving. It leverages adjacency lists to stay extremely flat. The flatness trades speed for flexibility. Speed probably won't matter, but if it does we can always trade memory for speed with graph query caching.

[additional examples](https://docs.google.com/spreadsheets/d/1HOTb7OyPEe5LN2MiTD6JkP0n9DY2IOPA5xGQkWnbEEI/edit?gid=1721436380#gid=1721436380&range=C4).

```txt
{
  ruleset_nodes:[],         // from Ruleset Object
  ruleset_edges:[],         // from Ruleset Object
  rulset_constraints:[],    // from Ruleset Object
  campaign_state_nodes:[],  // from Campaign Object
  campaign_state_edges:[],  // from Campaign Object
  unresolved_transforms:[]  // FIFO queue of Transform DSL objects.
  game_state_patches:[],    // Chronological list of Game State Patch DSL objects.
  game_clock:0
}
```


### DSLs
Languages to express nodes, edges, transformations, and constraints.
TODO restate which problems these solve to match the updated definitions and problems

#### Transformation DSL (english & programmatic)
The transformation DSL enables GM interactions and programmatic transformations.

| English format | Edge format |
| ------------- | --------------- |
| Bob attacks Orc | `{type:'attack',source:'Bob',sink:'Orc'}` |
| Bob casts fireball at Orc | TBD. Requires multiple to express them as simple edges. |
| | |
| // edge transforms | |
| [set, edge_id, new_sink_value] | sets existing edge value |
| [delete, edge_id] | deletes the edge |
| [create, source, sink] | creates a new edge |
| | |
| // list transforms | |
| [map, transform, collection] | |
| [filter, transform, collection] | |
| [omit, transform, collection] | |


[Previous English DSL Examples here](https://docs.google.com/spreadsheets/d/1HOTb7OyPEe5LN2MiTD6JkP0n9DY2IOPA5xGQkWnbEEI/edit?gid=1637849901#gid=1637849901&range=A1) for now.

#### Ruleset Node DSL
Language to specify independent concepts in the rulebook.

{name:string}
Example
{name:"sword"}
{name:"cast"}
{name:"attack"}

#### Ruleset Edge DSL
Language to specify relations between concepts in the rulebook.

`{source:string,type:string,sink:string}`

Examples:
`{source:'Bob',type:'cast',sink:'Bob'}`
`{source:'Bob',type:'cast',sink:'Tile2'}`
`{source:'Bob',type:'attack',sink:'Orc'}`

#### Campaign Nodes

Same as Ruleset Node DSL

#### Campaign Edges

Same as Ruleset Edge DSL

#### Ruleset Constraint DSL
For expressing constraints. It includes a string format for simple comparisons, similar to many CSP solvers. The Game Engine should apply a CSP solver to check them.

Constraints can get complex. For example, "Space conflict occurs when the temporal mode is Combat, and a creature occupies Map_Tile, and another creature enters Map_Tile during their turn, and both creatures are size small or greater, and each creature differs in size by <= 2, and the current time label is Turn End." That constraint implies many sub-constraints and transformations. Writing a function for each combination of constraints is infeasible.

This DSL supports complex constraints with operators that compose, much like "and", "or","during", and "differs" in the original english.

It is an intial take, and needs exploration to ensure it makes sense and enables the Game Engine to programmatically enforce the constraints with a CSP solver.

##### Constraint Operators

| Data format | String format |
| ------------- | --------------- |
| // relational: [name, rulebook_reference, value1, value2] | |
| \[eq, rulebook_reference, value1, value2] | value1==value2 |
| \[neq, rulebook_reference, value1, value2] | value1!=value2 |
| \[gte, rulebook_reference, value1, value2] | value1>=value2 |
| \[lte, rulebook_reference, value1, value2] | value1<=value2 |
| | |
| // logic: [name, rulebook_reference, constraints...] | |
| \[and, rulebook_reference, \[constraints...]] | constraint1 && constraint2 |
| \[or, rulebook_reference, \[constraints...]] | constraint1 \|\| constraint2 |
| \[xor, rulebook_reference, \[constraints...]] | |
| \[not, rulebook_reference, constraint] | !constraint1 |
| | |
| // list reductions: [name, rulebook_reference, constraint, collection] | |
| \[exists, rulebook_reference, constraint, collection] | |
| \[sum, rulebook_reference, collection] | |
| \[product, rulebook_reference, collection] | |
| \[count, rulebook_reference, collection] | |
| \[allEqual, rulebook_reference, collection] | |
| \[allDifferent, rulebook_reference, collection] | |
| | |
| // requirements/dependencies: [name, rulebook_reference, edge_source, collection] | |
| \[requires, rulebook_reference, edge_source, constraint] | |


##### Example Constraint Operator Composition

```txt
[requires, "PHB_REF_3", Occupies, [xor, [
   [exists, "PHB_REF_4", "source==TimeMode && type==Value && sink==NonCombat", "campaign_state_edges"],
   [and,"PHB_REF_5", [
      [exists, "PHB_REF_6", "source==TickPhase && type==Value && sink=='TurnEnd'", "campaign_state_edges"],
      [exists, "source==TimeMode && type==Value && sink==Combat", "campaign_state_edges"],
      [eq, 1, [count, [filter, "type==Occupies && source==Creature && sink==required_node_sink", "campaign_state_edges"]]]
   ]]
]]]
```

#### Game State Patch DSL

Undoable/replayable diff of a transform application's changed nodes and edges in `campaign_state_nodes` and `campaign_state_edges`.

Format TBD

