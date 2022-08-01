using StaticArrays
using Memoize
using Bijections

Vector3{T} = SVector{3, T} where T <: Integer
v(x, y, z) = SVector(x, y, z)

function name(vec3::Vector3)
	output = ""
	vec3[1] == 1 && (output *= "R")
	vec3[1] == -1 && (output *= "L")
	vec3[2] == 1 && (output *= "U")
	vec3[2] == -1 && (output *= "D")
	vec3[3] == 1 && (output *= "F")
	vec3[3] == -1 && (output *= "B")
	return output
end

Base.show(io::IO, vec3::Vector3) = print(io, "[$(vec3[1]), $(vec3[2]), $(vec3[3])] ($(name(vec3)))")
Base.show(io::IO, ::MIME"text/plain", vec3::Vector3) = print(io, vec3)

@enum Axis X Y Z

rotation_matrices_dict = Dict(
	X => θ -> [
		1   0       0
		0 cos(θ)  sin(θ)
		0 -sin(θ) cos(θ) 
	],

	Y => θ -> [
		cos(θ)  0 -sin(θ)
		   0    1   0
		sin(θ) 0 cos(θ)
	],

	Z => θ -> [
		cos(θ)   sin(θ) 0
		-sin(θ)  cos(θ) 0
		   0       0    1
	]
)

function rotate(position::Vector3, axis::Axis, θ::Real)::Vector3	
	m = rotation_matrices_dict[axis](θ)
	round.(Int, m * position)
end

@enum Face R U F L D B

const face_letters_dict = Bijection(Dict(
	'R' => R,
	'U' => U,
	'F' => F,
	'L' => L,
	'D' => D,
	'B' => B,
))

function name(face::Face)::Char
	face_letters_dict(face)
end

struct Move
	face::Face
	amount::Integer
end

function name(move::Move)
	face = name(move.face)
	amount = abs(move.amount)
	reverse = move.amount < 0 ? "\'" : ""
	if amount == 1
		amount = ""
	end

	"$face$amount$reverse"
end

Base.show(io::IO, move::Move) = print(io, name(move))

const face_planes_dict = Bijection(Dict{Face, Vector3}(
	R => [1, 0, 0],
	L => [-1, 0, 0],
	U => [0, 1, 0],
	D => [0, -1, 0],
	F => [0, 0, 1],
	B => [0, 0, -1],
))


function Move(input::AbstractString)
	regex = r"([RUFLDB]{1})(\d*)('?)"
	m = match(regex, input)
	if m === nothing
		error("Invalid input for move")
	end

	face, amount, reverse = m.captures
	
	face = face_letters_dict[face[1]]
	amount = length(amount) == 0 ? 1 : parse(Int, amount)
	reverse = length(reverse) == 0 ? 1 : -1

	Move(face, amount * reverse)
end

function parsealg(input::AbstractString)::Vector{Move}
	[Move(i) for i in split(rstrip(input), ' ')]
end

Base.show(io::IO, alg::Vector{Move}) = print(io, join(name.(alg), " "))
Base.show(io::IO, ::MIME"text/plain", alg::Vector{Move}) = print(io, join(name.(alg), " "))

function movedata(move::Move)
	axis = if move.face ∈ [R, L]
		X
	elseif move.face ∈ [U, D]
		Y
	else
		Z
	end

	angle = move.amount * π/2 * ((move.face ∈ [R, U, F]) ? 1 : -1)

	(angle, axis)
end

# Uses one 32 byte allocation
@memoize IdDict function move(position::Vector3, input::Move)::Vector3
	angle, axis = movedata(input)
	rotate(position, axis, angle)
end

struct Piece
	id::Vector3
	position::Vector3
	normal::Vector3
end

function Base.show(io::IO, piece::Piece)
	p = piece.position == piece.id ? "solved" : "at $(piece.position)"
	print(io, "Piece $(piece.id) $p with normal $(piece.normal)")
end

function Piece(id::Vector3, position::Vector3)::Piece
	i = findfirst(n -> abs(n) == 1, position)
	normal = [0, 0, 0]
	normal[i] = position[i]
	Piece(id, position, normal)
end

Piece(id::Vector3)::Piece = Piece(id, id)

Piece(x::Integer, y::Integer, z::Integer)::Piece = Piece(@SVector[x, y, z])

function Piece(id::Vector3, position::Vector3, orientation::Integer)
	mask = position .!= 0
	modulus = count(mask)
	parity = reduce(*, position[mask])
	orientation = mod(orientation * parity, modulus) + 1
	
	i = collect(1:3)[mask][orientation]
	normal = [0, 0, 0]
	normal[i] = position[mask][orientation]

	Piece(id, position, normal)
end

function makeedges()
	positions = []
	for (i, j) in [(1, 1), (1, -1), (-1, -1), (-1, 1)]
		positions = [positions..., [i, j, 0], [i, 0, j], [0, i, j]]
	end
	
	map(positions) do pos
		Piece(pos...)
	end
end

function makecorners()
	pieces = []
	for i in [1, -1]
		for j in [1, -1]
			for k in [1, -1]
				push!(pieces, Piece(i, j, k))
			end
		end
	end
	pieces
end

struct Cube{N}
	pieces::SVector{N, Piece}
end

FullCube = Cube{20}
Edges = Cube{12}
HalfEdges = Cube{6}
Corners = Cube{8}

const solved_cube = let
	c = makecorners()
	e = makeedges()
	Cube{20}([c; e])
end

Cube() = solved_cube
Cube{20}() = solved_cube
Cube{12}(cube::FullCube = solved_cube) = Edges(cube.pieces[9:20])
Cube{6}(cube::FullCube = solved_cube) = HalfEdges(cube.pieces[9:14])
Cube{8}(cube::FullCube = solved_cube) = Corners(cube.pieces[1:8])

Base.:(==)(a::Cube, b::Cube) = a.pieces == b.pieces

Base.show(io::IO, cube::Cube) = print(io, "$(length(cube.pieces)) length cube")
function Base.show(io::IO, ::MIME"text/plain", cube::Cube)
	print(io, cube)
	print(io, ":\n")

	for piece in cube.pieces
		print(io, "   ")
		println(io, piece)
	end
end

function isinrange(position::Vector3, plane::Vector3)::Bool
	i = findfirst(x -> x != 0, plane)
	position[i] == plane[i]
end

function move(cube::Cube{N}, input::Move)::Cube{N} where N
	map(cube.pieces) do piece
		if isinrange(piece.position, face_planes_dict[input.face])
			Piece(piece.id, move(piece.position, input), move(piece.normal, input))	
		else
			piece
		end
	end |> Cube
end

function move(cube::Cube{N}, alg::Vector{Move})::Cube{N} where N
	for input in alg
		cube = move(cube, input)
	end
	cube
end

function move(cube::Cube{N}, alg::String)::Cube{N} where N
	move(cube, parsealg(alg))
end

function issolved(cube::Cube{N}) where N
	cube == Cube{N}()
end

function issolved_thorough(cube::Cube{N}) where N
	map(cube.pieces) do piece
		piece.id == piece.position &&
		findfirst(n -> n != 0, piece.normal) == findfirst(n -> n != 0, piece.position)
	end |> all
end

const possible_moves = let
	m::Vector{Move} = []
	for face in instances(Face)
		for i in [2, -1, 1]
			m = [m..., Move(face, i)]
		end
	end
	SVector{18}(m)
end

function neighbours(cube::Cube{N})::SVector{18, Cube{N}} where N
	map(m -> move(cube, m), possible_moves)
end