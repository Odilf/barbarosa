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

using Dates

function cache_by_depth(max_depth::Integer, hashset::C) where C <: Cube
	@info "Started at $(now())"

	cache = cache_by_depth(hashset, 0, max_depth, getcache(C))

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

function cache_by_hash(range::AbstractRange, Set::Union{Type{Corners}, Type{Edges}}, cache=getcache(Set))
	try
		for h in range
			if cache[h] != 0xff
				# @info "Skipping hash $h"
				continue
			end

			@info "Caching hash $h"

			cube = dehash(h, Set)
			solve_length = IDAstar(cube, cache_heuristic(Set); silent=true) |> reconstruct_solution
			cache[h] = UInt8(solve_length |> length)
		end
	catch e
		if typeof(e) != InterruptException
			rethrow(e)
		end
	finally
		savecache(cache, Set)
	end
end

function cache_by_hash(Set::Union{Type{Corners}, Type{Edges}})
	cache_by_hash(first_uncached(getcache(Set)):permutations(Set), Set)
end

cache_by_hash(Edges)