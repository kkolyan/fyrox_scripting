---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteRayCastOptions
-----------------------------------------------------------

---@class RayCastOptions
---@field ray_origin Vector3 ---<p>A ray origin.</p>
---@field ray_direction Vector3 ---<p>A ray direction. Can be non-normalized.</p>
---@field max_len number ---<p>Maximum distance of cast.</p>
---@field groups InteractionGroups? ---<p>Groups to check.</p>
---@field sort_results boolean ---<p>Whether to sort intersections from closest to farthest.</p>
RayCastOptions_instance = {}

