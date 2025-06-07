# Input
class in FyroxLite.LiteInput
## Description
Utility class to poll player input events
## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| MouseLeft | int | 0 | Left Mouse Button ID |
| MouseRight | int | 1 |  |
| MouseMiddle | int | 2 |  |
| MouseBack | int | 3 |  |
| MouseForward | int | 4 |  |
## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| MouseMove | [Vector2](../LiteMath/Vector2.md) | get |  |
| MouseScroll | [Vector2](../LiteMath/Vector2.md) | get |  |
## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| bool | [IsMouseButtonDown](##) ( int button ) | is mouse button with ID `button` is in pressed state? |
| bool | [IsMouseButtonUp](##) ( int button ) |  |
| bool | [IsMouseButtonPressed](##) ( int button ) |  |
| bool | [IsKeyDown](##) ( [KeyCode](../LiteInput/KeyCode.md) key ) |  |
| bool | [IsKeyUp](##) ( [KeyCode](../LiteInput/KeyCode.md) key ) |  |
| bool | [IsKeyPressed](##) ( [KeyCode](../LiteInput/KeyCode.md) key ) |  |

