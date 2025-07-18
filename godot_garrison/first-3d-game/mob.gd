extends CharacterBody3D

# Minimum speed of the mob [m/s]
@export var min_speed = 10

# Maximum speed of the mob [m/s]
@export var max_speed = 18


func _physics_process(delta: float) -> void:
	move_and_slide()


func initialise(start_pos: Vector3, player_pos: Vector3) -> void:
	# Position mob and rotate it towards player
	look_at_from_position(start_pos, player_pos, Vector3.UP)

	# Randomly offset look direction
	# (so it's not fully directed at the player)
	rotate_y(randf_range(-PI/4, PI/4))

	# Calculate random (integer) speed
	var rand_speed = randi_range(min_speed, max_speed)

	# Calc fwd velocity (speed)
	velocity = Vector3.FORWARD * rand_speed

	# Then rotate it to move in dir the mob is looking
	velocity = velocity.rotated(Vector3.UP, rotation.y)


func _on_visible_on_screen_notifier_3d_screen_exited() -> void:
	queue_free()
