# Input
class in [FyroxLite](../../scripting_api.md).[Input](../Input.md)
## Description
Utility class to poll player input events
## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| `MouseLeft` | int | 0 | Left Mouse Button ID |
| `MouseRight` | int | 1 |  |
| `MouseMiddle` | int | 2 |  |
| `MouseBack` | int | 3 |  |
| `MouseForward` | int | 4 |  |
## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `MouseMove` | [Vector2](../Math/Vector2.md) | get |  |
| `MouseScroll` | [Vector2](../Math/Vector2.md) | get |  |
## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| bool | `IsMouseButtonDown` ( int <ins>button</ins> ) | is mouse button with ID `button` is in pressed state? |
| bool | `IsMouseButtonUp` ( int <ins>button</ins> ) |  |
| bool | `IsMouseButtonPressed` ( int <ins>button</ins> ) |  |
| bool | `IsKeyDown` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `IsKeyUp` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `IsKeyPressed` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |

