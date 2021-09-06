### A Pluto.jl notebook ###
# v0.15.1

using Markdown
using InteractiveUtils

# ╔═╡ 01e2e5fc-3f22-494b-8f51-43db0f788041
using HDF5

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
function constructpiece(vector::Vector{Int}, orientation = 0, position = nothing)
	position == nothing && (position = vector)
	Piece(vector, position, orientation)
end

# ╔═╡ f9a24873-4e03-4648-b966-e9aceb8b4d58
function constructpiece(name::String, orientation = 0, position = nothing)
	constructpiece(sum(v -> vectorofface(v), name), orientation, position)
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
Cube = Array{Piece}

# ╔═╡ 6ee79fca-e240-446a-8c66-53820ee965c2
md"""
A cube has 12 edges and 8 corners. This number is small enough to make it reasonable to manualy build the solved cube. 
"""

# ╔═╡ 41600c68-045e-4443-a187-601ff9186877
const pieces = ["RU", "RF", "RD", "RB", 
				"UF", "DF", "DB", "UB", 
				"LU", "LF", "LD", "LB",
	
				"RUF", "LUF", "RDF", "LDF", 
				"RUB", "LUB", "RDB", "LDB"]

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
	
	if piecetype(piece) == "Corner" && axis ≠ 3 && amount ≠ 2
		corner_parity = reduce(*, piece.position)
		offset = 2
		if corner_parity == -1
			offset = 1
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
name([1, -1, 1])

# ╔═╡ 05e59cf0-e71b-4efb-a2e0-443df1f23d34
name(constructpiece("RUB"))

# ╔═╡ 3942e7d2-1a0c-438b-ada6-c245a67ccfa5
name.(filter(piece -> pieceinplane(piece, 'B'), solved_cube))

# ╔═╡ 3b9ffb15-86ec-4ca2-9e04-0c8eadc61c54
name(rotate(constructpiece("RU"), 1, 1))

# ╔═╡ 6dd34804-f815-4fa4-995f-d6b8cf2a4ce1
function move(cube::Cube, move_name::String)
	move(cube, getmove(move_name))
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
move(solved_cube, "R2")

# ╔═╡ 04569bda-1b60-4c71-b7ca-54eba379264b
sexy_move = ["R", "U", "R'", "U'"]

# ╔═╡ 5e66352f-3989-4807-b295-a0cf31aae2d0
sexy_cube = move(solved_cube, sexy_move)

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

# ╔═╡ 9a51a1c7-1421-454b-84f8-812e45aa98b2
md"""
We now have a fully functional digital cube.
"""

# ╔═╡ d18076fa-2376-4f85-bfb3-4062f01176ac
issolvable(cube) = true

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
map(v -> name(v[1]), getmoves())

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

# ╔═╡ b0e10764-4df6-4822-b676-5d8af215cd5c
begin
	caca = 0
	for i in 1:20
		if i > 3
			continue
		end
		caca = i
	end
	caca
end

# ╔═╡ 0bd91365-0c38-443c-b243-a73a2d11713a
md"""
Also a function to return a nice string with the solution would be useful, since moves are added to the begging of the solution array. 
"""

# ╔═╡ 6e846e60-5274-4095-b7e8-b83f13adcab4
md"""
## Rudimentary heuristic calculation

In the context of solving a Rubik's cube, a heuristic should approximate the number of moves left to solve the cube. For the heuristic to be admissible (that is, it guarantees the IDA* to find the optimal solution) it has to never surpass the actual number of moves left. 

A rudimentary heuristic could be based on Manhattan distance, which is easy to implement. However it is not very accurate. 

The most common approach is to save big tables of solutions for specific pieces on the cube and take the maximunm. The groups are the corners, and each two halves of the edges. 

While it takes some computational effort to calculate these tables, the cost of looking up a heuristic is practically instant once it is done. The size is not nearly big enough to pose a challenge to store the values in memory (the size is calculated in the appendix).

"""

# ╔═╡ cbce1f2e-6869-4918-b5bc-bfd4ad275e86
md"""
We will store the table using the HDF5 format and julia library
"""

# ╔═╡ 8f075ce2-f0a2-407e-b737-7602278f2d07


# ╔═╡ 56600142-c703-420f-9b9b-39d65cb6588c
function heuristic(cube::Cube)
	1
end

# ╔═╡ 18b8f949-3c34-4648-83ed-c21b6c9bad67
function search(cube, g, threshold, solution, new_threshold)
	
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
		new_solution, new_theshold = search(
			move(cube, current_move),
			g + path_cost,
			threshold,
			[current_move, solution...],
			new_threshold
		)
		
		if new_solution ≠ nothing
			return new_solution, new_threshold
		end
	end
	return nothing, 69420
end

# ╔═╡ b95875a3-912f-4993-b248-86f9432c8f91
function IDAsolve(cube, limit = 10)
	if !issolvable(cube)
		error("Cube is unsolvable")
	end

	threshold = heuristic(cube)
	new_threshold = threshold
	
	for i in 1:limit
		solution, new_threshold = search(cube, 0, threshold, [], Inf)
		
		if solution ≠ nothing
			return solution.reverse()
		end
		
		threshold = new_threshold
	end
	
	error("No solution found with iteration limit $limit")
end

# ╔═╡ da373699-225b-4ffe-b89b-635027c76f98
begin
	sexy_solution = IDAsolve(sexy_cube, 2)
	name(sexy_solution)
end

# ╔═╡ 4d99d8b7-814a-40be-b563-44f367321cdf
prettyprint(sexy_solution)

# ╔═╡ a494d531-9f41-4430-b590-82754cdc6503
prettyprint(IDAsolve(move(solved_cube, ["R", "L", "U", "D2", "F'", "U2"])))

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
#### Corners
There are $3$ ways to orient any corner and $8$ places to place them. Because of parity, we know the orientation of the last corner given the other seven, so we have $3^7$ possible corner orientations. 

In a solved cube, the same happens with the positions. However we are not taking into account the edges, so any corner culd be in any position, which gives us $8!$ combinations for corners. 

We multiply them together to get $3^7\cdot8! = 88\,179\,840$ possible combinations. Storing each number as a byte, we have just over 88 megabytes. 

Similarly with 6 of the edges, there are 2 ways to orient them and 12 places to put them, so we get $2^6 \cdot 12!/6!= 42577920$ which is some 42.5 megabytes. 

The final size totals just over 130 megabytes by storing each heuristic as a byte. 
"""

# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
HDF5 = "f67ccb44-e63f-5c2f-98bd-6dc0ccc4ba2f"
PlutoUI = "7f904dfe-b85e-4ff6-b463-dae2292396a8"

[compat]
HDF5 = "~0.15.6"
PlutoUI = "~0.7.9"
"""

# ╔═╡ 00000000-0000-0000-0000-000000000002
PLUTO_MANIFEST_TOML_CONTENTS = """
# This file is machine-generated - editing it directly is not advised

[[ArgTools]]
uuid = "0dad84c5-d112-42e6-8d28-ef12dabb789f"

[[Artifacts]]
uuid = "56f22d72-fd6d-98f1-02f0-08ddc0907c33"

[[Base64]]
uuid = "2a0f44e3-6c83-55bd-87e4-b1978d98bd5f"

[[Blosc]]
deps = ["Blosc_jll"]
git-tree-sha1 = "84cf7d0f8fd46ca6f1b3e0305b4b4a37afe50fd6"
uuid = "a74b3585-a348-5f62-a45c-50e91977d574"
version = "0.7.0"

[[Blosc_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Lz4_jll", "Pkg", "Zlib_jll", "Zstd_jll"]
git-tree-sha1 = "e747dac84f39c62aff6956651ec359686490134e"
uuid = "0b7ba130-8d10-5ba8-a3d6-c5182647fed9"
version = "1.21.0+0"

[[Compat]]
deps = ["Base64", "Dates", "DelimitedFiles", "Distributed", "InteractiveUtils", "LibGit2", "Libdl", "LinearAlgebra", "Markdown", "Mmap", "Pkg", "Printf", "REPL", "Random", "SHA", "Serialization", "SharedArrays", "Sockets", "SparseArrays", "Statistics", "Test", "UUIDs", "Unicode"]
git-tree-sha1 = "727e463cfebd0c7b999bbf3e9e7e16f254b94193"
uuid = "34da2185-b29b-5c13-b0c7-acf172513d20"
version = "3.34.0"

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

[[HDF5]]
deps = ["Blosc", "Compat", "HDF5_jll", "Libdl", "Mmap", "Random", "Requires"]
git-tree-sha1 = "83173193dc242ce4b037f0263a7cc45afb5a0b85"
uuid = "f67ccb44-e63f-5c2f-98bd-6dc0ccc4ba2f"
version = "0.15.6"

[[HDF5_jll]]
deps = ["Artifacts", "JLLWrappers", "LibCURL_jll", "Libdl", "OpenSSL_jll", "Pkg", "Zlib_jll"]
git-tree-sha1 = "fd83fa0bde42e01952757f01149dd968c06c4dba"
uuid = "0234f1f7-429e-5d53-9886-15a909be8d59"
version = "1.12.0+1"

[[InteractiveUtils]]
deps = ["Markdown"]
uuid = "b77e0a4c-d291-57a0-90e8-8db25a27a240"

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

[[LinearAlgebra]]
deps = ["Libdl"]
uuid = "37e2e46d-f89d-539d-b4ee-838fcccc9c8e"

[[Logging]]
uuid = "56ddb016-857b-54e1-b83d-db4d58db5568"

[[Lz4_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "5d494bc6e85c4c9b626ee0cab05daa4085486ab1"
uuid = "5ced341a-0733-55b8-9ab6-a4889d929147"
version = "1.9.3+0"

[[Markdown]]
deps = ["Base64"]
uuid = "d6f4376e-aef5-505a-96c1-9c027394607a"

[[MbedTLS_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "c8ffd9c3-330d-5841-b78e-0817d7145fa1"

[[Mmap]]
uuid = "a63ad114-7e13-5084-954f-fe012c677804"

[[MozillaCACerts_jll]]
uuid = "14a3606d-f60d-562e-9121-12d972cd8159"

[[NetworkOptions]]
uuid = "ca575930-c2e3-43a9-ace4-1e988b2c1908"

[[OpenSSL_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "15003dcb7d8db3c6c857fda14891a539a8f2705a"
uuid = "458c3c95-2e84-50aa-8efc-19380b2a3a95"
version = "1.1.10+0"

[[Parsers]]
deps = ["Dates"]
git-tree-sha1 = "438d35d2d95ae2c5e8780b330592b6de8494e779"
uuid = "69de0a69-1ddd-5017-9359-2bf0b02dc9f0"
version = "2.0.3"

[[Pkg]]
deps = ["Artifacts", "Dates", "Downloads", "LibGit2", "Libdl", "Logging", "Markdown", "Printf", "REPL", "Random", "SHA", "Serialization", "TOML", "Tar", "UUIDs", "p7zip_jll"]
uuid = "44cfe95a-1eb2-52ea-b672-e2afdf69b78f"

[[PlutoUI]]
deps = ["Base64", "Dates", "InteractiveUtils", "JSON", "Logging", "Markdown", "Random", "Reexport", "Suppressor"]
git-tree-sha1 = "44e225d5837e2a2345e69a1d1e01ac2443ff9fcb"
uuid = "7f904dfe-b85e-4ff6-b463-dae2292396a8"
version = "0.7.9"

[[Preferences]]
deps = ["TOML"]
git-tree-sha1 = "00cfd92944ca9c760982747e9a1d0d5d86ab1e5a"
uuid = "21216c6a-2e73-6563-6e65-726566657250"
version = "1.2.2"

[[Printf]]
deps = ["Unicode"]
uuid = "de0858da-6303-5e67-8744-51eddeeeb8d7"

[[REPL]]
deps = ["InteractiveUtils", "Markdown", "Sockets", "Unicode"]
uuid = "3fa0cd96-eef1-5676-8a61-b3b8758bbffb"

[[Random]]
deps = ["Serialization"]
uuid = "9a3f8284-a2c9-5f02-9a11-845980a1fd5c"

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

[[Serialization]]
uuid = "9e88b42a-f829-5b0c-bbe9-9e923198166b"

[[SharedArrays]]
deps = ["Distributed", "Mmap", "Random", "Serialization"]
uuid = "1a1011a3-84de-559e-8e89-a11a2f7dc383"

[[Sockets]]
uuid = "6462fe0b-24de-5631-8697-dd941f90decc"

[[SparseArrays]]
deps = ["LinearAlgebra", "Random"]
uuid = "2f01184e-e22b-5df5-ae63-d93ebab69eaf"

[[Statistics]]
deps = ["LinearAlgebra", "SparseArrays"]
uuid = "10745b16-79ce-11e8-11f9-7d13ad32a3b2"

[[Suppressor]]
git-tree-sha1 = "a819d77f31f83e5792a76081eee1ea6342ab8787"
uuid = "fd094767-a336-5f1f-9728-57cf17d0bbfb"
version = "0.2.0"

[[TOML]]
deps = ["Dates"]
uuid = "fa267f1f-6049-4f14-aa54-33bafae1ed76"

[[Tar]]
deps = ["ArgTools", "SHA"]
uuid = "a4e569a6-e804-4fa4-b0f3-eef7a1d5b13e"

[[Test]]
deps = ["InteractiveUtils", "Logging", "Random", "Serialization"]
uuid = "8dfed614-e22c-5e08-85e1-65c5234f0b40"

[[UUIDs]]
deps = ["Random", "SHA"]
uuid = "cf7118a7-6976-5b1a-9a39-7adc72f591a4"

[[Unicode]]
uuid = "4ec0a83e-493e-50e2-b9ac-8f72acf5a8f5"

[[Zlib_jll]]
deps = ["Libdl"]
uuid = "83775a58-1f1d-513f-b197-d71354ab007a"

[[Zstd_jll]]
deps = ["Artifacts", "JLLWrappers", "Libdl", "Pkg"]
git-tree-sha1 = "cc4bf3fdde8b7e3e9fa0351bdeedba1cf3b7f6e6"
uuid = "3161d3a3-bdf6-5164-811a-617609db77b4"
version = "1.5.0+0"

[[nghttp2_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "8e850ede-7688-5339-a07c-302acd2aaf8d"

[[p7zip_jll]]
deps = ["Artifacts", "Libdl"]
uuid = "3f19e933-33d8-53b3-aaab-bd5110c3b7a0"
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
# ╠═f9a24873-4e03-4648-b966-e9aceb8b4d58
# ╠═f84832a1-1fe9-4df7-a813-a4be7a079d83
# ╟─7b02de6a-20b2-411a-b374-aa461a527109
# ╠═6f0c7aa8-392d-42c9-9638-55ef53e9aa25
# ╠═44787f95-0fc3-4afd-b682-0b8eb8e66d4b
# ╠═05e59cf0-e71b-4efb-a2e0-443df1f23d34
# ╟─5045dcc9-de20-4d6e-9a60-31b633f61d11
# ╠═fe09f584-9be3-42bc-bc61-b4e9cc1dd700
# ╟─6ee79fca-e240-446a-8c66-53820ee965c2
# ╠═41600c68-045e-4443-a187-601ff9186877
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
# ╠═3f5b9fc4-abfa-4d14-b846-1becb0a9bbbb
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
# ╠═5e66352f-3989-4807-b295-a0cf31aae2d0
# ╠═6fe5470e-4c34-4fc2-97d1-a6c897253019
# ╟─5b7c7056-b147-4d39-99b6-46d080c746c1
# ╠═02ec5f2a-ece5-4860-aa02-470e129335fc
# ╠═842f3cdb-a66e-450a-b579-bd3bd3948210
# ╠═21f5b811-76d1-441d-bb29-ee7a4aa3dc0e
# ╠═caae487b-7942-452f-81c4-a290d8205271
# ╟─9a51a1c7-1421-454b-84f8-812e45aa98b2
# ╠═d18076fa-2376-4f85-bfb3-4062f01176ac
# ╟─9d2d5937-c2c0-4b95-83d4-d3a459d9e48e
# ╟─d7cd03ed-f4e4-415d-8520-41ab9c427270
# ╟─210f9363-5fcb-44d8-8198-b88632d0829e
# ╠═1e62b8fa-35df-4593-b353-afc707cd877d
# ╠═bdadcf12-efa9-4f40-ac27-c83917b65ddf
# ╠═ae71c271-0c27-47a7-bc49-0b71c903106d
# ╠═a2a846ec-3fb2-4211-9c70-30a094d4959c
# ╠═05388f6c-a6e5-4cab-9bd2-8d7020a259df
# ╠═4bc529a0-71bd-4670-bc28-f47b50d6ddfd
# ╠═ced36f40-c119-4967-828f-f40a4a48fd1d
# ╟─c6aea8d4-4200-46a6-ad8d-94f4f647a150
# ╠═b95875a3-912f-4993-b248-86f9432c8f91
# ╠═18b8f949-3c34-4648-83ed-c21b6c9bad67
# ╠═da373699-225b-4ffe-b89b-635027c76f98
# ╠═b0e10764-4df6-4822-b676-5d8af215cd5c
# ╟─0bd91365-0c38-443c-b243-a73a2d11713a
# ╠═4d99d8b7-814a-40be-b563-44f367321cdf
# ╠═a494d531-9f41-4430-b590-82754cdc6503
# ╟─6e846e60-5274-4095-b7e8-b83f13adcab4
# ╟─cbce1f2e-6869-4918-b5bc-bfd4ad275e86
# ╠═01e2e5fc-3f22-494b-8f51-43db0f788041
# ╠═8f075ce2-f0a2-407e-b737-7602278f2d07
# ╠═56600142-c703-420f-9b9b-39d65cb6588c
# ╟─c16bb000-c3f8-4b6f-9e36-4efb9166f3d8
# ╟─e64bd260-babe-4b06-94cb-e8d18b2035ad
# ╠═b2d6301f-f41c-42f8-a03b-68b443701c52
# ╠═a965fd07-b833-4c9e-87bb-f27125421a2c
# ╟─3d7ad0a2-c9f2-4f07-aa2c-bb32099be4d0
# ╟─498a345d-9126-4cd2-8ee0-b9f7bad31f85
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
