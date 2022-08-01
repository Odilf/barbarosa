function cache_by_depth(cache::Vector{UInt8}, cube::HashSet, threshold::Integer, moves::Integer = 0)::Vector{UInt8}
	if moves > threshold
		return []
	end

	for neighbour in neighbours(cube)
		cache_by_depth(cache, neighbour, threshold, moves + 1)
	end

	i = if cube isa Edges
		hash(cube)[1]
	else
		hash(cube)
	end
	cache[i] = moves

	cache
end

function cache_corners_by_depth(depth::Integer)
	cache = getcache()
	cache.corners = cache_by_depth(cache.corners, cube() |> corners, depth)
	savecache(cache)
end

function cache_edges_by_depth(depth::Integer)
	cache = getcache()
	cache.edges = cache_by_depth(cache.edges, edges(cube()), depth)
	savecache(cache)
end



function mus_corner_heuristic(cache::Vector{UInt8}, fallback=manhattan)
	if length(cache) != corner_permutations
		error("Incorrect cache passed to function (it is $(length(cache)) instead of $corner_permutations")
	end

	function heuristic(cube::Corners)
		v = cache[hash(cube)]
		if v != 0
			return v
		else
			return manhattan(cube)
		end
	end

	return heuristic
end

function cache_corners(cache::Cache, range::AbstractRange; kwargs...)
	heuristic = mus_corner_heuristic(cache.corners)

	for i in range
		if cache.corners[i] != 0
			println("Skipping caching hash $i")
		else
			state = dehash_corners(i)
			solution = IDAstar(state, heuristic; kwargs...)
			cache.corners[i] = length(solution)
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
		v = cache[hash(cube)]
		if v != 0
			return v
		else
			return manhattan(cube)
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
			cache.edges[i] = length(solution)
			@info "Cached hash $(i)!"
		end
	end

	@warn "REMEMBER TO SAVE CACHE!"

	cache
end