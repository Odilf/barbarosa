using Dates

function cache_by_depth(cache::Vector{UInt8}, cube::Cube{N}, threshold::Integer, moves::Integer = 0) where N
	for connection ∈ neighbouring_moves
		if moves < threshold
			cache = cache_by_depth(cache, move(cube, connection.moves), threshold, moves + connection.cost)
		end
	end

	i = hash(cube)
	v = cache[i]
	if v > moves
		cache[i] = moves
	end

	cache
end

CacheSet = Union{Type{Corners}, Type{Edges}}

function cache_by_depth(depth::Integer, set::CacheSet)
	cache = cache_by_depth(getcache(set), set == Edges ? HalfEdges() : set(), depth)
	savecache(cache, set)
end

function mus_heuristic(set::Set, cache::Vector{UInt8}; fallback) where {Set <: CacheSet}
	if length(cache) != permutations(set)
		error("Incorrect cache passed to function (it is $(length(cache)) instead of $(permutations(set))")
	end

	function heuristic(cube)
		cached = cache[hash(cube)]
		if cached != 0xff
			return cached
		else
			return fallback(cube)
		end
	end
end

function cache_by_hash(set::CacheSet, cache::Vector{UInt8}, range::AbstractRange; fallback, silent=true, IDA_kwargs...)
	heuristic = mus_heuristic(set, cache; fallback)

	for i ∈ range
		if cache[i] != 0xff
			println("Skipping caching hash $i because it is cached already ($(cache[i]))")
		else
			println("Caching $i")
			state = dehash(i, set)
			solution = IDAstar(state, heuristic; silent, IDA_kwargs...)
			v = UInt8(length(solution))
			cache[i] = v
			@info "Cached hash $i to $(v)!"
		end
	end

	cache
end

function cache_by_hash(set::CacheSet, range::AbstractRange; kwargs...)
	cache = cache_by_hash(set, getcache(set), range; kwargs...)
	savecache(cache, set)
end

function cache_by_hash(set::CacheSet; limit::Integer=permutations(set), IDAkwargs...)
	start_time = Dates.format(now(), "HH:MM:SS")
	@info "Started at $start_time"

	cache = getcache(set)
	range = first_uncached(cache):limit
	fallback = _ -> max(cache) + 1
	try
		cache = cache_by_hash(set, cache, range; fallback, IDAkwargs...)
	finally
		savecache(cache, set)
	end
end