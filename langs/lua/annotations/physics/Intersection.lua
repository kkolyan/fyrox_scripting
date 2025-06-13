---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields

-----------------------------------------------------------
------ fyrox_lite::lite_physics::LiteIntersection
-----------------------------------------------------------

--- <p>A ray intersection result.</p>
---@class Intersection
---@field collider Node ---<p>A handle of the collider with which intersection was detected.</p>
---@field normal Vector3 ---<p>A normal at the intersection position.</p>
---@field position Vector3 ---<p>A position of the intersection in world coordinates.</p>
---@field feature FeatureId ---<p>Additional data that contains a kind of the feature with which intersection was detected as well as its index.</p> <h1>Important notes.</h1> <p>FeatureId::Face might have index that is greater than amount of triangles in a triangle mesh, this means that intersection was detected from “back” side of a face. To “fix” that index, simply subtract amount of triangles of a triangle mesh from the value.</p>
---@field toi number ---<p>Distance from the ray origin.</p>
Intersection_instance = {}

