using Dates

function cache_by_depth(state::Cube, depth:: Integer, max_depth::Integer, cache::Vector{UInt8})
	if depth >= max_depth
		return cache
	end

	cache = cache_if_uncached_symmetry(state, cache, depth)

	for connection in Cube3x3.neighbouring_moves
		cache = cache_by_depth(move(state, connection.moves), depth + connection.cost, max_depth, cache)
	end	

	return cache
end

function cache_by_depth(max_depth::Integer, hashset::C) where C <: Cube
	@info "Started at $(now())"

	Set = C == Edges ? HalfEdges : C

	cache = cache_by_depth(Set(), 0, max_depth, getcache(C))

	@info "Finished at $(now())"
	savecache(cache, C)
end

function cache_if_uncached_symmetry(state::Cube, cache::Vector{UInt8}, value::Integer)
	h = hash(state)
	if value >= cache[h] # The state is cached, so we skip it
		return cache
	end
	
	cache[h] = value 
	for m ∈ symmetry_matrices[2:48]
		h = hash(transform(state, m))
		cache[h] = value
	end

	return cache
end

function find_closest_cached_hash(state::Cube, cache::Vector{UInt8}, depth::Integer, max_depth::Integer)
	if depth >= max_depth
		return Nothing
	end

	h = hash(state)
	if cache[h] != 0xff
		return cache[h]
	end

	map(neighbouring_moves) do c
		moves, cost = c.moves, c.cost

		result = find_closest_cached_hash(move(state, moves), cache, depth + cost, max_depth)

		if result == Nothing
			Inf
		else
			result + cost
		end
	end |> minimum
end

function cache_by_hash(range::AbstractRange, Set::Union{Type{Corners}, Type{Edges}}, cache=getcache(Set))
	try
		Threads.@threads for h in range
			if cache[h] == 0xff
				# I'm not sure this is even better
				# Iterative deepening
				i = 1
				result = Inf
				while result == Inf
					result = find_closest_cached_hash(dehash(h, Set), cache, 0, i)
					i += 1
				end
				
				cache[h] = result
			end
		end
	catch e
		savecache(cache, Set)
	finally
		savecache(cache, Set)
		return cache
	end 
end


function generate_cache(Set::Union{Type{Corners}, Type{Edges}})
	cache = getcache(Set)
	cached_states = filter(x -> x != 0xff, cache) |> length

	# Cache up to depth 5 (~34s)
	if (Set == Corners && cached_states < 212817) || ((Set == HalfEdges) || (Set == Edges) && cached_states < 994633)
		@info "Initializing cache by caching up to depth 5"
		cache_by_depth(5, Set())
	end

	@info "Starting to cache hashes in order"

	chunk_size = 10_000

	i = first_uncached(getcache(Set))

	while i < permutations(Set)
		@info "Caching from $i to $(i + chunk_size)"

		# This saves the cache
		cache_by_hash(i:(i + chunk_size), Set, getcache(Set))
		i += chunk_size
	end
end

