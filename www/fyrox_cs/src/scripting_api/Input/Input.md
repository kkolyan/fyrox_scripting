# Input
class in [FyroxLite](../../scripting_api.md).[Input](../Input.md)

## Description
<p>Utility class to poll player input events</p>

## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| `MouseLeft` | int | 0 | <p>Left Mouse Button ID</p> |
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
| bool | `IsMouseButtonDown` ( int <ins>button</ins> ) | <p>is mouse button with ID <code>button</code> is in pressed state?</p> |
| bool | `IsMouseButtonUp` ( int <ins>button</ins> ) |  |
| bool | `IsMouseButtonPressed` ( int <ins>button</ins> ) |  |
| bool | `IsKeyDown` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `IsKeyUp` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `IsKeyPressed` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
