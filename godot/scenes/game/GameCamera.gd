extends Camera2D

# Moveable and zoomable 2D camera
#
# This script implements a 2D camera that can be moved and zoomed independently.
# It is inspired by the camera movement in real-time strategy games, and
# supports the following features:
#
# - Move the camera using keys
# - Move the camera by moving the mouse to the border of the viewport
# - Zoom in and out using keys or the mouse wheel
#
# Various parameters of the camera movement can be configured through external
# variables, making it easy to adjust the camera for different use cases.
#
# The implementation is inspired by the following resources:
#
# - https://github.com/carmel4a/RTS-Camera2D
# - https://www.gdquest.com/tutorial/godot/2d/camera-zoom/

# Controls how fast the camera moves
export (float) var camera_speed := 512.0

# Limits how close a player can zoom in
export (float) var minimum_zoom := 0.5

# Limits how far a player can zoom out
export (float) var maximum_zoom := 2.0

# Controls how fast the camera zooms in and out
export (float) var zoom_speed := 0.1

# Controls the length of the zoom animation
export var zoom_duration := 0.2

# Target zoom level for the camera
var zoom_level := 1.0 setget _set_zoom_level

# Limit the camera around the borders of the map
var camera_limit_left := limit_left
var camera_limit_right := limit_right
var camera_limit_top := limit_top
var camera_limit_bottom := limit_bottom


func _ready() -> void:
	_set_limits()


func _physics_process(delta: float) -> void:
	_move_camera(delta)
	_zoom_camera()
	_set_limits()


func _move_camera(delta: float):
	var velocity = _input_to_velocity()

	var zoomed_camera_speed = camera_speed * zoom_level

	position.x = clamp(
		lerp(position.x, position.x + velocity.x * zoomed_camera_speed, delta),
		camera_limit_left,
		camera_limit_right
	)
	position.y = clamp(
		lerp(position.y, position.y + velocity.y * zoomed_camera_speed, delta),
		camera_limit_top,
		camera_limit_bottom
	)


func _zoom_camera():
	if Input.is_action_pressed("camera_zoom_in"):
		_set_zoom_level(zoom_level - zoom_speed)

	if Input.is_action_pressed("camera_zoom_out"):
		_set_zoom_level(zoom_level + zoom_speed)


func _input_to_velocity() -> Vector2:
	var velocity = Vector2(0.0, 0.0)

	if Input.is_action_pressed("camera_move_left"):
		velocity.x -= 1.0
	if Input.is_action_pressed("camera_move_right"):
		velocity.x += 1.0
	if Input.is_action_pressed("camera_move_top"):
		velocity.y -= 1.0
	if Input.is_action_pressed("camera_move_bottom"):
		velocity.y += 1.0

	return velocity.normalized()


func _set_limits() -> void:
	var viewport = get_viewport().get_visible_rect()

	var viewport_width_offset = viewport.size.x / 2 * zoom_level
	var viewport_height_offset = viewport.size.y / 2 * zoom_level

	camera_limit_left = limit_left + viewport_width_offset
	camera_limit_right = limit_right - viewport_width_offset
	camera_limit_top = limit_top + viewport_height_offset
	camera_limit_bottom = limit_bottom - viewport_height_offset


func _set_zoom_level(value: float) -> void:
	var tween = $Tween

	zoom_level = clamp(value, minimum_zoom, maximum_zoom)

	tween.interpolate_property(
		self,
		"zoom",
		zoom,
		Vector2(zoom_level, zoom_level),
		zoom_duration,
		tween.TRANS_SINE,
		tween.EASE_OUT
	)

	tween.start()
