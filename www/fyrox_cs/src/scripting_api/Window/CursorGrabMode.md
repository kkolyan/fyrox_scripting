# CursorGrabMode
enum in [FyroxLite](../../scripting_api.md).[Window](../Window.md)

## Properties
| Property | Description |
|---|---|
| `None` | <p>No grabbing of the cursor is performed.</p> |
| `Confined` | <p>The cursor is confined to the window area.</p> <p>There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you want to do so.</p> <h2>Platform-specific</h2> <ul> <li><strong>macOS:</strong> Not implemented. Always returns [<code>ExternalError::NotSupported</code>] for now.</li> <li><strong>iOS / Android / Web:</strong> Always returns an [<code>ExternalError::NotSupported</code>].</li> </ul> |
| `Locked` | <p>The cursor is locked inside the window area to the certain position.</p> <p>There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you want to do so.</p> <h2>Platform-specific</h2> <ul> <li><strong>X11 / Windows:</strong> Not implemented. Always returns [<code>ExternalError::NotSupported</code>] for now.</li> <li><strong>iOS / Android:</strong> Always returns an [<code>ExternalError::NotSupported</code>].</li> </ul> |

