---@uuid f5f9c3b0-5e5b-4cf3-ae27-649ba3a7b56a
---@class Game : GlobalScript
---@field player Node
---@field beacons Vector3[]
---@field frags number
---@field wounds number
---@field hud Text
Game = script_class()

---@param scene_path string
function Game:init(scene_path)
	if not scene_path then
		scene_path = "data/scene.rgs"
	end
	Scene:load_async(scene_path)

	self.hud = Text:new({
		font_size = 40,
		foreground = {
			solid_color = Color.BLACK
		}
	})
	self.beacons = {}
end

function Game:update()
	if Input:is_key_down(KeyCode.Escape) then
		os.exit()
	end
	self.hud.text_async = string.format("Wounds: %s\nKilled Guards: %s", self.wounds, self.frags)
end

function Game:inc_frags()
	self.frags = self.frags + 1
end

function Game:inc_wounds()
	self.wounds = self.wounds + 1
end


---@generic T
---@param o T?
---@param msg string
---@return T
function not_nil(o, msg)
    if o then
        return o
    else
        error(msg)
    end
end
