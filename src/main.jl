module Cube3x3
	export Vector3, Piece, Cube, cube, Move, move, issolved, scramble, neighbours, corners, edges, orientation, Corners, Edges, HalfEdges, HashSet, Algs

	include("3x3/3x3.jl")
	include("3x3/scrambler.jl")
	include("3x3/algs.jl")
end

module Search
	export IDAstar, manhattan, reconstruct_solution

	using ..Cube3x3

	include("search/heuristics.jl")
	include("search/IDA*.jl")
end

module MUS
	export edge_permutations, corner_permutations, hash

	using ..Cube3x3
	using ..Search

	include("MUS/hash.jl")
	include("MUS/mus_cache.jl")
end

module Barbarosa
	using ..Cube3x3
	using ..MUS
	using ..Search
end