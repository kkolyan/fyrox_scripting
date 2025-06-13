---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_window::LiteCursorGrabMode
-----------------------------------------------------------

---@class CursorGrabMode_static
CursorGrabMode = {}

--- <p>No grabbing of the cursor is performed.</p>
---@type CursorGrabMode
CursorGrabMode.None = {}

--- <p>The cursor is confined to the window area.</p>
--- <p>There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you
--- want to do so.</p>
--- <h2>Platform-specific</h2>
--- <ul>
--- <li><strong>macOS:</strong> Not implemented. Always returns [<code>ExternalError::NotSupported</code>] for now.</li>
--- <li><strong>iOS / Android / Web:</strong> Always returns an [<code>ExternalError::NotSupported</code>].</li>
--- </ul>
---@type CursorGrabMode
CursorGrabMode.Confined = {}

--- <p>The cursor is locked inside the window area to the certain position.</p>
--- <p>There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you
--- want to do so.</p>
--- <h2>Platform-specific</h2>
--- <ul>
--- <li><strong>X11 / Windows:</strong> Not implemented. Always returns [<code>ExternalError::NotSupported</code>] for now.</li>
--- <li><strong>iOS / Android:</strong> Always returns an [<code>ExternalError::NotSupported</code>].</li>
--- </ul>
---@type CursorGrabMode
CursorGrabMode.Locked = {}
CursorGrabMode = {}

---@class CursorGrabMode
CursorGrabMode_instance = {}

