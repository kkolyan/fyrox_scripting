---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_ui::Brush
-----------------------------------------------------------

--- <p>Brush defines a way to fill an arbitrary surface.</p>
---@class Brush
---@field solid_color Color? ---<p>A brush, that fills a surface with a solid color.</p>
---@field linear_gradient LinearGradient? ---<p>A brush, that fills a surface with a linear gradient, which is defined by two points in local coordinates and a set of stop points. See [<code>GradientPoint</code>] for more info.</p>
---@field radial_gradient RadialGradient? ---<p>A brush, that fills a surface with a radial gradient, which is defined by a center point in local coordinates and a set of stop points. See [<code>GradientPoint</code>] for more info.</p>
Brush_instance = {}

