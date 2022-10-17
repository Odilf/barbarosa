function cache_by_depth(state::Cube, depth:: Integer, max_depth::Integer, cache::Vector{UInt8})
	if depth >= max_depth
		return cache
	end

	cache = cache_if_uncached_symmetry(state, cache, depth)

	for connection in Cube3x3.neighbouring_moves
		try
			cache_by_depth(move(state, connection.moves), depth + connection.cost, max_depth, cache)
		catch
			return cache
		end
	end

	return cache
end

using Dates

function cache_by_depth(max_depth::Integer, hashset::C) where C <: Cube
	@info "Started at $(now())"
	cache = cache_by_depth(hashset, 0, max_depth, getcache(C))
	@info "Finished at $(now())"
	savecache(cache, C)
end

function cache_if_uncached_symmetry(state::Cube, cache::Vector{UInt8}, value::Integer)
	hashes::Vector{Int} = Vector{Int}(undef, 48)
	for (i, m) in enumerate(symmetry_matrices)
		h = hash(transform(state, m))
		if cache[h] <= value
			# @info "State $h already hashed. Skipping rest of hash sets"
			return cache
		end
		hashes[i] = h
	end
	
	# @info "Caching sets $hashes"
	for h in hashes
		cache[h] = value
	end

	return cache
end

function symmetry_cache(depth::Integer, max_depth::Integer, state::Cube, cache::Vector{UInt8})
	if depth >= max_depth
		return cache
	end

	h = hash(state)
	if depth < cache[h]
		cache[h] = depth
	end

	for cube ∈ symmetries(state)
		cache = symmetry_cache(depth + 1, max_depth, move(cube, "R"), cache)
	end

	return cache
end