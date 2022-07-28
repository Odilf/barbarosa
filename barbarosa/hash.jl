using .Cube3x3
using StaticArrays

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

# Corner hash
function Base.hash(corners::Corners)
	piece_hash = map(enumerate(corners)) do (i, (pos, piece))
		i = i - 1 # 0 indexing, makes my life easier
		index = findfirst(pair -> pair.first == piece.position, corners)
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