extends Node

@export var mob_scene: PackedScene

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mob_timer_timeout() -> void:
	# Instantiate mob scene
	var mob := mob_scene.instantiate()

	# Sample random position on spawn path
	var mob_spawn_loc := $SpawnPath/SpawnLocation
	mob_spawn_loc.progress_ratio = randf()

	# Get player position
	var player_pos = $Player.position

	# Call mob init method
	mob.initialise(mob_spawn_loc.position, player_pos)

	# Add mob as child of this node
	add_child(mob)


func _on_player_hit() -> void:
	$MobTimer.stop()
