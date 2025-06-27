extends Area2D

# Player move speed (pixels/sec)
@export var speed = 400

# Dimensions of the game window
var screen_size: Vector2

# ------------------------------------------------------------------------------
# ------------------------------------------------------------------------------

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	screen_size = get_viewport_rect().size


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	# Player movement vector
	var velocity := Vector2.ZERO

	# Check for input
	if Input.is_action_pressed("move_right"):
		velocity.x += 1
	if Input.is_action_pressed("move_left"):
		velocity.x -= 1
	if Input.is_action_pressed("move_down"):
		velocity.y += 1
	if Input.is_action_pressed("move_up"):
		velocity.y -= 1

	# Play corresponding animation if moving
	if velocity.length() > 0:
		velocity = velocity.normalized() * speed
		$AnimatedSprite2D.play()
	else:
		$AnimatedSprite2D.stop()

	# Move in given direction
	# (ensuring player does not go offscreen)
	position += velocity * delta
	position = position.clamp(Vector2.ZERO, screen_size)
