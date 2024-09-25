extends Label

# Called when the node enters the scene tree for the first time.
func _process(_delta: float) -> void:
	self.text = "[%s] %s" % ["SERVER" if OS.has_feature("dedicated_server") else "CLIENT", multiplayer.get_unique_id()]
