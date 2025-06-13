---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LitePhysics
-----------------------------------------------------------

---@class Physics_static
---@field EXCLUDE_FIXED number ---<p>Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached.</p>
---@field EXCLUDE_KINEMATIC number ---<p>Exclude from the query any collider attached to a kinematic rigid-body.</p>
---@field EXCLUDE_DYNAMIC number ---<p>Exclude from the query any collider attached to a dynamic rigid-body.</p>
---@field EXCLUDE_SENSORS number ---<p>Exclude from the query any collider that is a sensor.</p>
---@field EXCLUDE_SOLIDS number ---<p>Exclude from the query any collider that is not a sensor.</p>
---@field ONLY_DYNAMIC number ---<p>Excludes all colliders not attached to a dynamic rigid-body.</p>
---@field ONLY_KINEMATIC number ---<p>Excludes all colliders not attached to a kinematic rigid-body.</p>
---@field ONLY_FIXED number ---<p>Exclude all colliders attached to a non-fixed rigid-body (this will not exclude colliders not attached to any rigid-body).</p>
Physics = {}

---@class Physics
Physics_instance = {}

---@param opts RayCastOptions
---@return Intersection[]
function Physics:cast_ray(opts) end

