# MUS stands for "Moves until solved"

using Base.Threads
using StaticArrays

Cache = Vector{UInt8}

const extension = ".barbarosa"

@enum PieceType edge corner

function getpath(type::PieceType)
	folder = "src/MUS"
	folder * (type == corner ? "/corners" : "/edges") * extension
end

resetcache() = savecache(zeros(UInt8, corner_permutations), zeros(UInt8, edge_permutations))

function getcache()::Tuple{Cache, Cache}
	read(getpath(corner)), read(getpath(edge))
end

corner_cache, edge_cache = getcache()

assigncache() = ((corner_cache, edge_cache) = getcache())

function savecache(corner_cache::Cache, edge_cache::Cache)
	write(getpath(corner), corner_cache)
	write(getpath(edge), edge_cache)
end

savecache() = savecache(corner_cache, edge_cache)

function cacheprogress(corner_cache::Cache, edge_cache::Cache)
	c_count = count(n -> n != 0, corner_cache)
	c = c_count / corner_permutations * 100

	e_count = count(n -> n != 0, edge_cache)
	e = e_count / edge_permutations * 100
	"Corners: $c% ($c_count/$corner_permutations). Edges: $e% ($e_count/$edge_permutations)"
end

cacheprogress() = cacheprogress(corner_cache, edge_cache)

function generate_cache(state::Cube, moves::UInt8, threshold::Integer)::Nothing
	if threshold <= 0
		return
	end

	for neighbour in neighbours(state)
		generate_cache(neighbour, UInt8(moves + 1), threshold - 1)
	end

	(c, e1, e2) = hash(state)

	corner_cache[c] = moves
	edge_cache[e1] = moves
	edge_cache[e2] = moves

	return
end

function generate_corner_cache(corner_cache::Vector{UInt8}; range = 1:100)
	

	for i in range
	end
end