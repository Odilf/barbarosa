using StaticArrays

const symmetry_matrices = let 
	v = Vector{SMatrix{3, 3, Int}}()
	for i ∈ [-1, 1]
		for j ∈ [-1, 1]
			for k ∈ [-1, 1]
				push!(v, 
					@SMatrix[i 0 0; 0 j 0; 0 0 k],
					@SMatrix[0 j 0; 0 0 k; i 0 0],
					@SMatrix[0 0 k; i 0 0; 0 j 0],

					@SMatrix[i 0 0; 0 0 k; 0 j 0],
					@SMatrix[0 0 k; 0 j 0; i 0 0],
					@SMatrix[0 j 0; i 0 0; 0 0 k],
				)
			end
		end
	end
	v
end

struct DeltaPiece
	id::Vector3
	Δ::Vector3
	orientation::Integer
end

DeltaPiece(piece::Piece) = DeltaPiece(piece.id, piece.position - piece.id, orientation(piece))

function transform(piece::DeltaPiece, matrix::SMatrix{3, 3, Int})
	id = matrix * piece.id
	position = id + matrix * piece.Δ
	Piece(id, position, piece.orientation)
end

function transform(cube::C, matrix::SMatrix{3, 3, Int}) where C
	map(cube.pieces) do piece
		transform(DeltaPiece(piece), matrix)
	end |> C
end

function Base.sort(cube::C) where C <: Cube
	map(C().pieces) do sorted
		index = findfirst(piece -> piece.id == sorted.id, cube.pieces)
		cube.pieces[index]
	end |> C
end

function symmetries(cube::C) where C <: Cube
	delta_cube = map(DeltaPiece, cube.pieces)

	map(symmetry_matrices) do m
		pieces = map(piece -> transform(piece, m), delta_cube)
		SVector(pieces...) |> C |> sort
	end
end

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
	permutation_hash + orientation_hash * factorial(8) + 1
end

function Base.hash(half::HalfEdges)::Integer
	permutation_hash = hash_permutations(permutations(half; pool=Edges()); max=12)
	orientation_hash = hash_orientations(orientation.(half.pieces), 2)

	# Stuff to get the number (1 indexed)
	permutation_hash * 2^6 + orientation_hash + 1
end

function Base.hash(edges::Edges)::Tuple{Int64, Int64}
	hash(HalfEdges(edges.pieces[1:6])), hash(HalfEdges(edges.pieces[7:12]))
end

function symmetryhashes(cube::C) where C <: Cube
	map(symmetries(cube)) do scube
		hash(scube)
	end
end

symmetryhash(cube::Cube) = symmetryhashes(cube) |> minimum