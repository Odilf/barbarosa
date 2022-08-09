function cache_by_depth(cache::Vector{UInt8}, cube::Cube{N}, threshold::Integer, moves::Integer = 0)where N
	for connection in neighbouring_moves
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

function cache_corners_by_depth(depth::Integer)
	corner_cache = cache_by_depth(getcache().corners, Corners(), depth)
	cache = getcache()
	cache.corners = corner_cache
	savecache(cache)
end

function cache_edges_by_depth(depth::Integer)
	edge_cache = cache_by_depth(getcache().edges, HalfEdges(), depth)
	cache = getcache()
	cache.edges = edge_cache
	savecache(cache)
end

function mus_corner_heuristic(cache::Vector{UInt8}; fallback=manhattan)
	if length(cache) != corner_permutations
		error("Incorrect cache passed to function (it is $(length(cache)) instead of $corner_permutations")
	end

	function heuristic(cube::Corners)
		cached = cache[hash(cube)]
		if cached != 0xff
			# println("Is cached")
			return cached
		else
			f = fallback(cube)
			# println("Fallback is $f")
			return f
		end
	end

	return heuristic
end

function cache_corners(cache::Cache, range::AbstractRange; fallback=cube->manhattan(cube)*10, kwargs...)
	heuristic = mus_corner_heuristic(cache.corners; fallback)

	for i in range
		if cache.corners[i] != 0xff
			println("Skipping caching hash $i")
		else
			state = dehash_corners(i)
			solution = IDAstar(state, heuristic; kwargs...)
			cache.corners[i] = UInt8(length(solution))
			@info "Cached hash $(i)!"
		end
	end

	@warn "REMEMBER TO SAVE CACHE!"

	cache
end

cache_corners(range::AbstractRange; kwargs...) = cache_corners(getcache().corners)

savecache(getcache().corners, Edges)

function mus_edge_heuristic(cache::Vector{UInt8}, fallback=manhattan)
	if length(cache) != edge_permutations
		error("Incorrect cache passed to function (it is $(length(cache)) instead of $edge_permutations")
	end

	function heuristic(cube::HalfEdges)
		cached = cache[hash(cube)]
		if cached != 0xff
			return cached
		else
			return fallback(cube)
		end
	end

	return heuristic
end

function cache_edges(cache::Cache, range::AbstractRange; kwargs...)
	heuristic = mus_edge_heuristic(cache.edges)

	for i in range
		if cache.edges[i] != 0xff
			println("Skipping caching hash $i")
		else
			state = dehash_edges(i)
			solution = IDAstar(state, heuristic; kwargs...)
			cache.edges[i] = UInt8(length(solution))
			@info "Cached hash $(i)!"
		end
	end

	@warn "REMEMBER TO SAVE CACHE!"

	cache
end