# State Transformation Syntax


## Problem
1. We want to play a game where:
    1. Rules from rule sources are highly variable
    2. Rules from rule sources transform the world while playing
    3. The GM transforms the world for players when they Act while playing
    4. The world maintains its ongoing transforms while playing
    5. Rules from rule sources constrain transformations while playing
    6. The world is always in a valid state.
    7. The GM can override rules while playing.
    8. The GM can easily undo world transformations while playing.
    9. The GM can easily trace rule provenance back to its source while playing.
    10. The GM can suspend a game without losing the state of the world or rules while playing.
2. Non-functional Requirements
    1. Testability - to prevent bugs
    2. Reliability
    3. Speed - so we can play it sooner rather than later

## Criteria for a Good Solution
For 1.1, ensure maximum flexibility for different game systems. Transformations and constraints, whether from rules, the GM, or the world itself, should be agnostic to any particular game system's language. Each transformation or constraint should be maximally composable to ensure they can be can programmatically enforced and executed. See the Constraint DSL for additional details.

## Solutions
To facilitate composability, all solutions are normalized to graph nodes and directed edges in the game state, making it largely a collection of adjacency lists. Every node and edge within the following game state graph is transformable and constrainable via its following DSLs. Further, each DSL's items are as single-responsiblity and composable as possible (or will be after we iterate on them).

### Game State

For 1.2-1.10 & 2.1-2.3, we express all possible influences on the game state including transformation rules, constraint rules, rule overrides, player acts, and all other game data as part of the game state. Below is a minimal example, and here are [additional examples](https://docs.google.com/spreadsheets/d/1HOTb7OyPEe5LN2MiTD6JkP0n9DY2IOPA5xGQkWnbEEI/edit?gid=1721436380#gid=1721436380&range=C4).

```txt
{
  ruleset_nodes:[], // concepts in the ruleset (e.g. Creature, PC, Multiverse, Plane, Map, TimeIncrement)
  ruleset_edges:[], // valid directed node>node edge types. Includes static relations (e.g. chest contains sword. creature is superset of PC.) and dynamic relations aka transformations (PC moves to Tile)
  rulset_constraints:[], // enforces edge constraints
  campaign_state_nodes:[], // campaign-specific instances of ruleset_nodes
  campaign_state_edges:[], // campaign-specific instances of ruleset_edges
  game_state_patches:[], undoable/replayable patches to the game state. Appended when time Ticks or the GM overrides something.
}
```

### DSLs




#### English DSL
For GM interactions (1.3,1.7-1.9) we provide an english DSL (e.g. "Bob moves west"). The application code converts that english DSL into transforms and constraints.

[English DSL Examples here](https://docs.google.com/spreadsheets/d/1HOTb7OyPEe5LN2MiTD6JkP0n9DY2IOPA5xGQkWnbEEI/edit?gid=1637849901#gid=1637849901&range=A1) for now.


#### Transformation DSL
For transforms - 1.2-1.4: We provide a transform DSL. The application code contains a transformation function to create patches from those transforms, and apply those patches to the game state. Patches are applied on each "Tick" and on GM overrides.

| Data format | Note |
| ------------- | --------------- |
| // list transforms | |
| [map, transform, collection] | |
| [filter, transform, collection] | |
| [omit, transform, collection] | |
| | |
| // edge transforms | |
| [set, edge_id, new_sink_value] | sets existing edge value |
| [delete, edge_id] | deletes the edge |
| [create, source, sink] | creates a new edge |

#### Constraint DSL
For constraints and state validity - 1.5,1.6: We provide a constraint DSL. It includes a string format for readability when practical, similar to many CSP solvers. The application code contains a constraint solver to check them.

Consider a spatial conflict constraint: "Space conflict occurs when the temporal mode is Combat, and a creature occupies Map_Tile, and another creature enters Map_Tile during their turn, and both creatures are size small or greater, and each creature differs in size by <= 2, and the current time is Turn End."

That constraint implies many sub-constraints and transformations. Writing a function for each combination of constraints is infeasible. This DSL addresses feasibility with a small set of constraint operations that can be composed into any larger constraint, much like the english does. The final step of rulebook ingestion a set of concept nodes, relationship edges (including transformations), and a set of constraints comprised from the Constraint DSL's items. (TBD if that statement works for Alan)

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

Note: the constraints may change if implementing a CSP solver is significantly slower than adopting a library to do it.

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
| | |

