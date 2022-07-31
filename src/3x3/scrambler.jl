using Random

isedge(piece::Piece) = sum(abs.(piece.position)) == 2

# Kinda dangerous
corners(cube::Cube)::SVector{8, Pair{Vector3, Piece}} = cube[1:8]
edges(cube::Cube)::SVector{12, Pair{Vector3, Piece}} = cube[9:20]

filter(pair -> pair.first == 2, cube())

# I'm so proud of this
function orientation(position::Vector3, normal::Vector3)
	# Represent "loop"
	mask = position .!= 0 
	p = position[mask]
	n = normal[mask]

	# Get parity. If it is negative, the loop is inversed
	parity = reduce(*, p)

	# Count spaces of separation
	if findfirst(i -> abs(i) == 1, n) |> isnothing
		error("What $position, $normal")
	end
	index = findfirst(i -> abs(i) == 1, n) - 1
	
	# Mod it and apply parity
	n = length(p)
	mod((n - index) * parity, n)
end

orientation(pair::Pair{Vector3, Piece}) = orientation(pair.first, pair.second.normal)

function orientation(cube::HashSet)
	edges = 0
	corners = 0

	for (pos, piece) in cube
		n = orientation(pos, piece.normal)
		if isedge(piece)
			edges += n
		else
			corners += n
		end
	end

	(edges = edges, corners = corners)
end

orientation(cube::Vector{Pair{Vector3, Piece}}) = orientation(SVector(cube...))

function isoriented(cube::Cube)
	(e, c) = orientation(cube)
	e % 2 == 0 && c % 3 == 0
end

function randomorientation(piece::Piece, position::Vector3)
	mask = position .!= 0
	i = collect(1:3)[mask] |> rand
	n = [0, 0, 0]
	n[i] = position[i]
	Piece(piece.position, n)
end

function randomize(input::T)::T where {T <: HashSet}
	positions = shuffle([pos for (pos, _) in input])

	map(zip(positions, input)) do (pos, (_, piece))
		pos => randomorientation(piece, pos)
	end |> SVector
end

function twist(normal::Vector3, position::Vector3, n::Integer = 1)::Vector3
	if n <= 0
		return normal
	end

	parity = reduce(*, filter(v -> v != 0, position))
	i = findfirst(x -> abs(x) == 1, normal)

	for _ in 1:3
		i = mod(i - 1 + parity, 3) + 1

		if (position[i] != 0)
			normal = [0, 0, 0]
			normal[i] = position[i]

			return twist(v(normal...), position, n - 1)
		end
	end

	error("Unreachable, in theory")
end

function twist(pair::Pair{Vector3, Piece}, n::Integer = 1)::Pair{Vector3, Piece}
	(pos, piece) = pair
	pos => Piece(piece.position, twist(piece.normal, pos, n))
end

# Ew
function swaps(v::SVector{N, <:Integer}) where N
	v = MVector(v)
	total = 0
	for i in eachindex(v)
		n = v[i]
		j = findfirst(n -> n == i, v)

		if j != i
			total += 1
			v[j], v[i] = v[i], v[j]
		end
	end

	return total
end

function scramble()::Cube
	e = randomize(cube() |> edges) |> Vector
	c = randomize(cube() |> corners) |> Vector
	(eo, _) = orientation(e)
	(_, co) = orientation(c)

	# Flip edge if orientation is incorrect
	if eo % 2 != 0
		i = 1
		(pos, piece) = e[i]
		e[i] = pos => Piece(piece.position, twist(piece.normal, v(pos...)))
	end

	# Twist corner if orientation is incorrect
	dif = co % 3
	if dif != 0
		i = 1
		(pos, piece) = c[i]
		c[i] = pos => Piece(piece.position, twist(piece.normal, v(pos...), dif))
	end
	
	# Swap if swap parity is incorrect
	if swaps(permutations(c)) % 2 != swaps(permutations(e)) % 2
		e[1], e[2] = e[2].first => Piece(e[1].second.position, e[2].second.normal), e[1].first => Piece(e[2].second.position, e[1].second.normal)
	end

	SVector(c..., e...)
end

function permutations(cube::SVector{N, Pair{Vector3, Piece}}, search_pool=cube)::SVector{N, <:Integer} where N
	map(cube) do (pos, _)
		findfirst(pair -> pair.second.position == pos, search_pool)
	end
end

permutations(cube::Vector{Pair{Vector3, Piece}}) = permutations(SVector(cube...))