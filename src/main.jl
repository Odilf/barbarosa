module Cube3x3
	export Vector3, Piece, Cube, move, issolved, scramble, neighbours, orientation, Corners, Edges, HalfEdges, Algs

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
	export edge_permutations, corner_permutations, hash, dehash_corners, dehash_edges

	using ..Cube3x3
	using ..Search

	include("MUS/hash.jl")
	include("MUS/dehash.jl")
	include("MUS/cache.jl")
	include("MUS/generation.jl")
end

module Barbarosa
	using ..Cube3x3
	using ..MUS
	using ..Search
end