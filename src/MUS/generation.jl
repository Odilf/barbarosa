function cache_by_depth(cache::Vector{UInt8}, cube::Cube{N}, threshold::Integer, moves::Integer = 0)::Vector{UInt8} where N
	if moves > threshold
		return cache
	end

	for connection in connections
		cache = cache_by_depth(cache, move(cube, connection.moves), threshold, moves + connection.cost)
	end

	i = N == 12 ? hash(cube)[1] : hash(cube)

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
	edge_cache = cache_by_depth(cache.edges, HalfEdges(), depth)
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
		if cached != 0
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
		if cache.corners[i] != 0
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

function mus_edge_heuristic(cache::Vector{UInt8}, fallback=manhattan)
	if length(cache) != edge_permutations
		error("Incorrect cache passed to function (it is $(length(cache)) instead of $edge_permutations")
	end

	function heuristic(cube::HalfEdges)
		cached = cache[hash(cube)]
		if cached != 0
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
		if cache.edges[i] != 0
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

# let 
# 	h = mus_corner_heuristic(getcache().corners; fallback=x -> manhattan(x)*10)
# 	c = move(Corners(), "R L D F R L D2 F R D")
# 	steps = IDAstar(c, h)

# 	h.(steps), reconstruct_solution(steps)
# end

# let
# 	cache = cache_by_depth(Cache().corners, Corners(), 2, 0)
# 	i = hash(move(Corners(), "R2 F"))
# 	cache[i]
# end