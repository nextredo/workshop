extends Node

@export var mob_scene: PackedScene


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

	# Connect the mob to the score label
	# (so the score updates when this mob is squashed)
	mob.squashed.connect($UserInterface/ScoreLabel._on_mob_squashed.bind())


func _on_player_hit() -> void:
	$MobTimer.stop()
