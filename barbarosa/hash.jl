using .Cube3x3
using StaticArrays

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

# Corner hash
function Base.hash(cube::SVector{8, Pair{Vector3, Piece}})
	piece_hash = map(enumerate(cube)) do (i, (pos, piece))
		i = i - 1 # 0 indexing, makes my life easier
		index = findfirst(pair -> pair.first == piece.position, cube)
	end

	orientations = map(enumerate(cube[1:end-1])) do (i, (pos, piece))
		i = i - 1
		index = orientation(pos, piece.normal)
		index * 3^i
	end

	# Stuff to get the number
	permutations_hash(SVector{8}(piece_hash)) + sum(orientations) * factorial(8)
end

# Edge hash
function Base.hash(cube::SVector{12, Pair{Vector3, Piece}})
	halves = [SVector{6}(cube[1:6]), SVector{6}(cube[7:12])]

	map(halves) do half
		piece_hash = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1 # 0 indexing, makes my life easier
			index = findfirst(pair -> pair.first == piece.position, cube)
		end

		orientations = map(enumerate(half)) do (i, (pos, piece))
			i = i - 1
			index = orientation(pos, piece.normal)
			index * 2^i
		end

		# Stuff to get the number
		permutations_hash(SVector{6}(piece_hash), min=6) * 2^6 + sum(orientations)
	end
end

function Base.hash(cube::Cube)
	[hash(cube |> corners), hash(cube |> edges)...]
end

function permutations_hash(vector::SVector{N, <:Integer} where N; min = 0)
	l = length(vector)
	map(enumerate(vector)) do (i, n)
		i = i
		n -= min
		n -= sum(vector[i + 1:end] .< n)
		(n - 1) * factorial(i - 1)
	end |> sum
end