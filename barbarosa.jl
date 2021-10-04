### A Pluto.jl notebook ###
# v0.16.1

using Markdown
using InteractiveUtils

# This Pluto notebook uses @bind for interactivity. When running this notebook outside of Pluto, the following 'mock version' of @bind gives bound variables a default value (instead of an error).
macro bind(def, element)
    quote
        local el = $(esc(element))
        global $(esc(def)) = Core.applicable(Base.get, el) ? Base.get(el) : missing
        el
    end
end

# ╔═╡ 13773eb4-bd8b-4a24-8bed-661ad60b9101
using Random

# ╔═╡ 0ce2b455-047d-43c0-b90c-2a0cb8c66ec6
using Plots

# ╔═╡ b2d6301f-f41c-42f8-a03b-68b443701c52
using PlutoUI

# ╔═╡ 2d3eb219-6234-44c5-9dbb-bc2b47e99958
md"""
# Solving the cube
We want to solve the Rubik's cube using IDA* with an AI based heuristic. This problem seems perfect fitted for Julia. 

A functional programming approach will be used throughout the notebook. 
"""

# ╔═╡ e05e9488-c991-40ee-890d-63f0d04dbe00
md"""
## Building the cube
The first order of buisness is building a digital cube to manipulate
"""

# ╔═╡ 86573f98-6657-444a-8512-555b2305dbf9
md"""
### Defining the axes and planes
Firstly we have to define some notation. Namely, "R" means right; "U", up; "F", front; "L", left; "D", down and "B", back. 

As for the direction, the "R" side will be the x-axis, the "U" side will be the y-axis, and the "F" side will be the z-axis. This is chosen based on the frequency of each move. 

We will create a `vectorofface` function that returns a (normalized) vector that indicates the axis in 3D space for each face.

"""

# ╔═╡ e06f3ff2-7b70-4af9-b5a1-29d6a95d9416
function vectorofface(face::Char)
	face == 'R' ? [1,  0,  0] : 
	face == 'U' ? [0,  1,  0] : 
	face == 'F' ? [0,  0,  1] : 
	face == 'L' ? [-1, 0,  0] : 
	face == 'D' ? [0, -1,  0] : 
	face == 'B' ? [0,  0, -1] : 
	error("Face $face doesn't exist")
end

# ╔═╡ f22871f6-03e5-4146-a24b-0b19de53cef3
md"""
It will also be useful to get a plane for each face. A plane, in this aplication, only needs an axis (1, 2 or 3 for x, y or z) and a direction (1 or -1). 
"""

# ╔═╡ a6ae98d4-a7c4-41b5-aceb-b07cbd9c5ee0
struct Plane
	axis::Int
	direction::Int
end

# ╔═╡ b6b5d4df-ca08-4a4c-8cd7-79ba8e471dee
function planeofface(face::Char)
	face == 'R' ? Plane(1, 1) : 
	face == 'L' ? Plane(1, -1) : 
	face == 'U' ? Plane(2, 1) : 
	face == 'D' ? Plane(2, -1) : 
	face == 'F' ? Plane(3, 1) : 
	face == 'B' ? Plane(3, -1) : 
	error("Face $face doesn't exist")
end

# ╔═╡ 59f39125-dbe6-4246-b729-d26a48ab967c
planeofface('R')

# ╔═╡ 3119ebc3-6281-41c0-834b-e9ee925908b8
md"""
We will also create a function to retrieve the name of the planes
"""

# ╔═╡ bf24b380-366e-415f-a665-6721d83ec87b
function name(vector::Vector{Int})
	name = ""
	vector[1] == 1 && (name *= "R")
	vector[1] == -1 && (name *= "L")
	vector[2] == 1 && (name *= "U")
	vector[2] == -1 && (name *= "D")
	vector[3] == 1 && (name *= "F")
	vector[3] == -1 && (name *= "B")
	name
end

# ╔═╡ 0d089088-2389-4121-bea9-50a8e2a0d3aa
function name(plane::Plane)
	axis, direction = plane.axis, plane.direction
	if axis == 1 
		direction == 1 && (name = "R")
		direction == -1 && (name = "L")
	elseif axis == 2
		direction == 1 && (name = "U")
		direction == -1 && (name = "D")
	elseif axis == 3
		direction == 1 && (name = "F")
		direction == -1 && (name = "B")
	end
	name
end

# ╔═╡ 8eff452c-e23e-4c61-ba0f-e43d1a922ded
function name(array::Vector{<:Any})
	name.(array)
end

# ╔═╡ f23d399c-b6b9-4f71-9608-e7975a6aa497
md"""
### Creating the pieces
The pieces will be defined as structs. 

Each piece has a position where it belongs and the current position is in, if the cube is scrambled. These will be called `solved_position` and `position`, respectively. 

Also every piece has an orientation. While it is technically possible to represent orientation with vectors (and could even be considered more idiomatic), seeing as an edge only has two orientations and a corner three, we can have lighter definitions. 

An edge is considered to be correctly oriented if the R, U, L or D part of any given edge is facing any of these faces. This concept is known as EO (edge orientation) and is used in some speedsolving methods like ZZ. This is especially useful since only F and B moves modify edge orientation

A corner's orientation is defined according to how many clockwise turns it needs to get to the correct orientation. Usually, a corner is considered to be correctly oriented if the U or D part of the piece is facing said faces. However, it will be easier to change the U and D faces with the F and B faces, since the selected faces have a different behaviour when moving the cube, and this way it is consistent with the edges. 

A correctly oriented piece will be asigned a `0` in the `orientation` field. A `solved_orientation` field would, then, be reduntant, since we know that it is solved if it is the orientation is `0`.
"""

# ╔═╡ a28bafb6-b95a-48f3-b8a3-5cb6894977b2
struct Piece
	solved_position::Vector{Int}
	position::Vector{Int}
	orientation::Int8
end

# ╔═╡ 5d7aaac0-dc85-4891-b78d-7fbf5a5c87b5
md"""
However, it is awkard to construct pieces with vectors. It is much more natural to use the name of the faces where a piece in. For example, in a standard cube, the green-white edge would be represented by the vector `[0, 1, 1]`, while it would be much nicer to call it "UF". 

For this reason, we will adjust the constructor so that it creates pieces this way, as well as with vectors. 

This function has no logic to ensure the pieces created make sense. 
"""

# ╔═╡ 1a76e513-c4ee-4333-aee8-c0c95d7bcf9b
function constructpiece(vector::Vector{<:Int}, orientation = 0, position = nothing)
	position == nothing && (position = vector)
	Piece(vector, position, orientation)
end

# ╔═╡ 47a4e5ae-d72a-4823-8579-f47653b28a3a
function vectorofpiece(name::String)
	sum(v -> vectorofface(v), name)
end

# ╔═╡ f9a24873-4e03-4648-b966-e9aceb8b4d58
function constructpiece(name::String, orientation = 0, position = nothing)
	constructpiece(vectorofpiece(name), orientation, position)
end

# ╔═╡ f84832a1-1fe9-4df7-a813-a4be7a079d83
constructpiece("FD")

# ╔═╡ 7b02de6a-20b2-411a-b374-aa461a527109
md"""
A function for retrieving the name of the piece can also be useful, mainly for debugging. 
"""

# ╔═╡ 6f0c7aa8-392d-42c9-9638-55ef53e9aa25
name(piece::Piece) = name(piece.solved_position)

# ╔═╡ 5045dcc9-de20-4d6e-9a60-31b633f61d11
md"""
### Assembling the cube

A generic cube will be defined as a struct, with arrays for edges and corners. 
"""

# ╔═╡ fe09f584-9be3-42bc-bc61-b4e9cc1dd700
Cube = Vector{Piece}

# ╔═╡ 9adab1f2-fa93-42c1-a00e-06258b17dd44
md"""
We will mix piece types, since they behave mostly in the same way. However, we will create a function to get only the desired pieces. 
"""

# ╔═╡ 6ee79fca-e240-446a-8c66-53820ee965c2
md"""
A cube has 12 edges and 8 corners. This number is small enough to make it reasonable to manualy build the solved cube. 
"""

# ╔═╡ 41600c68-045e-4443-a187-601ff9186877
const edges = [
	"RU", "RF", "RD", "RB", 
	"UF", "DF", "DB", "UB", 
	"LU", "LF", "LD", "LB"
]

# ╔═╡ 4be974ef-436a-441b-831c-5a0f62becb4a
const corners = [
	"RUF", "LUF", "RDF", "LDF", 
	"RUB", "LUB", "RDB", "LDB"
]

# ╔═╡ eb1d7422-adce-42d6-8319-947598e46682
const pieces = [edges; corners]

# ╔═╡ a757bb4e-c82a-4459-a967-33260ad902c0
const solved_cube = Cube(
	constructpiece.(pieces)
)

# ╔═╡ 7930b143-d396-467d-81df-eb33337006db
md"""
### Making moves

The available moves will only be the face moves. Slice moves and wide moves like "M" or "r" can be changed to "R L'" or "L", but this will not be necessary as we will not be using these moves.

Each move changes the position and the orientation of a specific set of pieces. We will tackle three different problems.
"""

# ╔═╡ 3248b006-35de-46ec-878b-deb8448a68fc
md"""
#### Get pieces

We want to get the pieces in the plane of rotation. We could use somewhat sofisticated linear algebra to construct a plane and check the piece is inside it, but this is too convoluted. Instead, we will simply check the coordinates of each piece. 
"""

# ╔═╡ ce7f6993-c1fa-4064-b18f-92fcbfe63387
function pieceinplane(piece::Piece, plane::Plane)
	piece.position[plane.axis] == plane.direction
end

# ╔═╡ 856c1135-282e-48b4-afe2-ebbe92650d25
function pieceinplane(piece::Piece, face::Char)
	plane = planeofface(face)
	pieceinplane(piece, plane)
end

# ╔═╡ 243bc6f0-814a-4f73-a414-86864a217b43
md"""
#### Move pieces

Moving the pieces only requires a 90 or 180 degree rotation of the tangential components of the position vector of a piece. This can be easily done with some rudimentary roation matrices. 

"""

# ╔═╡ 384c32a9-fc0c-4563-8fc0-8eedf0ac0f20
function rotate(vector::Vector{Int}, axis, amount)
	vector = deepcopy(vector)
	i = (axis % 3 + 1, (axis + 1) % 3 + 1)
	if amount == 2
		vector[i[1]], vector[i[2]] = -vector[i[1]], -vector[i[2]]
	elseif amount == 1
		vector[i[1]], vector[i[2]] = vector[i[2]], -vector[i[1]]
	elseif amount == -1
		vector[i[1]], vector[i[2]] = -vector[i[2]], vector[i[1]]
	end
	vector
end

# ╔═╡ fda5ad80-b79f-4464-b35d-fe6a29bca77d
# Piece type input for rotating
function rotate(piece::Piece, axis, amount) 
	vector = rotate(piece.position, axis, amount)
	constructpiece(vector, piece.orientation)
end

# ╔═╡ 7cbcc253-69d3-40b9-a559-311a8bf98a12
rotate([1, 1, 0], 1, 1)

# ╔═╡ 3f5b9fc4-abfa-4d14-b846-1becb0a9bbbb
md"""
#### Orient pieces

Piece orientation is very different for edges and corners

For edges, F and B moves (or z-axis moves) toggle orientation, and that's it.

For corners, F and B moves don't change orientation. Other moves increase by two the even corners and by one the odd ones. An even corner is one where the sign of the product of their coordinates is positive. This is easy enough to see by trying it out. 
"""

# ╔═╡ 410ac675-a7e8-484a-93fb-bf6bbb3115e6
md"""
Since this is the first time we are going to distinguish between edges and corners, a function will be defined that returns the type of piece. 
"""

# ╔═╡ 638a1ca2-850b-429d-9d76-4823a0f7b0bd
function piecetype(piece::Piece)
	sum = reduce(+, abs.(piece.solved_position))
	if sum == 1
		"Center"
	elseif sum == 2
		"Edge"
	elseif sum == 3
		"Corner"
	else
		error("Uknown piece type")
	end
end

# ╔═╡ 80e6e702-3766-41f4-a303-d224e2818752
getpieces(cube::Cube, piece_type::String) = filter(v -> piecetype(v) == piece_type, cube)

# ╔═╡ b4a3bf53-de29-4890-a6f2-3746a6022265
md"""
Since the rotation logic is pretty crucial for piece orientation, we'll include the latter logic directly in a `movepiece` function because it would be impractical to make separate functions for this.
"""

# ╔═╡ 51fc06f2-cdfb-4951-b6c4-0a59c6bc2608
md"""
#### Putting the move logic together

Having done the previous work, this step is (should be) fairly straightfoward.
"""

# ╔═╡ 325c6dcb-b38b-4fb5-93ef-4ae934a5b963
struct Move
	plane::Plane
	amount::Int
end

# ╔═╡ 0e4f52b0-9831-488f-af79-5bd60c999f1d
function movepiece(piece::Piece, move::Move)
	axis, direction, amount = move.plane.axis, move.plane.direction, move.amount
	move_position = rotate(piece.position, axis, amount)
	
	if piecetype(piece) == "Edge" && axis == 3 && amount ≠ 2
		orientation = (piece.orientation + 1) % 2
	else
		orientation = piece.orientation
	end
	
	if axis ≠ 3 && amount ≠ 2 && piecetype(piece) == "Corner" 
		corner_parity = reduce(*, piece.position)
		offset = 1
		if corner_parity == -1
			offset = 2
		end
		orientation = (piece.orientation + offset) % 3
			
	end
	constructpiece(piece.solved_position, orientation, move_position)
end

# ╔═╡ ccc74b2d-1c83-44df-99bf-745aed13cc71
function move(cube::Cube, move::Move)
	moved_cube = map(
	piece -> pieceinplane(piece, move.plane) 
		? movepiece(piece, move) 
		: piece = piece, 
	cube)
	
	Cube(moved_cube)
end

# ╔═╡ 045dc3e9-e1e8-4aa2-b3aa-d906d88dccee
md"""
It is also very helpful to be able to input moves with letter notation (like "R2" and "B'"). 
"""

# ╔═╡ 19b07f28-d7a5-41a0-b9c4-2cde0c575b26
function getmove(move_name::String)
	plane = planeofface(move_name[1])
	amount = 1
	if length(move_name) == 2
		move_name[2] == '2' && (amount = 2)
		move_name[2] == '\'' && (amount = -1)
	end
	Move(plane, amount)
end

# ╔═╡ 24d516a6-cfe4-493f-978b-839c1814e836
movepiece(constructpiece("RUF"), getmove("R"))

# ╔═╡ e8b65eba-4b79-4aac-a614-122f469925fe
md"""
And also to be able to get the name of the move from the numeric representation
"""

# ╔═╡ 8e24fffa-b9fb-4d82-9865-63807edf49cf
function name(move::Move)
	face_name = name(move.plane)
	if move.amount == 1
		amount_name = ""
	elseif move.amount == 2
		amount_name = '2'
	elseif move.amount == -1
		amount_name = '\''
	end
	face_name * amount_name
end

# ╔═╡ 44787f95-0fc3-4afd-b682-0b8eb8e66d4b
name([1, -1, 1]), name(constructpiece("RUB"))

# ╔═╡ 3942e7d2-1a0c-438b-ada6-c245a67ccfa5
name.(filter(piece -> pieceinplane(piece, 'B'), solved_cube))

# ╔═╡ 3b9ffb15-86ec-4ca2-9e04-0c8eadc61c54
name(rotate(constructpiece("RU"), 1, 1))

# ╔═╡ 6dd34804-f815-4fa4-995f-d6b8cf2a4ce1
function move(cube::Cube, moves::String)
	move(cube, getmove.(convert.(String, split(moves))))
end

# ╔═╡ 66ef63eb-4362-439f-9197-9662319c5980
name(Move(Plane(1, 1), 1))

# ╔═╡ 22e4a4f6-17f3-40f3-98e8-432e646cc73d
md"""
We also want to be able to make moves as algorithms, so a function to do that will also defined. 
"""

# ╔═╡ 811e9bac-4f50-40b2-9ac0-5a0e46b65777
function move(cube::Cube, moves)
	makemove = move
	for move in moves
		cube = makemove(cube, move)
	end
	cube
end

# ╔═╡ bacb17e5-2fff-49fe-a27f-d35d67d59716
map(v -> v.orientation, move(solved_cube, ["L'", "L"]))

# ╔═╡ 229d2567-25e7-4c5c-8f60-ba00404eaa4d
move(solved_cube, Move(planeofface('R'), 1))

# ╔═╡ 8b95c08c-46b9-4be7-bf14-75ef5dd09f8d
move(solved_cube, "R2 D2")

# ╔═╡ 04569bda-1b60-4c71-b7ca-54eba379264b
const sexy_move = "R U R' U'"

# ╔═╡ aab1febf-b210-4b1e-89a5-141a84355835
const tperm = "R U R' U' R' F R2 U' R' U' R U R' F'"

# ╔═╡ 5e66352f-3989-4807-b295-a0cf31aae2d0
const sexy_cube = move(solved_cube, sexy_move)

# ╔═╡ 5b7c7056-b147-4d39-99b6-46d080c746c1
md"""
### Checking if the cube is solved

This is necessary, but kind of trivial. 
"""

# ╔═╡ 02ec5f2a-ece5-4860-aa02-470e129335fc
function issolved(piece::Piece)
	piece.position == piece.solved_position && piece.orientation == 0
end

# ╔═╡ 21f5b811-76d1-441d-bb29-ee7a4aa3dc0e
function issolved(cube::Cube)
	all(issolved.(cube))
end

# ╔═╡ 6fe5470e-4c34-4fc2-97d1-a6c897253019
issolved(move(solved_cube, repeat(sexy_move, 6)))

# ╔═╡ 842f3cdb-a66e-450a-b579-bd3bd3948210
issolved(movepiece(constructpiece("UFR"), getmove("R")))

# ╔═╡ caae487b-7942-452f-81c4-a290d8205271
issolved(solved_cube), issolved(move(solved_cube, "R"))

# ╔═╡ 430426a8-7ecd-4164-b1f5-917acef06584
md"""
What is less straightfoward is the solvability check for the cube. It is not strictly necessary, but useful to have nonetheless. 

The solvability check here is done first by checking if the sum of the orientations is divisible by 3 with the corners and by 2 with the edges. The positions are checked by the parity of swaps that would be needed to get every piece in the correct position. If the parity of corners is the same as of the edges, the cube will be solvable. This concept is widely know from blindfolded solving. 
"""

# ╔═╡ e5e2b105-fb37-4e5d-8727-2617c979ff3f
function parity(pieces::Vector{Piece})
	searched = []
	search = 1
	count = 0
	
	while length(searched) < length(pieces)
		
		# Find where piece goes
		goes = findfirst(map(
				v -> v.position == pieces[search].solved_position, 
				pieces)
		)
		push!(searched, search)
		
		if goes ∈ searched 
			# Find first unsearched index
			for i in 1:length(pieces)
				if i ∉ searched
					search = i
					break
				end
			end
		else
			count += 1
			search = goes
		end

	end
	
	count
end

# ╔═╡ d18076fa-2376-4f85-bfb3-4062f01176ac
function issolvable(cube::Cube; verbose=false)
	# cube = deepcopy(cube)
	
	# Filter by piecetype
	corners = getpieces(cube, "Corner")
	edges = getpieces(cube, "Edge")
	
	# Check orientation
	if sum(map(v -> v.orientation, corners)) % 3 ≠ 0
		return !verbose ? false : (false, "bad corner orientation")
	elseif sum(map(v -> v.orientation, edges)) % 2 ≠ 0
		return !verbose ? false : (false, "bad edge orientation")
	end
	
	# Check if cube has all pieces
	if Set(cube) ≠ Set(solved_cube)
		return !verbose ? true : (true, "isn't full cube")
	end
	
	# Check parity
	corner_parity = parity(corners)
	edge_parity = parity(edges)
	
	if iseven(edge_parity) ≠ iseven(corner_parity)
		return !verbose ? false : (false, "parity")
	end
	
	return !verbose ? true : (true, "no problems")
end

# ╔═╡ 3cf0df23-ee1e-4321-9581-91c6a6f97cc8
let
	cube = sexy_cube
	cube = move(sexy_cube, ["R", "U2", "R'", "L'", "D2", "F'", "B2"])
	# cube = move(solved_cube, ["L'"])
	parity(getpieces(cube, "Corner")), parity(getpieces(cube, "Edge"))
end

# ╔═╡ 9227bdd6-0fc0-4ea1-82b3-07c20d1b7b85
md"""
### Random state cube

Finally, we also have to be able to assemble a random state cube for our algorithm to solve later on.  
"""

# ╔═╡ cc524387-9048-4a3f-8200-a14550927bc3
function randomcube(; seed = 69420)
	
	positions = [shuffle(vectorofpiece.(edges)); shuffle(vectorofpiece.(corners))]
	
	cube = [constructpiece(piece, 0, position) 
		for (piece, position) in zip(pieces, positions)]

	# Swap two pieces (of the same type bc of construction) if cube is unsolvable
	# Then, reconstruct it
	if !issolvable(cube)
		positions[1], positions[2] = positions[2], positions[1]
		
		cube = [constructpiece(piece, 0, position) 
			for (piece, position) in zip(pieces, positions)]
	end

	if !issolvable(cube)
		error("Cube is still unsolvable ($(issolvable(cube, verbose=true)[2]))")
	end

	cube
end

# ╔═╡ 1353a3d7-4d9d-435a-a73a-d31a00a4eb89
issolvable(randomcube())

# ╔═╡ b062133e-6a7e-4df8-9415-828bf58572a9
cube = randomcube()

# ╔═╡ 9a51a1c7-1421-454b-84f8-812e45aa98b2
md"""
We now have a fully functional digital cube.
"""

# ╔═╡ 3455aff2-403d-468b-b378-f67fe50acce7
md"""
### Visualizing the cube

It will be of immense help being able to visualise the cube. We will do this by using the `Plots` package and the `plotly` backend. 
"""

# ╔═╡ e7c52b1c-cc1a-4652-87c7-77845c2be361
plotly()

# ╔═╡ 2235a441-fc02-4ddd-8649-3a79fa848a2c
md"""
First we will name a `Coordinate` type, for convinience
"""

# ╔═╡ 098daed8-7519-4b77-a629-39d49891b1ba
Coordinate = Tuple{Number, Number, Number}

# ╔═╡ c2174f2b-97bc-4040-9333-852d38f71d07
md"""
Then we will create a `drawpoints` function, since `Plots` is plot oriented but we want to use simple coordinates. 
"""

# ╔═╡ d86a8b70-48bc-41a7-b9f7-dae0ed1ba4c6
function drawpoints!(canvas, points; plot_style...)
	x = map(v -> v[1], points)
	y = map(v -> v[2], points)
	z = map(v -> v[3], points)
	x, y, z
	
	n = length(points)
	i = []
	j = []
	k = []
	for index in 0:(n - 2)
		i = [i..., index % n]
		j = [j..., (index + 1) % n]
		k = [k..., (index + 2) % n]
	end
	
	mesh3d!(canvas, x, y, z; connections = (i, j, k), plot_style...)
end

# ╔═╡ c22dcc99-a282-4625-8507-b64bc7749bc0
drawpoints(points; plot_style...) = drawpoints!(plot(), points; plot_style...)

# ╔═╡ a74aaff2-1e72-4bd4-9051-9644a4a5c3a7
md"""
We will use `drawpoints` to create a `drawquad` function that creates squares out of a center, an axis and a size. 
"""

# ╔═╡ f35291b4-e439-466d-85e2-a64287d011a1
function drawquad!(canvas, center; axis::Integer, size=1, plot_style...)
	if axis ∉ [1, 2, 3]
		throw("Invalid axis ($axis)")
	end
	
	if typeof(center) <: AbstractArray
		center = tuple(center...)
	end
	
	size = size/2
	
	if axis == 1
		a = center .+ (0, size, size)
		b = center .+ (0, size, -size)
		c = center .+ (0, -size, -size)
		d = center .+ (0, -size, size)
	elseif axis == 2
		a = center .+ (size, 0, size)
		b = center .+ (-size, 0, size)
		c = center .+ (-size, 0, -size)
		d = center .+ (size, 0, -size)
	elseif axis == 3
		a = center .+ (size, size, 0)
		b = center .+ (size, -size, 0)
		c = center .+ (-size, -size, 0)
		d = center .+ (-size, size, 0)
	end
	
	drawpoints!(canvas, [a, b, c, d]; plot_style...)
end

# ╔═╡ 529b2dc1-a2b9-40f2-a8c8-a90a39114bff
drawquad(center; axis::Integer, size=1, plot_style...) = drawquad!(plot(), center; axis, size=1, plot_style...)

# ╔═╡ e268a2df-8a90-453c-aec8-c44ef64c4c86
let
	a = (0, 0, 0)
	b = (0, 1, 0)
	c = (0, 1, 1)
	d = (0, 0, 1)
	
	canvas = plot()
	
	drawpoints!(canvas, [a, b, c, d]; color="#005f90")
	drawquad!(canvas, a, axis=2; color="#ff0000")
	
end

# ╔═╡ 3c82cf5a-864b-46c0-8843-f4b086f55953
md"""
We will use this to create a `drawpiece` function that takes pieces as arguments. We will also need a color scheme to color the pieces accordingly. 
"""

# ╔═╡ 15e13928-5bb2-47a8-9aca-941b2689a07b
color_scheme = [
	(planeofface('R'), "#ff0000"),
	(planeofface('U'), "#ffffff"),
	(planeofface('F'), "#00ff00"),
	(planeofface('L'), "#FFbc40"),
	(planeofface('D'), "#FFFF00"),
	(planeofface('B'), "#0000ff"),
]

# ╔═╡ b55c8950-bab4-4c43-bd21-2524ef64b465
function getfacecolor(plane::Plane, color_scheme = color_scheme) 
	color_scheme[findfirst(v -> v[1] == plane, color_scheme)][2]
end

# ╔═╡ c7d38b39-401a-4997-a266-767dfcbe6895
md"""
R face color is $(getfacecolor(planeofface('R')))
"""

# ╔═╡ dcfb75d5-44ab-470d-af0f-9717baa04f8d
function colorvector(piece::Piece)
	available_directions = [1, 2, 3]
	# while length(available_directions) > 0
		
	
	if piecetype(piece) == "Corner"
		orientation_plane = Plane(3, piece.solved_position[3])
	end
end

# ╔═╡ a8fdefaf-5d1d-4304-9b8c-0811a9b037a7
name(colorvector(constructpiece("LDB")))

# ╔═╡ 2e01f4b4-bb6a-4a6c-a9fa-620f3b1ea00d
function drawpiece!(canvas, piece::Piece; color_scheme = color_scheme, plot_style...)
	pos = piece.position
	
	for (i, coord) in enumerate(piece.position)
		if coord ≠ 0
			offset = [0.0, 0.0, 0.0]
			offset[i] = 0.5 * sign(coord)

			# ci = (i + piece.orientation) % sum(sign.(piece.solved_position)) + 1
			ci = i
			plane = Plane(ci, sign(piece.solved_position[ci]))
			i == 3 && throw(plane)
			color = getfacecolor(plane)

			drawquad!(canvas, pos .+ offset; axis=i, color=color, plot_style...)
		end
	end
	
	canvas
end

# ╔═╡ abed85c9-ccb0-403c-b911-111db23ab12f
let
	canvas = plot()
	# drawpiece!(canvas, solved_cube[1])
	# drawpiece!(canvas, moved_piece)
	moved_piece = movepiece(solved_cube[1], getmove("R"))
end

# ╔═╡ 565b4874-75ea-4b3c-af0b-5135c260f87e
function drawpiece(piece::Piece; color_scheme = color_scheme, plot_style...)
	drawpiece!(plot(), piece; color_scheme, plot_style...)
end

# ╔═╡ 23591091-95ec-4059-8760-520a1d02cfa7
md"""
Piece index: $(@bind index Slider(1:20))
"""

# ╔═╡ 3fff9fb7-4e51-4899-9580-37fc7e7e5f47
index

# ╔═╡ 0ea27284-bb3f-497e-8038-9a58973e2f11
drawpiece(solved_cube[index]; limits=[-1.5, 1.5])

# ╔═╡ d80132e2-dece-465b-a99f-39c9d1ce069e
md"""
Finally, we can draw the whole cube. 
"""

# ╔═╡ 61119ae6-07a7-4a84-9ca3-9839907ed8e2
function draw(cube::Cube; color_scheme=color_scheme, plot_style...)
	canvas = plot()
	
	for piece in solved_cube
		drawpiece!(canvas, piece; color_scheme=color_scheme, plot_style...)
	end
	
	canvas
end

# ╔═╡ 0b21d9a5-5f10-4f4d-bc19-c622f6908567
draw(move(solved_cube, "B2 U2"); legend=nothing)

# ╔═╡ 4a4b158f-8afa-4164-a238-bde8cae5dfff
md"""
#### I officially give up on cubevis, for now
"""

# ╔═╡ 9d2d5937-c2c0-4b95-83d4-d3a459d9e48e
md"""
# Algorithm search

The algorithm used will be IDA\* (Iterative deepening A\*), since it is impossible to store all the branches as per usual A\*. 

The A\* and IDA\* algorithms both are based on a heuristic. In our application, the heuristic should approximate the number of moves left to solve the cube. This will be done in a rudimentary, old-fashioned way first. Then, we will optimize it by training a neural network to get the heuristic from any given scrambled state. 

"""

# ╔═╡ d7cd03ed-f4e4-415d-8520-41ab9c427270
md"""
## Avoidance of redundant moves
Something to consider is that some moves are redundant if you have made another move previously. Namely, two moves of the same face can always be compressed to one (R R2 = R'). If a move of the opposite face is interpolated, it is also redunant (R U R = R2 U). Finally, the order of opposite face moves doesn't matter (R2 U = U R2). We don't want to search these moves since they will (or should) never be part of the solution we are looking for.

It is easy to incorporate logic to not return same face moves. However, it is increasingly difficult to hande opposite face logic. A smarter approach is not to allow opposite face moves. Instead, for each axis do all the possible combination of moves. 

For example, after the move R, the y-axis (U) would be chosen. Then, the available moves for that axis would be U, U2, U', D, D2, D', and all combinations between these U and D moves. The only problem is that with this approach the cost of the path is not always the same (but it is still the easier and faster approach). 
"""

# ╔═╡ 210f9363-5fcb-44d8-8198-b88632d0829e
md"""
We first implement the function with the axes as the arguments.
"""

# ╔═╡ 1e62b8fa-35df-4593-b353-afc707cd877d
function getmoves(axes::Vector{Int})
	amounts = [1, 2, -1]
	moves = []
	
	# Allocate size for 30 combinations of moves, to improve performance
	sizehint!(moves, 30)
	
	for axis in axes
		# Create the main single, double and reverse moves for each side
		primaries = map(amount -> Move(Plane(axis, 1), amount), amounts)
		secondaries = map(amount -> Move(Plane(axis, -1), amount), amounts)

		append!(moves, map(v -> (v, 1), primaries), map(v -> (v, 1), secondaries))
		
		#Create all the combinations and push them
		for i in primaries
			for j in secondaries
				push!(moves, ([i, j], 2))
			end
		end
	end
	
	moves
end

# ╔═╡ 10314b6d-30c0-49e2-a2b2-d7fdeb16a7a2
md"""
Then the more easy to use alternatives. 
"""

# ╔═╡ bdadcf12-efa9-4f40-ac27-c83917b65ddf
getmoves() = getmoves([1, 2, 3])

# ╔═╡ a2a846ec-3fb2-4211-9c70-30a094d4959c
function getmoves(previous::Move)
	axes = [1, 2, 3]
	if !ismissing(previous)
		axis = previous.plane.axis
		axes = deleteat!(axes, axis)
	end
	
	getmoves(axes)
end

# ╔═╡ 05388f6c-a6e5-4cab-9bd2-8d7020a259df
function getmoves(solution::Vector{<:Any})
	if length(solution) == 0
		getmoves()
	else
		getmoves(solution[1])
	end
end

# ╔═╡ 4bc529a0-71bd-4670-bc28-f47b50d6ddfd
getmoves(nothing::Nothing) = getmoves()

# ╔═╡ ae71c271-0c27-47a7-bc49-0b71c903106d
"All moves are, ", map(v -> name(v[1]), getmoves())

# ╔═╡ ced36f40-c119-4967-828f-f40a4a48fd1d
map(move -> name(move[1]), getmoves(getmove("D2")))

# ╔═╡ c6aea8d4-4200-46a6-ad8d-94f4f647a150
md"""
## IDA* algorithm implementation

IDA* is a depth-first search algorithm that recalculates the entire tree at each iteration. It uses a heuristic, $h$, to calculate the cost of a path. 
The starting threshold is the current heuristic for the particular state to be solved. 
The cost of a path is the number of moves to get to a node plus the heuristic for that node. So, $c = g + h$. 

If the tree is searched and exahusted given a certain threshold, then the tree is recomputed with the new threshold being the smallest cost that passed the threshold in the previous search. This is repeated until a solution is found. 
"""

# ╔═╡ 18b8f949-3c34-4648-83ed-c21b6c9bad67
function search(cube, g, threshold, solution, new_threshold, heuristic = _ -> 0)
	
	# Check if solved
	if issolved(cube)
		return solution, 69420
	end
	
	# Check threshold
	cost = g + heuristic(cube)
	if cost > threshold
		if cost < new_threshold
			new_threshold = cost
		end
		return nothing, new_threshold
	end
	
	# Search deeper
	for (current_move, path_cost) in getmoves(solution)
		new_solution, new_threshold = search(
			move(cube, current_move),
			g + path_cost,
			threshold,
			[current_move, solution...],
			new_threshold
		)
		
		if new_solution ≠ nothing
			return new_solution, 69420
		end
	end
	return nothing, new_threshold
end

# ╔═╡ 2cdef132-8e7e-47cd-a866-b6235245d0ac
md"""
Maybe the algorithm is faster if using a closure
"""

# ╔═╡ 0bd91365-0c38-443c-b243-a73a2d11713a
md"""
Also a function to return a nice string with the solution is implemented as a `pretty_print` keyword argument in the IDA* function. 
"""

# ╔═╡ 788ef26c-a08c-418d-b580-ca5382e236c6
function prettyprint(solution::Vector{<:Any}, delim=" ")
	solution = name.(solution)
	# Join move subarrays
	solution = map(v -> if typeof(v) <: Array{<:Any} 
			join(v, delim) 
			else v 
		end, solution)
	join(solution, delim)
end

# ╔═╡ b95875a3-912f-4993-b248-86f9432c8f91
function IDAsolve(cube; limit = 10, heuristic = _ -> 0, pretty_print = true, solvability_check = true)
	
	solvability, errormsg = issolvable(cube, verbose=true)
	if solvability_check && !solvability
		error("Cube is unsolvable ($errormsg)")
	end

	threshold = heuristic(cube)
	
	for i in 1:limit
		solution, threshold = search(cube, 0, threshold, [], Inf, heuristic)
		
		if solution ≠ nothing
			solution = reverse(solution)
			if pretty_print
				solution = prettyprint(solution)
			end
			return solution
		end
	end
	
	error("No solution found with iteration limit $limit")
end

# ╔═╡ da373699-225b-4ffe-b89b-635027c76f98
const sexy_solution = IDAsolve(sexy_cube)

# ╔═╡ 056aceb1-3c37-4414-ad44-235f8d9407b8
function closureIDA(cube; limit = 10, heuristic = _ -> 0, pretty_print = true)
	
	function search(cube, g, threshold, solution, new_threshold, heuristic = _ -> 0)
	
		# Check if solved
		if issolved(cube)
			return solution, 69420
		end

		# Check threshold
		cost = g + heuristic(cube)
		if cost > threshold
			if cost < new_threshold
				new_threshold = cost
			end
			return nothing, new_threshold
		end

		# Search deeper
		for (current_move, path_cost) in getmoves(solution)
			new_solution, new_threshold = search(
				move(cube, current_move),
				g + path_cost,
				threshold,
				[current_move, solution...],
				new_threshold
			)

			if new_solution ≠ nothing
				return new_solution, 69420
			end
		end
		return nothing, new_threshold
	end
	
	
	solvable, errormsg = issolvable(cube, verbose=true)
	if !solvable
		error("Cube is unsolvable ($errormsg)")
	end

	threshold = heuristic(cube)
	
	for i in 1:limit
		solution, threshold = search(cube, 0, threshold, [], Inf, heuristic)
		
		if solution ≠ nothing
			solution = reverse(solution)
			if pretty_print
				solution = prettyprint(solution)
			end
			return solution
		end
	end
	
	error("No solution found with iteration limit $limit")
	
	
end

# ╔═╡ db9edafd-80ac-43d0-92ad-bd859d6ae0d9
closureIDA(sexy_cube)

# ╔═╡ a494d531-9f41-4430-b590-82754cdc6503
let
	solution = IDAsolve(move(solved_cube, ["R", "L", "U", "D2", "F'"]), pretty_print = false)
	prettyprint(solution)
end

# ╔═╡ d5a661c1-55ad-42e3-b2da-3c3a810f9e2f
md"""
But can it solve a scrambled cube?
"""

# ╔═╡ 164dec9e-94cb-4a8a-a26a-695ce7898a25
# IDAsolve(randomcube())

# ╔═╡ 6e40c591-bb91-4212-b545-863a4c194734
md"""
No. It cannot. 

(yet)
"""

# ╔═╡ 6e846e60-5274-4095-b7e8-b83f13adcab4
md"""
## Rudimentary heuristic calculation

In the context of solving a Rubik's cube, a heuristic should approximate the number of moves left to solve the cube. For the heuristic to be admissible (that is, it guarantees the IDA* to find the optimal solution) it has to never surpass the actual number of moves left. 

A rudimentary heuristic could be based on Manhattan distance, which is easy to implement. However it is not very accurate. 

The most common approach is to save big tables of solutions for specific pieces on the cube and take the maximunm. The groups are the corners, and each two halves of the edges. 

While it takes some computational effort to calculate these tables, the cost of looking up a heuristic is practically instant once it is done. The size is not nearly big enough to pose a challenge to store the values in memory (the size is calculated in the appendix).

"""

# ╔═╡ bf2a43fc-d50e-491d-a5f7-2b813cdb7763
md"""
### Storing files
"""

# ╔═╡ 55864b4f-d748-4646-a5d3-6d0dcba004b8
struct FileStore
	filename::String
	items::Vector{UInt8}
end

# ╔═╡ f530f422-232c-46e6-a528-729e5a314612
const extension = ".barbarosa"

# ╔═╡ 82eba233-3c4e-4f53-a6bf-58b59b694d24
md"""
We will create a struct with custom `getrindex` and `setindex!` methods and a constructor to easily save the heuristics to disk, while being able to manipulate them as an array. All files generated by `barbarosa` will have a "$(extension)" extension. 
"""

# ╔═╡ 5feac4f5-1ca8-47b9-a76d-cfbc10debaa8
function filestore(filename::String)
	items = []
	if isfile(filename * extension)
		open(filename * ".barbarosa", "r") do file
			items = read(file)
		end
	end
	FileStore(filename, items)
end

# ╔═╡ a4a906be-2dbc-4804-9765-5e26ba4be3c2
Base.getindex(collection::FileStore, index::Integer) = collection.items[index]

# ╔═╡ 200adbec-330a-48f7-9464-132f1856ea6c
function Base.setindex!(collection::FileStore, value::Integer, index::Integer)
	difference = index - length(collection.items)
	
	value = convert(UInt8, value)
	
	if difference < 0
		collection.items[index] = value
	else
		append!(collection.items, zeros(difference), value)
	end
	
	open(collection.filename * extension, "w") do file
		write(file, collection.items)
	end
end

# ╔═╡ 21d41d53-c905-47ed-a3c1-f798686660b6
function delete(collection::FileStore)
	rm(collection.filename * extension)
	filter!(v -> false, collection.items)
end

# ╔═╡ 50f26cb0-2882-4cad-9545-5074091d7d98
let
	test_file = filestore("test")
	test_file[5] = 69
	test_file[2] = 5
	
	print = deepcopy(test_file.items)
	delete(test_file)
	print
end

# ╔═╡ d6c36b00-5fd7-4559-b491-273772affb42
md"""
### Indexing cube states
"""

# ╔═╡ 1973f18e-e8ef-4057-b7c1-c91386a9eec5
md"""
Since we have a huge amounts of states for which to store the heuristic, saving the state in each thingy would be very inefficient. Instead, we will create a indexing map for each state, along with corresponfig `getindex` and `setindex!` functions that take a cube state as inputs. 
"""

# ╔═╡ ab90af62-6a3f-47a2-b81a-501803ba29fb
md"""
Here, 0-indexing will be used since it will massively simplify the task.

The indexing map will work by first assigning each piece an index. Then, you will see what index you chose from 0 to the number of options available minus one. Then the value is calculated by multiplying the selection of each piece times the factioral of the available options. 

Mathematically:

$\sum_0^n s_n o_n!$

Where $n$ is the number of pieces, $s_n$ is the selection index at piece $n$ and $o_n$ is the number of options at piece $n$. 
"""

# ╔═╡ a80fc4fb-c8b0-49c2-b4e8-a455b9760537
function getpieceindex(piece::Piece, search_pool::Cube) 
	findfirst(map(v -> v.position == piece.solved_position, search_pool))
end

# ╔═╡ d0274275-06d9-4981-b682-9df25e5b8110
function getcubeindex(cube::Cube, search_pool::Cube, p::Integer)
	available_slots = collect(1:length(search_pool))
	# slots = []
	
	index = 0
	
	for piece in cube
		# id = getpieceindex(piece, search_pool)
		
		# Find the general index of the piece
		goes_index = findfirst(v -> v.solved_position == piece.position, search_pool)
		
		# Find what index that is in the remaining available slots
		goes_index = findfirst(isequal(goes_index), available_slots)

		# Get the selection index using the orientation
		slot_index = (goes_index - 1) * p + piece.orientation
	
		# Make used slot unavailable
		deleteat!(available_slots, goes_index)
		
		n = length(available_slots)
		n == 0 && continue
		
		index += slot_index * factorial(n) * p^(n - 1)
		
		# slots = [slots..., slot_index]
	end

	index
end

# ╔═╡ 1260e70e-7c23-44e8-ac23-96653fc2147f
md"""
Also let's add a method to auto detect the piece type. 
"""

# ╔═╡ f8d5de2b-5277-40ed-af67-195df4d624c7
function getcubeindex(cube::Cube)
	if all(map(piece -> piecetype(piece) === "Edge", cube))
		return getcubeindex(cube, getpieces(solved_cube, "Edge"), 2)
		
	elseif all(map(piece -> piecetype(piece) === "Corner", cube))
		return getcubeindex(cube, getpieces(solved_cube, "Corner"), 3)
		
	else
		throw("Cube doesn't have only one piece type")
	end
end

# ╔═╡ 98b9965d-91d3-4e28-be91-800e34ca8470
let
	onetwoswap = getpieces(solved_cube, "Corner")
	onetwoswap[7] = constructpiece("RDB", 1)
	onetwoswap[8] = constructpiece("LDB", 2)
	getcubeindex(onetwoswap)
end

# ╔═╡ fad4a032-e3ad-46bb-8baf-68dff4c43dfb
@bind moves TextField()

# ╔═╡ d60ebc71-5641-4c90-b282-dafb6ac4fe0a
issolvable(move(sexy_cube, ["R"])), issolvable(move(solved_cube, moves))

# ╔═╡ 73dbf146-2de4-46ea-adc3-23eb7fd31259
const corner_permutations = factorial(8) * 3^7

# ╔═╡ e9fff9ee-7d84-434f-847d-ea5fbd317cc2
const six_edge_permutations = Int(factorial(12) // factorial(6) * 2^6)

# ╔═╡ c1d7f044-30cf-4e8e-bb59-d11ba7249444
getcubeindex(getpieces(move(solved_cube, moves), "Edge")[1:6])

# ╔═╡ 81a03d2c-4c04-4bc0-b81e-d1f78347e653
getcubeindex(getpieces(randomcube(), "Corner"))

# ╔═╡ 2907cc8f-d1e4-4e7c-868f-e7fdf37bf0de
function buildcube(index::Integer, type::String)
	if startswith(type, "Edge")
		available_slots = reverse(collect(1:12))
		p = 2
	elseif startswith(type, "Corner")
		available_slots = reverse(collect(1:8))
		p = 3
	else
		throw("Uknown type to build cube")
	end
	
	slots = []
	while length(available_slots) > 0
		size = factorial(available_slots[1])
		push!(slots, convert(Int, floor(index/size)))
		
		index = index % size
		
		deleteat!(available_slots, 1)
	end
	
	slots
end

# ╔═╡ f764bdc5-5215-4f19-bbe2-a8122a75f2cb
buildcube(11022480, "Corner")

# ╔═╡ b84000e2-504a-4f3d-bdd2-d5d0d67a021c
getpieces(solved_cube, "Corner")

# ╔═╡ a9b08017-dc98-4f50-aa70-a8d5de0b06b2
# IDAsolve(move(getpieces(randomcube(), "Corner"), "R"))

# ╔═╡ 4862029f-c4dc-4e81-928c-678ece56610e
@bind scramble TextField()

# ╔═╡ 7bbf6647-f8a9-46d4-94d2-c8126f9cf6e4
solve = IDAsolve(move(getpieces(solved_cube, "Corner"), scramble))

# ╔═╡ 2862ebff-b001-425b-94f2-ab818f991f14
const superflip = "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2"

# ╔═╡ 1440bb19-2c18-465c-994c-8056e0f84203
move(solved_cube, scramble * solve)

# ╔═╡ 60e6d80b-eb5c-4e5a-83cf-cd9ea793173e
move(solved_cube, "R U")

# ╔═╡ c16bb000-c3f8-4b6f-9e36-4efb9166f3d8
md"""
# Appendix
"""

# ╔═╡ e64bd260-babe-4b06-94cb-e8d18b2035ad
md"""
## Library importation
`PlutoUI` was used in this project. 
"""

# ╔═╡ a965fd07-b833-4c9e-87bb-f27125421a2c
PlutoUI.TableOfContents()

# ╔═╡ 3d7ad0a2-c9f2-4f07-aa2c-bb32099be4d0
md"""
## Computation of rudimentary heuristic table size
"""

# ╔═╡ 498a345d-9126-4cd2-8ee0-b9f7bad31f85
md"""
There are $3$ ways to orient any corner and $8$ places to place them. Because of parity, we know the orientation of the last corner given the other seven, so we have $3^7$ possible corner orientations. 

In a solved cube, the same happens with the positions. However we are not taking into account the edges, so any corner culd be in any position, which gives us $8!$ combinations for corners. 

We multiply them together to get $3^7\cdot8! = 88\,179\,840$ possible combinations. Storing each number as a byte, we have just over 88 megabytes. 

Similarly with 6 of the edges, there are 2 ways to orient them and 12 places to put them, so we get $2^6 \cdot 12!/6!= 42577920$ which is some 42.5 megabytes. 

The final size totals just over 130 megabytes by storing each heuristic as a byte. 
"""

# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
Plots = "91a5bcdd-55d7-5caf-9e0b-520d859cae80"
PlutoUI = "7f904dfe-b85e-4ff6-b463-dae2292396a8"
Random = "9a3f8284-a2c9-5f02-9a11-845980a1fd5c"

[compat]
Plots = "~1.22.3"
PlutoUI = "~0.7.14"
"""

# ╔═╡ 00000000-0000-0000-0000-000000000002
PLUTO_MANIFEST_TOML_CONTENTS = """
# This file is machine-generated - editing it directly is not advised

[[Adapt]]
deps = ["LinearAlgebra"]
git-tree-sha1 = "84918055d15b3114ede17ac6a7182f68870c16f7"
uuid = "79e6a3ab-5dfb-504d-930d-738a2a938a0e"
version = "3.3.1"

[[ArgTools]]
uuid = "0dad84c5-d112-42e6-8d28-ef12dabb789f"

[[Artifacts]]
uuid = "56f22d72-fd6d-98f1-02f0-08ddc0907c33"

[[Base64]]
uuid = "2a0f44e3-6c83-55bd-87e4-b1978d98bd5f"

[[Bzip2_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "19a35467a82e236ff51bc17a3a44b69ef35185a2"
uuid = "6e34b625-4abd-537c-b88f-471c36dfa7a0"
version = "1.0.8+0"

[[Cairo_jll]]
deps = ["Artifacts", "Bzip2_jll", "Fontconfig_jll", "FreeType2_jll", "Glib_jll", "JLLWrappers", "LZO_jll", "Libdl", "Pixman_jll", "Pkg", "Xorg_libXext_jll", "Xorg_libXrender_jll", "Zlib_jll", "libpng_jll"]
git-tree-sha1 = "f2202b55d816427cd385a9a4f3ffb226bee80f99"
uuid = "83423d85-b0ee-5818-9007-b63ccbeb887a"
version = "1.16.1+0"

[[ColorSchemes]]
deps = ["ColorTypes", "Colors", "FixedPointNumbers", "Random"]
git-tree-sha1 = "a851fec56cb73cfdf43762999ec72eff5b86882a"
uuid = "35d6a980-a343-548e-a6ea-1d62b119f2f4"
version = "3.15.0"

[[ColorTypes]]
deps = ["FixedPointNumbers", "Random"]
git-tree-sha1 = "024fe24d83e4a5bf5fc80501a314ce0d1aa35597"
uuid = "3da002f7-5984-5a60-b8a6-cbb66c0b333f"
version = "0.11.0"

[[Colors]]
deps = ["ColorTypes", "FixedPointNumbers", "Reexport"]
git-tree-sha1 = "417b0ed7b8b838aa6ca0a87aadf1bb9eb111ce40"
uuid = "5ae59095-9a9b-59fe-a467-6f913c188581"
version = "0.12.8"

[[Compat]]
deps = ["Base64", "Dates", "DelimitedFiles", "Distributed", "InteractiveUtils", "LibGit2", "Libdl", "LinearAlgebra", "Markdown", "Mmap", "Pkg", "Printf", "REPL", "Random", "SHA", "Serialization", "SharedArrays", "Sockets", "SparseArrays", "Statistics", "Test", "UUIDs", "Unicode"]
git-tree-sha1 = "31d0151f5716b655421d9d75b7fa74cc4e744df2"
uuid = "34da2185-b29b-5c13-b0c7-acf172513d20"
version = "3.39.0"

[[CompilerSupportLibraries_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "e66e0078-7015-5450-92f7-15fbd957f2ae"

[[Contour]]
deps = ["StaticArrays"]
git-tree-sha1 = "9f02045d934dc030edad45944ea80dbd1f0ebea7"
uuid = "d38c429a-6771-53c6-b99e-75d170b6e991"
version = "0.5.7"

[[DataAPI]]
git-tree-sha1 = "cc70b17275652eb47bc9e5f81635981f13cea5c8"
uuid = "9a962f9c-6df0-11e9-0e5d-c546b8b5ee8a"
version = "1.9.0"

[[DataStructures]]
deps = ["Compat", "InteractiveUtils", "OrderedCollections"]
git-tree-sha1 = "7d9d316f04214f7efdbb6398d545446e246eff02"
uuid = "864edb3b-99cc-5e75-8d2d-829cb0a9cfe8"
version = "0.18.10"

[[DataValueInterfaces]]
git-tree-sha1 = "bfc1187b79289637fa0ef6d4436ebdfe6905cbd6"
uuid = "e2d170a0-9d28-54be-80f0-106bbe20a464"
version = "1.0.0"

[[Dates]]
deps = ["Printf"]
uuid = "ade2ca70-3891-5945-98fb-dc099432e06a"

[[DelimitedFiles]]
deps = ["Mmap"]
uuid = "8bb1440f-4735-579b-a4ab-409b98df4dab"

[[Distributed]]
deps = ["Random", "Serialization", "Sockets"]
uuid = "8ba89e20-285c-5b6f-9357-94700520ee1b"

[[Downloads]]
deps = ["ArgTools", "LibCURL", "NetworkOptions"]
uuid = "f43a241f-c20a-4ad4-852c-f6b1247861c6"

[[EarCut_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "3f3a2501fa7236e9b911e0f7a588c657e822bb6d"
uuid = "5ae413db-bbd1-5e63-b57d-d24a61df00f5"
version = "2.2.3+0"

[[Expat_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "b3bfd02e98aedfa5cf885665493c5598c350cd2f"
uuid = "2e619515-83b5-522b-bb60-26c02a35a201"
version = "2.2.10+0"

[[FFMPEG]]
deps = ["FFMPEG_jll"]
git-tree-sha1 = "b57e3acbe22f8484b4b5ff66a7499717fe1a9cc8"
uuid = "c87230d0-a227-11e9-1b43-d7ebe4e7570a"
version = "0.4.1"

[[FFMPEG_jll]]
deps = ["Artifacts", "Bzip2_jll", "FreeType2_jll", "FriBidi_jll", "JLLWrappers", "LAME_jll", "Libdl", "Ogg_jll", "OpenSSL_jll", "Opus_jll", "Pkg", "Zlib_jll", "libass_jll", "libfdk_aac_jll", "libvorbis_jll", "x264_jll", "x265_jll"]
git-tree-sha1 = "d8a578692e3077ac998b50c0217dfd67f21d1e5f"
uuid = "b22a6f82-2f65-5046-a5b2-351ab43fb4e5"
version = "4.4.0+0"

[[FixedPointNumbers]]
deps = ["Statistics"]
git-tree-sha1 = "335bfdceacc84c5cdf16aadc768aa5ddfc5383cc"
uuid = "53c48c17-4a7d-5ca2-90c5-79b7896eea93"
version = "0.8.4"

[[Fontconfig_jll]]
deps = ["Artifacts", "Bzip2_jll", "Expat_jll", "FreeType2_jll", "JLLWrappers", "Libdl", "Libuuid_jll", "Pkg", "Zlib_jll"]
git-tree-sha1 = "21efd19106a55620a188615da6d3d06cd7f6ee03"
uuid = "a3f928ae-7b40-5064-980b-68af3947d34b"
version = "2.13.93+0"

[[Formatting]]
deps = ["Printf"]
git-tree-sha1 = "8339d61043228fdd3eb658d86c926cb282ae72a8"
uuid = "59287772-0a20-5a39-b81b-1366585eb4c0"
version = "0.4.2"

[[FreeType2_jll]]
deps = ["Artifacts", "Bzip2_jll", "JLLWrappers", "Libdl", "Pkg", "Zlib_jll"]
git-tree-sha1 = "87eb71354d8ec1a96d4a7636bd57a7347dde3ef9"
uuid = "d7e528f0-a631-5988-bf34-fe36492bcfd7"
version = "2.10.4+0"

[[FriBidi_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "aa31987c2ba8704e23c6c8ba8a4f769d5d7e4f91"
uuid = "559328eb-81f9-559d-9380-de523a88c83c"
version = "1.0.10+0"

[[GLFW_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Libglvnd_jll", "Pkg", "Xorg_libXcursor_jll", "Xorg_libXi_jll", "Xorg_libXinerama_jll", "Xorg_libXrandr_jll"]
git-tree-sha1 = "dba1e8614e98949abfa60480b13653813d8f0157"
uuid = "0656b61e-2033-5cc2-a64a-77c0f6c09b89"
version = "3.3.5+0"

[[GR]]
deps = ["Base64", "DelimitedFiles", "GR_jll", "HTTP", "JSON", "Libdl", "LinearAlgebra", "Pkg", "Printf", "Random", "Serialization", "Sockets", "Test", "UUIDs"]
git-tree-sha1 = "c2178cfbc0a5a552e16d097fae508f2024de61a3"
uuid = "28b8d3ca-fb5f-59d9-8090-bfdbd6d07a71"
version = "0.59.0"

[[GR_jll]]
deps = ["Artifacts", "Bzip2_jll", "Cairo_jll", "FFMPEG_jll", "Fontconfig_jll", "GLFW_jll", "JLLWrappers", "JpegTurbo_jll", "Libdl", "Libtiff_jll", "Pixman_jll", "Pkg", "Qt5Base_jll", "Zlib_jll", "libpng_jll"]
git-tree-sha1 = "ef49a187604f865f4708c90e3f431890724e9012"
uuid = "d2c73de3-f751-5644-a686-071e5b155ba9"
version = "0.59.0+0"

[[GeometryBasics]]
deps = ["EarCut_jll", "IterTools", "LinearAlgebra", "StaticArrays", "StructArrays", "Tables"]
git-tree-sha1 = "58bcdf5ebc057b085e58d95c138725628dd7453c"
uuid = "5c1252a2-5f33-56bf-86c9-59e7332b4326"
version = "0.4.1"

[[Gettext_jll]]
deps = ["Artifacts", "CompilerSupportLibraries_jll", "JLLWrappers", "Libdl", "Libiconv_jll", "Pkg", "XML2_jll"]
git-tree-sha1 = "9b02998aba7bf074d14de89f9d37ca24a1a0b046"
uuid = "78b55507-aeef-58d4-861c-77aaff3498b1"
version = "0.21.0+0"

[[Glib_jll]]
deps = ["Artifacts", "Gettext_jll", "JLLWrappers", "Libdl", "Libffi_jll", "Libiconv_jll", "Libmount_jll", "PCRE_jll", "Pkg", "Zlib_jll"]
git-tree-sha1 = "7bf67e9a481712b3dbe9cb3dac852dc4b1162e02"
uuid = "7746bdde-850d-59dc-9ae8-88ece973131d"
version = "2.68.3+0"

[[Graphite2_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "344bf40dcab1073aca04aa0df4fb092f920e4011"
uuid = "3b182d85-2403-5c21-9c21-1e1f0cc25472"
version = "1.3.14+0"

[[Grisu]]
git-tree-sha1 = "53bb909d1151e57e2484c3d1b53e19552b887fb2"
uuid = "42e2da0e-8278-4e71-bc24-59509adca0fe"
version = "1.0.2"

[[HTTP]]
deps = ["Base64", "Dates", "IniFile", "Logging", "MbedTLS", "NetworkOptions", "Sockets", "URIs"]
git-tree-sha1 = "14eece7a3308b4d8be910e265c724a6ba51a9798"
uuid = "cd3eb016-35fb-5094-929b-558a96fad6f3"
version = "0.9.16"

[[HarfBuzz_jll]]
deps = ["Artifacts", "Cairo_jll", "Fontconfig_jll", "FreeType2_jll", "Glib_jll", "Graphite2_jll", "JLLWrappers", "Libdl", "Libffi_jll", "Pkg"]
git-tree-sha1 = "8a954fed8ac097d5be04921d595f741115c1b2ad"
uuid = "2e76f6c2-a576-52d4-95c1-20adfe4de566"
version = "2.8.1+0"

[[HypertextLiteral]]
git-tree-sha1 = "72053798e1be56026b81d4e2682dbe58922e5ec9"
uuid = "ac1192a8-f4b3-4bfe-ba22-af5b92cd3ab2"
version = "0.9.0"

[[IOCapture]]
deps = ["Logging", "Random"]
git-tree-sha1 = "f7be53659ab06ddc986428d3a9dcc95f6fa6705a"
uuid = "b5f81e59-6552-4d32-b1f0-c071b021bf89"
version = "0.2.2"

[[IniFile]]
deps = ["Test"]
git-tree-sha1 = "098e4d2c533924c921f9f9847274f2ad89e018b8"
uuid = "83e8ac13-25f8-5344-8a64-a9f2b223428f"
version = "0.5.0"

[[InteractiveUtils]]
deps = ["Markdown"]
uuid = "b77e0a4c-d291-57a0-90e8-8db25a27a240"

[[IterTools]]
git-tree-sha1 = "05110a2ab1fc5f932622ffea2a003221f4782c18"
uuid = "c8e1da08-722c-5040-9ed9-7db0dc04731e"
version = "1.3.0"

[[IteratorInterfaceExtensions]]
git-tree-sha1 = "a3f24677c21f5bbe9d2a714f95dcd58337fb2856"
uuid = "82899510-4779-5014-852e-03e436cf321d"
version = "1.0.0"

[[JLLWrappers]]
deps = ["Preferences"]
git-tree-sha1 = "642a199af8b68253517b80bd3bfd17eb4e84df6e"
uuid = "692b3bcd-3c85-4b1f-b108-f13ce0eb3210"
version = "1.3.0"

[[JSON]]
deps = ["Dates", "Mmap", "Parsers", "Unicode"]
git-tree-sha1 = "8076680b162ada2a031f707ac7b4953e30667a37"
uuid = "682c06a0-de6a-54ab-a142-c8b1cf79cde6"
version = "0.21.2"

[[JpegTurbo_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "d735490ac75c5cb9f1b00d8b5509c11984dc6943"
uuid = "aacddb02-875f-59d6-b918-886e6ef4fbf8"
version = "2.1.0+0"

[[LAME_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "f6250b16881adf048549549fba48b1161acdac8c"
uuid = "c1c5ebd0-6772-5130-a774-d5fcae4a789d"
version = "3.100.1+0"

[[LZO_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "e5b909bcf985c5e2605737d2ce278ed791b89be6"
uuid = "dd4b983a-f0e5-5f8d-a1b7-129d4a5fb1ac"
version = "2.10.1+0"

[[LaTeXStrings]]
git-tree-sha1 = "c7f1c695e06c01b95a67f0cd1d34994f3e7db104"
uuid = "b964fa9f-0449-5b57-a5c2-d3ea65f4040f"
version = "1.2.1"

[[Latexify]]
deps = ["Formatting", "InteractiveUtils", "LaTeXStrings", "MacroTools", "Markdown", "Printf", "Requires"]
git-tree-sha1 = "a4b12a1bd2ebade87891ab7e36fdbce582301a92"
uuid = "23fbe1c1-3f47-55db-b15f-69d7ec21a316"
version = "0.15.6"

[[LibCURL]]
deps = ["LibCURL_jll", "MozillaCACerts_jll"]
uuid = "b27032c2-a3e7-50c8-80cd-2d36dbcbfd21"

[[LibCURL_jll]]
deps = ["Artifacts", "LibSSH2_jll", "Libdl", "MbedTLS_jll", "Zlib_jll", "nghttp2_jll"]
uuid = "deac9b47-8bc7-5906-a0fe-35ac56dc84c0"

[[LibGit2]]
deps = ["Base64", "NetworkOptions", "Printf", "SHA"]
uuid = "76f85450-5226-5b5a-8eaa-529ad045b433"

[[LibSSH2_jll]]
deps = ["Artifacts", "Libdl", "MbedTLS_jll"]
uuid = "29816b5a-b9ab-546f-933c-edad1886dfa8"

[[Libdl]]
uuid = "8f399da3-3557-5675-b5ff-fb832c97cbdb"

[[Libffi_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "761a393aeccd6aa92ec3515e428c26bf99575b3b"
uuid = "e9f186c6-92d2-5b65-8a66-fee21dc1b490"
version = "3.2.2+0"

[[Libgcrypt_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Libgpg_error_jll", "Pkg"]
git-tree-sha1 = "64613c82a59c120435c067c2b809fc61cf5166ae"
uuid = "d4300ac3-e22c-5743-9152-c294e39db1e4"
version = "1.8.7+0"

[[Libglvnd_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libX11_jll", "Xorg_libXext_jll"]
git-tree-sha1 = "7739f837d6447403596a75d19ed01fd08d6f56bf"
uuid = "7e76a0d4-f3c7-5321-8279-8d96eeed0f29"
version = "1.3.0+3"

[[Libgpg_error_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "c333716e46366857753e273ce6a69ee0945a6db9"
uuid = "7add5ba3-2f88-524e-9cd5-f83b8a55f7b8"
version = "1.42.0+0"

[[Libiconv_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "42b62845d70a619f063a7da093d995ec8e15e778"
uuid = "94ce4f54-9a6c-5748-9c1c-f9c7231a4531"
version = "1.16.1+1"

[[Libmount_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "9c30530bf0effd46e15e0fdcf2b8636e78cbbd73"
uuid = "4b2f31a3-9ecc-558c-b454-b3730dcb73e9"
version = "2.35.0+0"

[[Libtiff_jll]]
deps = ["Artifacts", "JLLWrappers", "JpegTurbo_jll", "Libdl", "Pkg", "Zlib_jll", "Zstd_jll"]
git-tree-sha1 = "340e257aada13f95f98ee352d316c3bed37c8ab9"
uuid = "89763e89-9b03-5906-acba-b20f662cd828"
version = "4.3.0+0"

[[Libuuid_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "7f3efec06033682db852f8b3bc3c1d2b0a0ab066"
uuid = "38a345b3-de98-5d2b-a5d3-14cd9215e700"
version = "2.36.0+0"

[[LinearAlgebra]]
deps = ["Libdl"]
uuid = "37e2e46d-f89d-539d-b4ee-838fcccc9c8e"

[[Logging]]
uuid = "56ddb016-857b-54e1-b83d-db4d58db5568"

[[MacroTools]]
deps = ["Markdown", "Random"]
git-tree-sha1 = "5a5bc6bf062f0f95e62d0fe0a2d99699fed82dd9"
uuid = "1914dd2f-81c6-5fcd-8719-6d5c9610ff09"
version = "0.5.8"

[[Markdown]]
deps = ["Base64"]
uuid = "d6f4376e-aef5-505a-96c1-9c027394607a"

[[MbedTLS]]
deps = ["Dates", "MbedTLS_jll", "Random", "Sockets"]
git-tree-sha1 = "1c38e51c3d08ef2278062ebceade0e46cefc96fe"
uuid = "739be429-bea8-5141-9913-cc70e7f3736d"
version = "1.0.3"

[[MbedTLS_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "c8ffd9c3-330d-5841-b78e-0817d7145fa1"

[[Measures]]
git-tree-sha1 = "e498ddeee6f9fdb4551ce855a46f54dbd900245f"
uuid = "442fdcdd-2543-5da2-b0f3-8c86c306513e"
version = "0.3.1"

[[Missings]]
deps = ["DataAPI"]
git-tree-sha1 = "bf210ce90b6c9eed32d25dbcae1ebc565df2687f"
uuid = "e1d29d7a-bbdc-5cf2-9ac0-f12de2c33e28"
version = "1.0.2"

[[Mmap]]
uuid = "a63ad114-7e13-5084-954f-fe012c677804"

[[MozillaCACerts_jll]]
uuid = "14a3606d-f60d-562e-9121-12d972cd8159"

[[NaNMath]]
git-tree-sha1 = "bfe47e760d60b82b66b61d2d44128b62e3a369fb"
uuid = "77ba4419-2d1f-58cd-9bb1-8ffee604a2e3"
version = "0.3.5"

[[NetworkOptions]]
uuid = "ca575930-c2e3-43a9-ace4-1e988b2c1908"

[[Ogg_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "7937eda4681660b4d6aeeecc2f7e1c81c8ee4e2f"
uuid = "e7412a2a-1a6e-54c0-be00-318e2571c051"
version = "1.3.5+0"

[[OpenSSL_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "15003dcb7d8db3c6c857fda14891a539a8f2705a"
uuid = "458c3c95-2e84-50aa-8efc-19380b2a3a95"
version = "1.1.10+0"

[[Opus_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "51a08fb14ec28da2ec7a927c4337e4332c2a4720"
uuid = "91d4177d-7536-5919-b921-800302f37372"
version = "1.3.2+0"

[[OrderedCollections]]
git-tree-sha1 = "85f8e6578bf1f9ee0d11e7bb1b1456435479d47c"
uuid = "bac558e1-5e72-5ebc-8fee-abe8a469f55d"
version = "1.4.1"

[[PCRE_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "b2a7af664e098055a7529ad1a900ded962bca488"
uuid = "2f80f16e-611a-54ab-bc61-aa92de5b98fc"
version = "8.44.0+0"

[[Parsers]]
deps = ["Dates"]
git-tree-sha1 = "a8709b968a1ea6abc2dc1967cb1db6ac9a00dfb6"
uuid = "69de0a69-1ddd-5017-9359-2bf0b02dc9f0"
version = "2.0.5"

[[Pixman_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "b4f5d02549a10e20780a24fce72bea96b6329e29"
uuid = "30392449-352a-5448-841d-b1acce4e97dc"
version = "0.40.1+0"

[[Pkg]]
deps = ["Artifacts", "Dates", "Downloads", "LibGit2", "Libdl", "Logging", "Markdown", "Printf", "REPL", "Random", "SHA", "Serialization", "TOML", "Tar", "UUIDs", "p7zip_jll"]
uuid = "44cfe95a-1eb2-52ea-b672-e2afdf69b78f"

[[PlotThemes]]
deps = ["PlotUtils", "Requires", "Statistics"]
git-tree-sha1 = "a3a964ce9dc7898193536002a6dd892b1b5a6f1d"
uuid = "ccf2f8ad-2431-5c83-bf29-c5338b663b6a"
version = "2.0.1"

[[PlotUtils]]
deps = ["ColorSchemes", "Colors", "Dates", "Printf", "Random", "Reexport", "Statistics"]
git-tree-sha1 = "2537ed3c0ed5e03896927187f5f2ee6a4ab342db"
uuid = "995b91a9-d308-5afd-9ec6-746e21dbc043"
version = "1.0.14"

[[Plots]]
deps = ["Base64", "Contour", "Dates", "Downloads", "FFMPEG", "FixedPointNumbers", "GR", "GeometryBasics", "JSON", "Latexify", "LinearAlgebra", "Measures", "NaNMath", "PlotThemes", "PlotUtils", "Printf", "REPL", "Random", "RecipesBase", "RecipesPipeline", "Reexport", "Requires", "Scratch", "Showoff", "SparseArrays", "Statistics", "StatsBase", "UUIDs"]
git-tree-sha1 = "cfbd033def161db9494f86c5d18fbf874e09e514"
uuid = "91a5bcdd-55d7-5caf-9e0b-520d859cae80"
version = "1.22.3"

[[PlutoUI]]
deps = ["Base64", "Dates", "HypertextLiteral", "IOCapture", "InteractiveUtils", "JSON", "Logging", "Markdown", "Random", "Reexport", "UUIDs"]
git-tree-sha1 = "d1fb76655a95bf6ea4348d7197b22e889a4375f4"
uuid = "7f904dfe-b85e-4ff6-b463-dae2292396a8"
version = "0.7.14"

[[Preferences]]
deps = ["TOML"]
git-tree-sha1 = "00cfd92944ca9c760982747e9a1d0d5d86ab1e5a"
uuid = "21216c6a-2e73-6563-6e65-726566657250"
version = "1.2.2"

[[Printf]]
deps = ["Unicode"]
uuid = "de0858da-6303-5e67-8744-51eddeeeb8d7"

[[Qt5Base_jll]]
deps = ["Artifacts", "CompilerSupportLibraries_jll", "Fontconfig_jll", "Glib_jll", "JLLWrappers", "Libdl", "Libglvnd_jll", "OpenSSL_jll", "Pkg", "Xorg_libXext_jll", "Xorg_libxcb_jll", "Xorg_xcb_util_image_jll", "Xorg_xcb_util_keysyms_jll", "Xorg_xcb_util_renderutil_jll", "Xorg_xcb_util_wm_jll", "Zlib_jll", "xkbcommon_jll"]
git-tree-sha1 = "ad368663a5e20dbb8d6dc2fddeefe4dae0781ae8"
uuid = "ea2cea3b-5b76-57ae-a6ef-0a8af62496e1"
version = "5.15.3+0"

[[REPL]]
deps = ["InteractiveUtils", "Markdown", "Sockets", "Unicode"]
uuid = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"

[[Random]]
deps = ["Serialization"]
uuid = "9a3f8284-a2c9-5f02-9a11-845980a1fd5c"

[[RecipesBase]]
git-tree-sha1 = "44a75aa7a527910ee3d1751d1f0e4148698add9e"
uuid = "3cdcf5f2-1ef4-517c-9805-6587b60abb01"
version = "1.1.2"

[[RecipesPipeline]]
deps = ["Dates", "NaNMath", "PlotUtils", "RecipesBase"]
git-tree-sha1 = "7ad0dfa8d03b7bcf8c597f59f5292801730c55b8"
uuid = "01d81517-befc-4cb6-b9ec-a95719d0359c"
version = "0.4.1"

[[Reexport]]
git-tree-sha1 = "45e428421666073eab6f2da5c9d310d99bb12f9b"
uuid = "189a3867-3050-52da-a836-e630ba90ab69"
version = "1.2.2"

[[Requires]]
deps = ["UUIDs"]
git-tree-sha1 = "4036a3bd08ac7e968e27c203d45f5fff15020621"
uuid = "ae029012-a4dd-5104-9daa-d747884805df"
version = "1.1.3"

[[SHA]]
uuid = "ea8e919c-243c-51af-8825-aaa63cd721ce"

[[Scratch]]
deps = ["Dates"]
git-tree-sha1 = "0b4b7f1393cff97c33891da2a0bf69c6ed241fda"
uuid = "6c6a2e73-6563-6170-7368-637461726353"
version = "1.1.0"

[[Serialization]]
uuid = "9e88b42a-f829-5b0c-bbe9-9e923198166b"

[[SharedArrays]]
deps = ["Distributed", "Mmap", "Random", "Serialization"]
uuid = "1a1011a3-84de-559e-8e89-a11a2f7dc383"

[[Showoff]]
deps = ["Dates", "Grisu"]
git-tree-sha1 = "91eddf657aca81df9ae6ceb20b959ae5653ad1de"
uuid = "992d4aef-0814-514b-bc4d-f2e9a6c4116f"
version = "1.0.3"

[[Sockets]]
uuid = "6462fe0b-24de-5631-8697-dd941f90decc"

[[SortingAlgorithms]]
deps = ["DataStructures"]
git-tree-sha1 = "b3363d7460f7d098ca0912c69b082f75625d7508"
uuid = "a2af1166-a08f-5f64-846c-94a0d3cef48c"
version = "1.0.1"

[[SparseArrays]]
deps = ["LinearAlgebra", "Random"]
uuid = "2f01184e-e22b-5df5-ae63-d93ebab69eaf"

[[StaticArrays]]
deps = ["LinearAlgebra", "Random", "Statistics"]
git-tree-sha1 = "3c76dde64d03699e074ac02eb2e8ba8254d428da"
uuid = "90137ffa-7385-5640-81b9-e52037218182"
version = "1.2.13"

[[Statistics]]
deps = ["LinearAlgebra", "SparseArrays"]
uuid = "10745b16-79ce-11e8-11f9-7d13ad32a3b2"

[[StatsAPI]]
git-tree-sha1 = "1958272568dc176a1d881acb797beb909c785510"
uuid = "82ae8749-77ed-4fe6-ae5f-f523153014b0"
version = "1.0.0"

[[StatsBase]]
deps = ["DataAPI", "DataStructures", "LinearAlgebra", "Missings", "Printf", "Random", "SortingAlgorithms", "SparseArrays", "Statistics", "StatsAPI"]
git-tree-sha1 = "8cbbc098554648c84f79a463c9ff0fd277144b6c"
uuid = "2913bbd2-ae8a-5f71-8c99-4fb6c76f3a91"
version = "0.33.10"

[[StructArrays]]
deps = ["Adapt", "DataAPI", "StaticArrays", "Tables"]
git-tree-sha1 = "2ce41e0d042c60ecd131e9fb7154a3bfadbf50d3"
uuid = "09ab397b-f2b6-538f-b94a-2f83cf4a842a"
version = "0.6.3"

[[TOML]]
deps = ["Dates"]
uuid = "fa267f1f-6049-4f14-aa54-33bafae1ed76"

[[TableTraits]]
deps = ["IteratorInterfaceExtensions"]
git-tree-sha1 = "c06b2f539df1c6efa794486abfb6ed2022561a39"
uuid = "3783bdb8-4a98-5b6b-af9a-565f29a5fe9c"
version = "1.0.1"

[[Tables]]
deps = ["DataAPI", "DataValueInterfaces", "IteratorInterfaceExtensions", "LinearAlgebra", "TableTraits", "Test"]
git-tree-sha1 = "1162ce4a6c4b7e31e0e6b14486a6986951c73be9"
uuid = "bd369af6-aec1-5ad0-b16a-f7cc5008161c"
version = "1.5.2"

[[Tar]]
deps = ["ArgTools", "SHA"]
uuid = "a4e569a6-e804-4fa4-b0f3-eef7a1d5b13e"

[[Test]]
deps = ["InteractiveUtils", "Logging", "Random", "Serialization"]
uuid = "8dfed614-e22c-5e08-85e1-65c5234f0b40"

[[URIs]]
git-tree-sha1 = "97bbe755a53fe859669cd907f2d96aee8d2c1355"
uuid = "5c2747f8-b7ea-4ff2-ba2e-563bfd36b1d4"
version = "1.3.0"

[[UUIDs]]
deps = ["Random", "SHA"]
uuid = "cf7118a7-6976-5b1a-9a39-7adc72f591a4"

[[Unicode]]
uuid = "4ec0a83e-493e-50e2-b9ac-8f72acf5a8f5"

[[Wayland_jll]]
deps = ["Artifacts", "Expat_jll", "JLLWrappers", "Libdl", "Libffi_jll", "Pkg", "XML2_jll"]
git-tree-sha1 = "3e61f0b86f90dacb0bc0e73a0c5a83f6a8636e23"
uuid = "a2964d1f-97da-50d4-b82a-358c7fce9d89"
version = "1.19.0+0"

[[Wayland_protocols_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Wayland_jll"]
git-tree-sha1 = "2839f1c1296940218e35df0bbb220f2a79686670"
uuid = "2381bf8a-dfd0-557d-9999-79630e7b1b91"
version = "1.18.0+4"

[[XML2_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Libiconv_jll", "Pkg", "Zlib_jll"]
git-tree-sha1 = "1acf5bdf07aa0907e0a37d3718bb88d4b687b74a"
uuid = "02c8fc9c-b97f-50b9-bbe4-9be30ff0a78a"
version = "2.9.12+0"

[[XSLT_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Libgcrypt_jll", "Libgpg_error_jll", "Libiconv_jll", "Pkg", "XML2_jll", "Zlib_jll"]
git-tree-sha1 = "91844873c4085240b95e795f692c4cec4d805f8a"
uuid = "aed1982a-8fda-507f-9586-7b0439959a61"
version = "1.1.34+0"

[[Xorg_libX11_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libxcb_jll", "Xorg_xtrans_jll"]
git-tree-sha1 = "5be649d550f3f4b95308bf0183b82e2582876527"
uuid = "4f6342f7-b3d2-589e-9d20-edeb45f2b2bc"
version = "1.6.9+4"

[[Xorg_libXau_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "4e490d5c960c314f33885790ed410ff3a94ce67e"
uuid = "0c0b7dd1-d40b-584c-a123-a41640f87eec"
version = "1.0.9+4"

[[Xorg_libXcursor_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libXfixes_jll", "Xorg_libXrender_jll"]
git-tree-sha1 = "12e0eb3bc634fa2080c1c37fccf56f7c22989afd"
uuid = "935fb764-8cf2-53bf-bb30-45bb1f8bf724"
version = "1.2.0+4"

[[Xorg_libXdmcp_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "4fe47bd2247248125c428978740e18a681372dd4"
uuid = "a3789734-cfe1-5b06-b2d0-1dd0d9d62d05"
version = "1.1.3+4"

[[Xorg_libXext_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libX11_jll"]
git-tree-sha1 = "b7c0aa8c376b31e4852b360222848637f481f8c3"
uuid = "1082639a-0dae-5f34-9b06-72781eeb8cb3"
version = "1.3.4+4"

[[Xorg_libXfixes_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libX11_jll"]
git-tree-sha1 = "0e0dc7431e7a0587559f9294aeec269471c991a4"
uuid = "d091e8ba-531a-589c-9de9-94069b037ed8"
version = "5.0.3+4"

[[Xorg_libXi_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libXext_jll", "Xorg_libXfixes_jll"]
git-tree-sha1 = "89b52bc2160aadc84d707093930ef0bffa641246"
uuid = "a51aa0fd-4e3c-5386-b890-e753decda492"
version = "1.7.10+4"

[[Xorg_libXinerama_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libXext_jll"]
git-tree-sha1 = "26be8b1c342929259317d8b9f7b53bf2bb73b123"
uuid = "d1454406-59df-5ea1-beac-c340f2130bc3"
version = "1.1.4+4"

[[Xorg_libXrandr_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libXext_jll", "Xorg_libXrender_jll"]
git-tree-sha1 = "34cea83cb726fb58f325887bf0612c6b3fb17631"
uuid = "ec84b674-ba8e-5d96-8ba1-2a689ba10484"
version = "1.5.2+4"

[[Xorg_libXrender_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libX11_jll"]
git-tree-sha1 = "19560f30fd49f4d4efbe7002a1037f8c43d43b96"
uuid = "ea2f1a96-1ddc-540d-b46f-429655e07cfa"
version = "0.9.10+4"

[[Xorg_libpthread_stubs_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "6783737e45d3c59a4a4c4091f5f88cdcf0908cbb"
uuid = "14d82f49-176c-5ed1-bb49-ad3f5cbd8c74"
version = "0.1.0+3"

[[Xorg_libxcb_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "XSLT_jll", "Xorg_libXau_jll", "Xorg_libXdmcp_jll", "Xorg_libpthread_stubs_jll"]
git-tree-sha1 = "daf17f441228e7a3833846cd048892861cff16d6"
uuid = "c7cfdc94-dc32-55de-ac96-5a1b8d977c5b"
version = "1.13.0+3"

[[Xorg_libxkbfile_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libX11_jll"]
git-tree-sha1 = "926af861744212db0eb001d9e40b5d16292080b2"
uuid = "cc61e674-0454-545c-8b26-ed2c68acab7a"
version = "1.1.0+4"

[[Xorg_xcb_util_image_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_xcb_util_jll"]
git-tree-sha1 = "0fab0a40349ba1cba2c1da699243396ff8e94b97"
uuid = "12413925-8142-5f55-bb0e-6d7ca50bb09b"
version = "0.4.0+1"

[[Xorg_xcb_util_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libxcb_jll"]
git-tree-sha1 = "e7fd7b2881fa2eaa72717420894d3938177862d1"
uuid = "2def613f-5ad1-5310-b15b-b15d46f528f5"
version = "0.4.0+1"

[[Xorg_xcb_util_keysyms_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_xcb_util_jll"]
git-tree-sha1 = "d1151e2c45a544f32441a567d1690e701ec89b00"
uuid = "975044d2-76e6-5fbe-bf08-97ce7c6574c7"
version = "0.4.0+1"

[[Xorg_xcb_util_renderutil_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_xcb_util_jll"]
git-tree-sha1 = "dfd7a8f38d4613b6a575253b3174dd991ca6183e"
uuid = "0d47668e-0667-5a69-a72c-f761630bfb7e"
version = "0.3.9+1"

[[Xorg_xcb_util_wm_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_xcb_util_jll"]
git-tree-sha1 = "e78d10aab01a4a154142c5006ed44fd9e8e31b67"
uuid = "c22f9ab0-d5fe-5066-847c-f4bb1cd4e361"
version = "0.4.1+1"

[[Xorg_xkbcomp_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_libxkbfile_jll"]
git-tree-sha1 = "4bcbf660f6c2e714f87e960a171b119d06ee163b"
uuid = "35661453-b289-5fab-8a00-3d9160c6a3a4"
version = "1.4.2+4"

[[Xorg_xkeyboard_config_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Xorg_xkbcomp_jll"]
git-tree-sha1 = "5c8424f8a67c3f2209646d4425f3d415fee5931d"
uuid = "33bec58e-1273-512f-9401-5d533626f822"
version = "2.27.0+4"

[[Xorg_xtrans_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "79c31e7844f6ecf779705fbc12146eb190b7d845"
uuid = "c5fb5394-a638-5e4d-96e5-b29de1b5cf10"
version = "1.4.0+3"

[[Zlib_jll]]
deps = ["Libdl"]
uuid = "83775a58-1f1d-513f-b197-d71354ab007a"

[[Zstd_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "cc4bf3fdde8b7e3e9fa0351bdeedba1cf3b7f6e6"
uuid = "3161d3a3-bdf6-5164-811a-617609db77b4"
version = "1.5.0+0"

[[libass_jll]]
deps = ["Artifacts", "Bzip2_jll", "FreeType2_jll", "FriBidi_jll", "HarfBuzz_jll", "JLLWrappers", "Libdl", "Pkg", "Zlib_jll"]
git-tree-sha1 = "5982a94fcba20f02f42ace44b9894ee2b140fe47"
uuid = "0ac62f75-1d6f-5e53-bd7c-93b484bb37c0"
version = "0.15.1+0"

[[libfdk_aac_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "daacc84a041563f965be61859a36e17c4e4fcd55"
uuid = "f638f0a6-7fb0-5443-88ba-1cc74229b280"
version = "2.0.2+0"

[[libpng_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Zlib_jll"]
git-tree-sha1 = "94d180a6d2b5e55e447e2d27a29ed04fe79eb30c"
uuid = "b53b4c65-9356-5827-b1ea-8c7a1a84506f"
version = "1.6.38+0"

[[libvorbis_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Ogg_jll", "Pkg"]
git-tree-sha1 = "c45f4e40e7aafe9d086379e5578947ec8b95a8fb"
uuid = "f27f6e37-5d2b-51aa-960f-b287f2bc3b7a"
version = "1.3.7+0"

[[nghttp2_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "8e850ede-7688-5339-a07c-302acd2aaf8d"

[[p7zip_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "3f19e933-33d8-53b3-aaab-bd5110c3b7a0"

[[x264_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "4fea590b89e6ec504593146bf8b988b2c00922b2"
uuid = "1270edf5-f2f9-52d2-97e9-ab00b5d0237a"
version = "2021.5.5+0"

[[x265_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "ee567a171cce03570d77ad3a43e90218e38937a9"
uuid = "dfaa095f-4041-5dcd-9319-2fabd8486b76"
version = "3.5.0+0"

[[xkbcommon_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg", "Wayland_jll", "Wayland_protocols_jll", "Xorg_libxcb_jll", "Xorg_xkeyboard_config_jll"]
git-tree-sha1 = "ece2350174195bb31de1a63bea3a41ae1aa593b6"
uuid = "d8fb68d0-12a3-5cfd-a85a-d49703b185fd"
version = "0.9.1+5"
"""

# ╔═╡ Cell order:
# ╟─2d3eb219-6234-44c5-9dbb-bc2b47e99958
# ╟─e05e9488-c991-40ee-890d-63f0d04dbe00
# ╟─86573f98-6657-444a-8512-555b2305dbf9
# ╠═e06f3ff2-7b70-4af9-b5a1-29d6a95d9416
# ╟─f22871f6-03e5-4146-a24b-0b19de53cef3
# ╠═a6ae98d4-a7c4-41b5-aceb-b07cbd9c5ee0
# ╠═b6b5d4df-ca08-4a4c-8cd7-79ba8e471dee
# ╠═59f39125-dbe6-4246-b729-d26a48ab967c
# ╟─3119ebc3-6281-41c0-834b-e9ee925908b8
# ╠═bf24b380-366e-415f-a665-6721d83ec87b
# ╠═0d089088-2389-4121-bea9-50a8e2a0d3aa
# ╠═8eff452c-e23e-4c61-ba0f-e43d1a922ded
# ╟─f23d399c-b6b9-4f71-9608-e7975a6aa497
# ╠═a28bafb6-b95a-48f3-b8a3-5cb6894977b2
# ╟─5d7aaac0-dc85-4891-b78d-7fbf5a5c87b5
# ╠═1a76e513-c4ee-4333-aee8-c0c95d7bcf9b
# ╠═47a4e5ae-d72a-4823-8579-f47653b28a3a
# ╠═f9a24873-4e03-4648-b966-e9aceb8b4d58
# ╠═f84832a1-1fe9-4df7-a813-a4be7a079d83
# ╟─7b02de6a-20b2-411a-b374-aa461a527109
# ╠═6f0c7aa8-392d-42c9-9638-55ef53e9aa25
# ╠═44787f95-0fc3-4afd-b682-0b8eb8e66d4b
# ╟─5045dcc9-de20-4d6e-9a60-31b633f61d11
# ╠═fe09f584-9be3-42bc-bc61-b4e9cc1dd700
# ╟─9adab1f2-fa93-42c1-a00e-06258b17dd44
# ╠═80e6e702-3766-41f4-a303-d224e2818752
# ╟─6ee79fca-e240-446a-8c66-53820ee965c2
# ╠═41600c68-045e-4443-a187-601ff9186877
# ╠═4be974ef-436a-441b-831c-5a0f62becb4a
# ╠═eb1d7422-adce-42d6-8319-947598e46682
# ╠═a757bb4e-c82a-4459-a967-33260ad902c0
# ╟─7930b143-d396-467d-81df-eb33337006db
# ╟─3248b006-35de-46ec-878b-deb8448a68fc
# ╠═ce7f6993-c1fa-4064-b18f-92fcbfe63387
# ╠═856c1135-282e-48b4-afe2-ebbe92650d25
# ╠═3942e7d2-1a0c-438b-ada6-c245a67ccfa5
# ╟─243bc6f0-814a-4f73-a414-86864a217b43
# ╠═384c32a9-fc0c-4563-8fc0-8eedf0ac0f20
# ╠═fda5ad80-b79f-4464-b35d-fe6a29bca77d
# ╠═3b9ffb15-86ec-4ca2-9e04-0c8eadc61c54
# ╠═7cbcc253-69d3-40b9-a559-311a8bf98a12
# ╟─3f5b9fc4-abfa-4d14-b846-1becb0a9bbbb
# ╟─410ac675-a7e8-484a-93fb-bf6bbb3115e6
# ╠═638a1ca2-850b-429d-9d76-4823a0f7b0bd
# ╟─b4a3bf53-de29-4890-a6f2-3746a6022265
# ╠═0e4f52b0-9831-488f-af79-5bd60c999f1d
# ╠═24d516a6-cfe4-493f-978b-839c1814e836
# ╠═bacb17e5-2fff-49fe-a27f-d35d67d59716
# ╟─51fc06f2-cdfb-4951-b6c4-0a59c6bc2608
# ╠═325c6dcb-b38b-4fb5-93ef-4ae934a5b963
# ╠═ccc74b2d-1c83-44df-99bf-745aed13cc71
# ╠═229d2567-25e7-4c5c-8f60-ba00404eaa4d
# ╟─045dc3e9-e1e8-4aa2-b3aa-d906d88dccee
# ╠═19b07f28-d7a5-41a0-b9c4-2cde0c575b26
# ╟─e8b65eba-4b79-4aac-a614-122f469925fe
# ╠═8e24fffa-b9fb-4d82-9865-63807edf49cf
# ╠═6dd34804-f815-4fa4-995f-d6b8cf2a4ce1
# ╠═8b95c08c-46b9-4be7-bf14-75ef5dd09f8d
# ╠═66ef63eb-4362-439f-9197-9662319c5980
# ╟─22e4a4f6-17f3-40f3-98e8-432e646cc73d
# ╠═811e9bac-4f50-40b2-9ac0-5a0e46b65777
# ╠═04569bda-1b60-4c71-b7ca-54eba379264b
# ╠═aab1febf-b210-4b1e-89a5-141a84355835
# ╠═5e66352f-3989-4807-b295-a0cf31aae2d0
# ╠═6fe5470e-4c34-4fc2-97d1-a6c897253019
# ╟─5b7c7056-b147-4d39-99b6-46d080c746c1
# ╠═02ec5f2a-ece5-4860-aa02-470e129335fc
# ╠═842f3cdb-a66e-450a-b579-bd3bd3948210
# ╠═21f5b811-76d1-441d-bb29-ee7a4aa3dc0e
# ╠═caae487b-7942-452f-81c4-a290d8205271
# ╟─430426a8-7ecd-4164-b1f5-917acef06584
# ╠═d18076fa-2376-4f85-bfb3-4062f01176ac
# ╠═e5e2b105-fb37-4e5d-8727-2617c979ff3f
# ╠═3cf0df23-ee1e-4321-9581-91c6a6f97cc8
# ╠═d60ebc71-5641-4c90-b282-dafb6ac4fe0a
# ╠═1353a3d7-4d9d-435a-a73a-d31a00a4eb89
# ╟─9227bdd6-0fc0-4ea1-82b3-07c20d1b7b85
# ╠═13773eb4-bd8b-4a24-8bed-661ad60b9101
# ╠═cc524387-9048-4a3f-8200-a14550927bc3
# ╠═b062133e-6a7e-4df8-9415-828bf58572a9
# ╟─9a51a1c7-1421-454b-84f8-812e45aa98b2
# ╟─3455aff2-403d-468b-b378-f67fe50acce7
# ╠═0ce2b455-047d-43c0-b90c-2a0cb8c66ec6
# ╟─e7c52b1c-cc1a-4652-87c7-77845c2be361
# ╟─2235a441-fc02-4ddd-8649-3a79fa848a2c
# ╟─098daed8-7519-4b77-a629-39d49891b1ba
# ╟─c2174f2b-97bc-4040-9333-852d38f71d07
# ╠═d86a8b70-48bc-41a7-b9f7-dae0ed1ba4c6
# ╟─c22dcc99-a282-4625-8507-b64bc7749bc0
# ╟─a74aaff2-1e72-4bd4-9051-9644a4a5c3a7
# ╠═f35291b4-e439-466d-85e2-a64287d011a1
# ╠═529b2dc1-a2b9-40f2-a8c8-a90a39114bff
# ╟─e268a2df-8a90-453c-aec8-c44ef64c4c86
# ╟─3c82cf5a-864b-46c0-8843-f4b086f55953
# ╠═15e13928-5bb2-47a8-9aca-941b2689a07b
# ╠═b55c8950-bab4-4c43-bd21-2524ef64b465
# ╟─c7d38b39-401a-4997-a266-767dfcbe6895
# ╠═dcfb75d5-44ab-470d-af0f-9717baa04f8d
# ╠═a8fdefaf-5d1d-4304-9b8c-0811a9b037a7
# ╠═2e01f4b4-bb6a-4a6c-a9fa-620f3b1ea00d
# ╠═abed85c9-ccb0-403c-b911-111db23ab12f
# ╠═0b21d9a5-5f10-4f4d-bc19-c622f6908567
# ╟─565b4874-75ea-4b3c-af0b-5135c260f87e
# ╟─23591091-95ec-4059-8760-520a1d02cfa7
# ╟─3fff9fb7-4e51-4899-9580-37fc7e7e5f47
# ╠═0ea27284-bb3f-497e-8038-9a58973e2f11
# ╟─d80132e2-dece-465b-a99f-39c9d1ce069e
# ╠═61119ae6-07a7-4a84-9ca3-9839907ed8e2
# ╟─4a4b158f-8afa-4164-a238-bde8cae5dfff
# ╟─9d2d5937-c2c0-4b95-83d4-d3a459d9e48e
# ╟─d7cd03ed-f4e4-415d-8520-41ab9c427270
# ╟─210f9363-5fcb-44d8-8198-b88632d0829e
# ╠═1e62b8fa-35df-4593-b353-afc707cd877d
# ╟─10314b6d-30c0-49e2-a2b2-d7fdeb16a7a2
# ╟─bdadcf12-efa9-4f40-ac27-c83917b65ddf
# ╠═a2a846ec-3fb2-4211-9c70-30a094d4959c
# ╟─05388f6c-a6e5-4cab-9bd2-8d7020a259df
# ╟─4bc529a0-71bd-4670-bc28-f47b50d6ddfd
# ╠═ae71c271-0c27-47a7-bc49-0b71c903106d
# ╟─ced36f40-c119-4967-828f-f40a4a48fd1d
# ╟─c6aea8d4-4200-46a6-ad8d-94f4f647a150
# ╠═b95875a3-912f-4993-b248-86f9432c8f91
# ╠═18b8f949-3c34-4648-83ed-c21b6c9bad67
# ╟─2cdef132-8e7e-47cd-a866-b6235245d0ac
# ╠═056aceb1-3c37-4414-ad44-235f8d9407b8
# ╠═da373699-225b-4ffe-b89b-635027c76f98
# ╠═db9edafd-80ac-43d0-92ad-bd859d6ae0d9
# ╟─0bd91365-0c38-443c-b243-a73a2d11713a
# ╠═788ef26c-a08c-418d-b580-ca5382e236c6
# ╠═a494d531-9f41-4430-b590-82754cdc6503
# ╟─d5a661c1-55ad-42e3-b2da-3c3a810f9e2f
# ╠═164dec9e-94cb-4a8a-a26a-695ce7898a25
# ╟─6e40c591-bb91-4212-b545-863a4c194734
# ╟─6e846e60-5274-4095-b7e8-b83f13adcab4
# ╟─bf2a43fc-d50e-491d-a5f7-2b813cdb7763
# ╟─82eba233-3c4e-4f53-a6bf-58b59b694d24
# ╠═55864b4f-d748-4646-a5d3-6d0dcba004b8
# ╠═f530f422-232c-46e6-a528-729e5a314612
# ╠═5feac4f5-1ca8-47b9-a76d-cfbc10debaa8
# ╠═a4a906be-2dbc-4804-9765-5e26ba4be3c2
# ╠═200adbec-330a-48f7-9464-132f1856ea6c
# ╠═21d41d53-c905-47ed-a3c1-f798686660b6
# ╠═50f26cb0-2882-4cad-9545-5074091d7d98
# ╟─d6c36b00-5fd7-4559-b491-273772affb42
# ╟─1973f18e-e8ef-4057-b7c1-c91386a9eec5
# ╟─ab90af62-6a3f-47a2-b81a-501803ba29fb
# ╠═a80fc4fb-c8b0-49c2-b4e8-a455b9760537
# ╠═d0274275-06d9-4981-b682-9df25e5b8110
# ╠═98b9965d-91d3-4e28-be91-800e34ca8470
# ╟─1260e70e-7c23-44e8-ac23-96653fc2147f
# ╟─f8d5de2b-5277-40ed-af67-195df4d624c7
# ╠═fad4a032-e3ad-46bb-8baf-68dff4c43dfb
# ╠═73dbf146-2de4-46ea-adc3-23eb7fd31259
# ╠═e9fff9ee-7d84-434f-847d-ea5fbd317cc2
# ╠═c1d7f044-30cf-4e8e-bb59-d11ba7249444
# ╠═81a03d2c-4c04-4bc0-b81e-d1f78347e653
# ╠═2907cc8f-d1e4-4e7c-868f-e7fdf37bf0de
# ╠═f764bdc5-5215-4f19-bbe2-a8122a75f2cb
# ╠═b84000e2-504a-4f3d-bdd2-d5d0d67a021c
# ╠═a9b08017-dc98-4f50-aa70-a8d5de0b06b2
# ╠═4862029f-c4dc-4e81-928c-678ece56610e
# ╠═7bbf6647-f8a9-46d4-94d2-c8126f9cf6e4
# ╠═2862ebff-b001-425b-94f2-ab818f991f14
# ╠═1440bb19-2c18-465c-994c-8056e0f84203
# ╠═60e6d80b-eb5c-4e5a-83cf-cd9ea793173e
# ╟─c16bb000-c3f8-4b6f-9e36-4efb9166f3d8
# ╟─e64bd260-babe-4b06-94cb-e8d18b2035ad
# ╠═b2d6301f-f41c-42f8-a03b-68b443701c52
# ╟─a965fd07-b833-4c9e-87bb-f27125421a2c
# ╟─3d7ad0a2-c9f2-4f07-aa2c-bb32099be4d0
# ╟─498a345d-9126-4cd2-8ee0-b9f7bad31f85
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
