extends Node

@export var mob_scene: PackedScene


func _ready() -> void:
	$UserInterface/Retry.hide()


func _unhandled_input(event) -> void:
	if event.is_action_pressed("ui_accept") and $UserInterface/Retry.visible:
		# Restart current scene
		get_tree().reload_current_scene()


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
	$UserInterface/Retry.show()
