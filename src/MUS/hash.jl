using StaticArrays

Cube3x3.permutations(elements::Integer, choose::Integer) = reduce(*, (elements - choose + 1):elements)

function hash_permutations(vector::SVector{N, <:Integer}; max::Integer=N)::Integer where N
	map(enumerate(vector)) do (i, n)
		i = i - 1 # 0-indexing

		# Decision (Eg.: if we have selected 1 and 3, 4 is the second decision)
		n = n - sum(vector[1:i] .< n) - 1

		n * permutations(max, i) # Encode decision
	end |> sum
end

function hash_orientations(orientations::SVector{N, <:Integer}, modulus::Integer)::Integer where N
	map(enumerate(orientations)) do (i, orientation)
		orientation * modulus^(i - 1)
	end |> sum
end

hash_orientations(o::Vector{<:Integer}, m::Integer) = hash_orientations(SVector(o...), m)

function Base.hash(corners::Corners)::Integer
	permutation_hash = hash_permutations(permutations(corners; pool=Corners()); max=8)
	orientation_hash = hash_orientations(orientation.(corners.pieces[1:end-1]), 3)

	# Stuff to get the number (1 indexed)
	return permutation_hash + orientation_hash * factorial(8) + 1
end

function Base.hash(half::HalfEdges; pool::Cube=Edges())::Integer
	permutation_hash = hash_permutations(permutations(half; pool); max=12)
	orientation_hash = hash_orientations(orientation.(half.pieces), 2)

	# Stuff to get the number (1 indexed)
	return permutation_hash * 2^6 + orientation_hash + 1
end

const second_edges = Edges([Edges().pieces[7:12]..., Edges().pieces[1:6]...])

function Base.hash(edges::Edges)::Tuple{Int64, Int64}
	return (
		hash(HalfEdges(edges.pieces[1:6]), pool=Edges()), 
		hash(HalfEdges(edges.pieces[7:12]), pool=second_edges)
	)
end

function Base.hash(cube::Cube{20})::Tuple{Int64, Int64, Int64}
	(hash(Corners(cube)), hash(Edges(cube))...)
end