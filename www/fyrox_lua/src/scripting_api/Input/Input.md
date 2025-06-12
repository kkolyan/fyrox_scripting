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
| `mouse_move` | [Vector2](../Math/Vector2.md) | get |  |
| `mouse_scroll` | [Vector2](../Math/Vector2.md) | get |  |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| bool | `is_mouse_button_down` ( int <ins>button</ins> ) | <p>is mouse button with ID <code>button</code> is in pressed state?</p> |
| bool | `is_mouse_button_up` ( int <ins>button</ins> ) |  |
| bool | `is_mouse_button_pressed` ( int <ins>button</ins> ) |  |
| bool | `is_key_down` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `is_key_up` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |
| bool | `is_key_pressed` ( [KeyCode](../Input/KeyCode.md) <ins>key</ins> ) |  |

