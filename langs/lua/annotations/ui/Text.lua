---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_ui::LiteText
-----------------------------------------------------------

--- <p>Text is a simple widget that allows you to print text on screen. It has various options like word wrapping, text
--- alignment, and so on.</p>
---@class Text_static
Text = {}

---@class Text
---@field text_async string <p>sets the text of UI element. applied at the end of frame.</p>
Text_instance = {}

---@param state TextBuilder
---@return Text
function Text:new(state) end

