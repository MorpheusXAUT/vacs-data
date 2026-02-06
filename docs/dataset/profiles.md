# Profile Configuration

Profiles define the layout and organization of direct access keys for voice communication. They provide a customizable interface that determines how controllers access and call stations within vacs.

Profile configuration is stored as individual JSON files in the `profiles/` directory inside an FIR's dataset directory.

## File Location

```
dataset/{FIR_CODE}/profiles/{PROFILE_ID}.json
```

Each profile is stored in a separate JSON file named after the profile's `id`.

## File Structure

Each profile file must contain a single profile object. There are two types of profiles available:

- **Tabbed Profile**: Organizes keys into multiple tabs with a grid layout
- **Geo Profile**: Uses a flexible geometric layout with containers, buttons, and dividers

## Key Terms

- **Profile**: A layout configuration that defines how direct access keys are organized and displayed
- **Direct Access Key**: A button that allows calling a specific station
- **Station ID**: Reference to a station defined in the stations configuration
- **Tab**: A page in a tabbed profile containing a grid of keys
- **Geo Node**: A building block in a geo profile (container, button, or divider)
- **Container**: A layout element that groups and arranges child nodes

## Profile Types

### Tabbed Profile

A tabbed profile organizes keys into multiple tabs, each containing a grid of direct access keys.

**Required fields:**

- `id` (string): Unique profile identifier
- `type` (string): Must be `"Tabbed"`
- `tabs` (array): One or more tabs, each with a label and page

**Structure:**

```jsonc
{
  "id": "LOWW",
  "type": "Tabbed",
  "tabs": [
    {
      "label": "Tab Name",
      "page": {
        "rows": 4,
        "keys": [
          /* array of direct access keys */
        ],
      },
    },
  ],
}
```

### Geo Profile

A geo profile uses a flexible container-based layout system, allowing for custom geometric arrangements of buttons and dividers.

**Required fields:**

- `id` (string): Unique profile identifier
- `type` (string): Must be `"Geo"`
- `direction` (string): Flex direction (`"row"` or `"col"`)
- `children` (array): One or more geo nodes (containers, buttons, or dividers)

**Structure:**

```jsonc
{
  "id": "LOVV",
  "type": "Geo",
  "direction": "col",
  "children": [
    /* array of geo nodes */
  ],
}
```

## Profile Fields

### Common Profile Fields

| Field  | Type   | Required | Description                                                                               |
| :----- | :----- | :------- | :---------------------------------------------------------------------------------------- |
| `id`   | String | Yes      | Unique profile identifier. Must start with the FIR's country code (e.g., `LOWW`, `LOVV`). |
| `type` | String | Yes      | Profile type. Must be either `"Tabbed"` or `"Geo"`.                                       |

## Tabbed Profile Components

### Tab

Represents a single tab in a tabbed profile.

| Field   | Type             | Required | Description                           |
| :------ | :--------------- | :------- | :------------------------------------ |
| `label` | String           | Yes      | Tab name displayed in the interface.  |
| `page`  | DirectAccessPage | Yes      | The page containing the grid of keys. |

### DirectAccessPage

Defines a grid layout of direct access keys.

| Field  | Type                     | Required | Description                             |
| :----- | :----------------------- | :------- | :-------------------------------------- |
| `rows` | Integer                  | Yes      | Number of rows in the grid (minimum 1). |
| `keys` | Array of DirectAccessKey | Yes      | Array of keys to display in the grid.   |

### DirectAccessKey

Represents a single callable button.

| Field        | Type             | Required | Description                                                                                                               |
| :----------- | :--------------- | :------- | :------------------------------------------------------------------------------------------------------------------------ |
| `label`      | Array of strings | Yes      | Multi-line label (up to 3 lines). Can be empty array for blank keys.                                                      |
| `station_id` | String           | No       | Station ID to call when pressed. Mutually exclusive with `page`. If neither is specified, the button will be disabled.    |
| `page`       | DirectAccessPage | No       | Subpage to open when pressed. Mutually exclusive with `station_id`. If neither is specified, the button will be disabled. |

## Geo Profile Components

### GeoPageContainer

A layout container that arranges child nodes using flexbox.

| Field             | Type              | Required | Description                                     |
| :---------------- | :---------------- | :------- | :---------------------------------------------- |
| `direction`       | String            | Yes      | Flex direction: `"row"` or `"col"`.             |
| `children`        | Array of GeoNodes | Yes      | Child nodes (containers, buttons, or dividers). |
| `height`          | String            | No       | Container height (e.g., `"100%"`, `"20rem"`).   |
| `width`           | String            | No       | Container width (e.g., `"100%"`, `"20rem"`).    |
| `padding`         | Number            | No       | Padding on all sides (≥ 0).                     |
| `padding_left`    | Number            | No       | Left padding (≥ 0).                             |
| `padding_right`   | Number            | No       | Right padding (≥ 0).                            |
| `padding_top`     | Number            | No       | Top padding (≥ 0).                              |
| `padding_bottom`  | Number            | No       | Bottom padding (≥ 0).                           |
| `gap`             | Number            | No       | Gap between child elements (≥ 0).               |
| `justify_content` | String            | No       | Flexbox justify-content property.               |
| `align_items`     | String            | No       | Flexbox align-items property.                   |

**Valid `justify_content` values:**

- `"flex-start"`, `"flex-end"`, `"center"`, `"space-between"`, `"space-around"`, `"space-evenly"`

**Valid `align_items` values:**

- `"flex-start"`, `"flex-end"`, `"center"`, `"stretch"`, `"baseline"`

### GeoPageButton

A clickable button that can trigger a direct access page or call a station.

| Field   | Type             | Required | Description                                                |
| :------ | :--------------- | :------- | :--------------------------------------------------------- |
| `label` | Array of strings | Yes      | Multi-line label (1-3 lines, cannot be empty).             |
| `size`  | Number           | Yes      | Button size (> 0). Controls relative sizing in the layout. |
| `page`  | DirectAccessPage | No       | Optional nested page to display when button is pressed.    |

### GeoPageDivider

A visual divider line used to separate sections.

| Field         | Type   | Required | Description                                               |
| :------------ | :----- | :------- | :-------------------------------------------------------- |
| `orientation` | String | Yes      | Line orientation: `"horizontal"` or `"vertical"`.         |
| `thickness`   | Number | Yes      | Line thickness in pixels (> 0).                           |
| `color`       | String | Yes      | CSS color (e.g., `"#000000"`, `"rgb(0,0,0)"`, `"black"`). |
| `oversize`    | Number | No       | Extension beyond container bounds in pixels (> 0).        |

## Validation Rules

The following validation rules apply:

- `id` must be unique across the entire dataset
- `id` must start with the FIR's two-letter country code
- For Tabbed Profiles:
  - Must have at least one tab
  - Each tab must have a non-empty label
  - Each page must have at least one row
- For Geo Profiles:
  - Must have a valid direction (`"row"` or `"col"`)
  - Must have at least one child node
  - All size values (`height`, `width`) must match pattern `^\d+(%|rem)$`
  - All numeric values (padding, gap, size, thickness) must be appropriate to their type (non-negative or positive)
- Labels:
  - Can have up to 3 lines
  - Geo button labels must have at least 1 line
  - Direct access key labels can be empty (for blank keys)
- `station_id` references must exist in the stations configuration

## Common Pitfalls

When configuring profiles, watch out for these issues:

- **Invalid station references**: Ensure all `station_id` values reference existing stations in your stations configuration.
- **Size unit format**: Container sizes must include the unit (`%` or `rem`), e.g., `"100%"` not `"100"`.
- **Empty geo button labels**: Geo buttons require at least one line in their label (unlike direct access keys which can be blank).
- **Negative values**: Padding, gap, size, and thickness values must be non-negative (or positive where specified).
- **Invalid flexbox values**: Use only the documented values for `justify_content` and `align_items`.

## Examples

### Simple tabbed profile

This example defines a basic tabbed profile with one tab containing a 4-row grid of keys.

```json
{
  "id": "LOWW",
  "type": "Tabbed",
  "tabs": [
    {
      "label": "Ground",
      "page": {
        "rows": 4,
        "keys": [
          {
            "label": ["LOWW", "Delivery"],
            "station_id": "LOWW_DEL"
          },
          {
            "label": ["LOWW", "Ground"],
            "station_id": "LOWW_GND"
          },
          {
            "label": ["LOWW", "Tower"],
            "station_id": "LOWW_TWR"
          },
          {
            "label": []
          }
        ]
      }
    }
  ]
}
```

What this means in practice:

- The profile is identified as `LOWW` and can be referenced by positions
- It displays a single tab labeled "Ground"
- The tab contains a 4-row grid with three active keys and one blank key
- Each active key has a two-line label and calls the corresponding station when pressed
- The blank key (empty label array) occupies space but is not callable

### Multi-tab profile

This example shows a tabbed profile with multiple tabs for different operational areas.

```json
{
  "id": "LOWW",
  "type": "Tabbed",
  "tabs": [
    {
      "label": "Aerodrome",
      "page": {
        "rows": 3,
        "keys": [
          {
            "label": ["LOWW", "Delivery"],
            "station_id": "LOWW_DEL"
          },
          {
            "label": ["LOWW", "Ground"],
            "station_id": "LOWW_GND"
          },
          {
            "label": ["LOWW", "Tower"],
            "station_id": "LOWW_TWR"
          }
        ]
      }
    },
    {
      "label": "Approach",
      "page": {
        "rows": 2,
        "keys": [
          {
            "label": ["LOWW", "Approach"],
            "station_id": "LOWW_APP"
          },
          {
            "label": ["LOWW", "Arrival"],
            "station_id": "LOWW_F_APP"
          }
        ]
      }
    }
  ]
}
```

What this means in practice:

- The profile has two tabs that controllers can switch between
- The "Aerodrome" tab contains a 3-row grid with ground operations keys
- The "Approach" tab contains a 2-row grid with approach control keys
- Controllers can quickly navigate between different operational contexts

### Basic geo profile

This example demonstrates a simple geo profile using a vertical container layout.

```json
{
  "id": "LOVV",
  "type": "Geo",
  "direction": "col",
  "height": "100%",
  "width": "100%",
  "gap": 8,
  "children": [
    {
      "direction": "row",
      "gap": 8,
      "children": [
        {
          "label": ["LOVV", "N5"],
          "size": 1,
          "page": {
            "rows": 2,
            "keys": [
              {
                "label": ["LOVV N5"],
                "station_id": "LOVV_N5"
              },
              {
                "label": ["LOVV N6"],
                "station_id": "LOVV_N6"
              }
            ]
          }
        },
        {
          "label": ["LOVV", "S5"],
          "size": 1,
          "page": {
            "rows": 2,
            "keys": [
              {
                "label": ["LOVV S5"],
                "station_id": "LOVV_S5"
              },
              {
                "label": ["LOVV S6"],
                "station_id": "LOVV_S6"
              }
            ]
          }
        }
      ]
    }
  ]
}
```

What this means in practice:

- The profile uses a geometric layout instead of fixed tabs
- The root container is a vertical column (`"col"`) that fills the full available space
- Inside is a horizontal row (`"row"`) containing two equally-sized buttons (`size: 1`)
- Each button opens a nested direct access page when pressed
- The nested pages contain grids of station keys for different sectors
