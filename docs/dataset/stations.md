# Station Configuration

Stations define the individual frequencies, services, or logical sectors within an FIR. They represent _what can be called_ over the voice system (e.g. Delivery, Ground, Tower, Approach sectors), independently of how controllers log in on VATSIM.

Station configuration is stored in stations.toml or stations.json inside an FIR's dataset directory.

## File Location

```
dataset/{FIR_CODE}/stations.{toml|json}
```

## File Structure

The file must contain a single top-level array named `stations`.

TOML

```toml
[[stations]]
# Station 1 definition

[[stations]]
# Station 2 definition
```

JSON representation:

```jsonc
{
  "stations": [
    {
      /* Station 1 definition */
    },
    {
      /* Station 2 definition */
    },
  ],
}
```

## Key Terms

- **Station**: A callable frequency/service (e.g., `LOWW_TWR`, `LOWW_N1`)
- **Position**: A controller login that can cover one or more stations (e.g., `LOWW_TWR` or `LOVV_N_CTR`)
- **Coverage**: Which position is currently handling a station and will receive calls for it

## Station Fields

Each station entry describes one callable station.

| Field           | Type             | Required                 | Description                                                                                                             |
| :-------------- | :--------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------- |
| `id`            | String           | Yes                      | Globally unique station identifier (e.g., `LOWW_TWR`, `LOVV_N1`). Must start with the FIR's two-letter country code.    |
| `parent_id`     | String           | No                       | Optional parent station ID, used to inherit coverage from another station.                                              |
| `controlled_by` | Array of strings | If no `parent_id` is set | Ordered list of Position IDs used for coverage resolution (highest priority first). Required unless `parent_id` is set. |

## Validation Rules

The following validation rules apply:

- `id` must be unique across the entire dataset
- `controlled_by`:
  - must contain at least one position if present
  - must not contain duplicates
- If a station does not define `parent_id`, it must define `controlled_by`
- If a station defines `parent_id`, `controlled_by` becomes optional

## Coverage Priority (`controlled_by`)

The `controlled_by` field defines the **coverage order** for a station.

It is an ordered list of Position IDs, evaluated from start to end:

- The first online position in the list is considered to be covering the station
- If the highest-priority position goes offline, coverage falls back to the next one
- If multiple listed positions are online at the same time, only the first matching one is used

This field is used for **coverage calculations**.

## Station Inheritance (`parent_id`)

The `parent_id` field allows a station to **inherit coverage priority** from another station.

Inheritance works by **appending** the resolved `controlled_by` list of the parent station to the station's own `controlled_by` list.

This process is transitive: if the parent station itself has a `parent_id`, its parent's coverage list is appended as well.

## Coverage Resolution

The effective coverage list for a station is resolved as follows:

1. Start with the station's own `controlled_by` list (if defined)
2. If `parent_id` is set:
   - Resolve the parent station's coverage list
   - Append it to the station's list
3. Repeat recursively until no `parent_id` remains
4. The final list is evaluated in order for online positions

If a position ID appears multiple times in the final list (e.g. because it is listed in `controlled_by` for multiple stations in the inheritance chain), only the first occurrence is used.

## Common pitfalls

When configuring stations, watch out for these issues:

- **Order matters**: put the position with the highest priority first
- **Missing positions**: make sure all positions referenced by `controlled_by` exist
- **Circular inheritance**: if a `parent_id` points to a station already processed in the inheritance chain, the coverage resolution will stop at that point and the resulting coverage list will be incomplete

## Examples

### Simple fallback coverage

This example defines a station that is normally handled by one position, but can be covered by another if the primary controller is offline.

```toml
[[stations]]
id = "LOWW_TWR"
controlled_by = ["LOWW_TWR", "LOWW_E_TWR"]
```

What this means in practice:

- If `LOWW_TWR` is online, it covers the Tower station
- If `LOWW_TWR` goes offline, but `LOWW_E_TWR` is online, coverage falls back to `LOWW_E_TWR`
- If both are online, `LOWW_TWR` has priority over `LOWW_E_TWR` and thus covers the station

### Coverage chain using parent ID

This example defines a station with coverage inheritance using a `parent_id`.

```toml
[[stations]]
id = "LOWW_DEL"
parent_id = "LOWW_GND"
controlled_by = ["LOWW_DEL"]

[[stations]]
id = "LOWW_GND"
parent_id = "LOWW_TWR"
controlled_by = ["LOWW_GND", "LOWW_W_GND"]

[[stations]]
id = "LOWW_TWR"
controlled_by = ["LOWW_TWR", "LOWW_E_TWR"]
```

What this means in practice:

- `LOWW_DEL` is controlled by the same-named position `LOWW_DEL`
- Since `LOWW_DEL` has `LOWW_GND` as a parent, its coverage list is extended with the coverage list of `LOWW_GND` (`LOWW_GND`, `LOWW_W_GND`)
- Since `LOWW_GND` has `LOWW_TWR` as a parent, its coverage list is extended with the coverage list of `LOWW_TWR` (`LOWW_TWR`, `LOWW_E_TWR`)
- The final coverage list for `LOWW_DEL` is `LOWW_DEL`, `LOWW_GND`, `LOWW_W_GND`, `LOWW_TWR`, `LOWW_E_TWR`
- If no lower level position is online, `LOWW_DEL` will be covered by `LOWW_E_TWR`

### Stable deduplication

In this example, some positions appear both in a station and in its parent. This is allowed and expected. The final coverage list is deduplicated in a stable manner, meaning that the order of the remaining elements is preserved and only the first occurrence of each position is kept.

```toml
[[stations]]
id = "LOWW_W_GND"
parent_id = "LOWW_GND"
controlled_by = ["LOWW_W_GND", "LOWW_GND", "LOWW_TWR", "LOWW_E_TWR"]

[[stations]]
id = "LOWW_GND"
parent_id = "LOWW_TWR"
controlled_by = ["LOWW_GND", "LOWW_W_GND", "LOWW_E_TWR", "LOWW_TWR"]
```

What this means in practice:

- `LOWW_W_GND` is controlled by `LOWW_W_GND`, `LOWW_GND`, `LOWW_TWR` and `LOWW_E_TWR` in that order
- Since `LOWW_W_GND` has `LOWW_GND` as a parent, its coverage list is extended with the coverage list of `LOWW_GND` (`LOWW_GND`, `LOWW_W_GND`, `LOWW_E_TWR`, `LOWW_TWR`)
- The parent station's coverage is added to the end of the list, resulting in `LOWW_W_GND`, `LOWW_GND`, `LOWW_TWR`, `LOWW_E_TWR`, `LOWW_GND`, `LOWW_W_GND`, `LOWW_E_TWR`, `LOWW_TWR`
- After stable deduplication, the final coverage list for `LOWW_W_GND` is `LOWW_W_GND`, `LOWW_GND`, `LOWW_TWR`, `LOWW_E_TWR`
- This ensures that coverage priority is properly maintained even if stations define overlapping coverage lists

### Station without its own coverage list

In this example, some stations do not define their own coverage list, but instead rely entirely on their parents' coverage list.

```toml
[[stations]]
id = "LOVV_N5"
parent_id = "LOVV_N6"

[[stations]]
id = "LOVV_N6"
parent_id = "LOVV_N7"

[[stations]]
id = "LOVV_N7"
controlled_by = [
  "LOVV_NU_CTR",
  "LOVV_EU_CTR",
  "LOVV_U_CTR",
  "LOVV_N_CTR",
  "LOVV_E_CTR",
  "LOVV_CTR",
  "LOVV_C_CTR",
]
```

What this means in practice:

- `LOVV_N5` has no `controlled_by` list of its own, so its coverage list is determined entirely by its parent `LOVV_N6`
- `LOVV_N6` has no coverage list of its own either, but defines `LOVV_N7` as its parent
- `LOVV_N7` is controlled by `LOVV_NU_CTR`, `LOVV_EU_CTR`, `LOVV_U_CTR`, `LOVV_N_CTR`, `LOVV_E_CTR`, `LOVV_CTR`, `LOVV_C_CTR`
- The final coverage list for `LOVV_N5` (and `LOVV_N6`) is `LOVV_NU_CTR`, `LOVV_EU_CTR`, `LOVV_U_CTR`, `LOVV_N_CTR`, `LOVV_E_CTR`, `LOVV_CTR`, `LOVV_C_CTR`
