using StaticArrays

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

# Corner hash
function Base.hash(corners::Corners)
	piece_hash = map(corners) do (pos, _)
		findfirst(pair -> pair.second.position == pos, corners)
	end

	orientations = map(enumerate(corners[1:end-1])) do (i, (pos, piece))
		i = i - 1
		index = orientation(pos, piece.normal)
		index * 3^i
	end

	# Stuff to get the number (1 indexed)
	permutations_hash(SVector{8}(piece_hash), max=8) + sum(orientations) * factorial(8) + 1
end

# Edge hash
function Base.hash(edges::Edges)
	halves = [SVector{6}(edges[1:6]), SVector{6}(edges[7:12])]

	map(halves) do half
		piece_hash = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1 # 0 indexing, makes my life easier
			index = findfirst(pair -> pair.first == piece.position, edges)
		end

		orientations = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1
			index = orientation(pos, piece.normal)
			index * 2^i
		end

		# Stuff to get the number (1 indexed)
		permutations_hash(SVector{6}(piece_hash), max=12) * 2^6 + sum(orientations) + 1
	end
end

function Base.hash(cube::Cube)
	[hash(cube |> corners); hash(cube |> edges)]
end

function permutations_hash(vector::SVector{N, <:Integer} where N; max::Integer)
	fmax = factorial(max)
	map(enumerate(vector)) do (i, n)
		i = i - 1
		n = n - sum(vector[1:i] .< n) - 1
		n * fmax ÷ factorial(max - i)
	end |> sum
end

function generate_corners(hash::Integer)
	hash -= 1
	permutations_hash = hash % factorial(8)
	orientation_hash = hash ÷ factorial(8)

	orientations = map(1:8) do i
		(orientation_hash % 3^i) ÷ 3^(i - 1)
	end

	permutations = map(0:7) do i
		(permutations_hash % (factorial(8) ÷ factorial(8 - i - 1))) ÷ (factorial(8) ÷ factorial(8 - i)) + 1
	end

	permutations = let
		# permutations = [permutations; 1]
		result = []
		options = collect(1:8)
		for decision in permutations
			push!(result, options[decision])
			deleteat!(options, decision)
		end
		result
	end

	c = cube() |> corners
	map(zip(permutations, orientations, c)) do (p, o, (_, piece))
		pair = c[p].first => piece
		Cube3x3.twist(pair, o)
	end
end

# let 
# 	c =  move(cube(), "F") |> corners
# 	h = c |> hash
# 	f = h |> generate_corners

# 	f
# end