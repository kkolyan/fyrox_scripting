# CursorGrabMode
enum in [FyroxLite](../../scripting_api.md).[Window](../Window.md)
## Properties
| Property | Description |
|---|---|
| `None` | No grabbing of the cursor is performed. |
| `Confined` | The cursor is confined to the window area.

There's no guarantee that the cursor will be hidden. You should hide it by yourself if you
want to do so.

## Platform-specific

- **macOS:** Not implemented. Always returns [`ExternalError::NotSupported`] for now.
- **iOS / Android / Web:** Always returns an [`ExternalError::NotSupported`]. |
| `Locked` | The cursor is locked inside the window area to the certain position.

There's no guarantee that the cursor will be hidden. You should hide it by yourself if you
want to do so.

## Platform-specific

- **X11 / Windows:** Not implemented. Always returns [`ExternalError::NotSupported`] for now.
- **iOS / Android:** Always returns an [`ExternalError::NotSupported`]. |

