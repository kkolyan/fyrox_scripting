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
| `mouse_move` | [Vector2](../Math/Vector2.md) | get |  |
| `mouse_scroll` | [Vector2](../Math/Vector2.md) | get |  |
## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| bool | `is_mouse_button_down` ( int <ins>button</ins> ) | is mouse button with ID `button` is in pressed state? |
| bool | `is_mouse_button_up` ( int <ins>button</ins> ) |  |
| bool | `is_mouse_button_pressed` ( int <ins>button</ins> ) |  |
| bool | `is_key_down` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `is_key_up` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `is_key_pressed` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |

