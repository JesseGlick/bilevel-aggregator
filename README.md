# bilevel-aggregator

Aggregate groups of groups.

This crate provides structures to group and aggregate data by some composite key value, and then group the aggregated results by some component of that key. The unique key for each row of aggregated data is called the *full key*, the key by which aggregated results are grouped is called the *group key*, and that portion of the full key that does not form part of the *group key* is called the *aggregation key*. The group key and aggregation key can each be simple or composite.

Two primary structures are provided: BilevelSet, which merely lists the aggregation keys associated with each group key, and BilevelMap, which allows a payload to be kept for each pairing.

Three implementations are provided:

- One for where the group key and the aggregation key are copy types.
- One for where neither the group key nor the aggregation key is a copy type.
- One for where the group key is a copy type, but the aggregation key is not.
