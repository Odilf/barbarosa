using .Threads

function cache!(hash::Integer, value::Integer, cache::Vector{UInt8})
	if value < cache[hash]
		cache[hash] = value
	end
end

"""
	cache_to_depth!(depth::Integer, ::C, cache=getcache(C)) where C <: Cube

Caches all combinations of moves up to a certain depth. 

Modifies `cache` in place.
"""
function cache_to_depth!(depth::Integer, ::C, cache=getcache(C)) where C <: Cube

	function depth_loop(state::Cube, current_depth::Integer)
		if current_depth > depth
			return
		end

		cache!(hash(state), current_depth, cache)

		for connection ∈ neighbouring_moves
			depth_loop(move(state, connection.moves), current_depth + connection.cost)
		end
	end

	depth_loop(C(), 0)
	return cache
end

"""
	cache_neighbours(::C, cache::Vector{UInt8}; target::Integer) where C <: Cube
	cache_neighbours(::C) where C <: Cube

Caches value `target` if a neighbour has cache `target - 1`.

This produces complete results only if cache for `target - 1` is complete. 

Uses multithreading. 
"""
function cache_neighbours!(::C, cache::Vector{UInt8}; target::Integer) where C <: Cube
	visited = 0
	@threads for h ∈ 1:length(cache)
		if cache[h] == target - 1
			state = dehash(h, C)
			for m ∈ Cube3x3.all_possible_moves
				cache!(hash(move(state, m)), target, cache)
			end

			visited += 1
		end
	end

	println("Visited $visited nodes")
end

function cache_neighbours(::C, range::AbstractRange) where C <: Cube
	set = C == Edges ? HalfEdges : Corners
	cache = getcache(set)

	# Start by caching the first state (`hash(C())` should be 1)
	cache[hash(set())] = 0x00

	println("Starting with cache $(getcache()) \n")

	for i ∈ range	
		println("Iteration $i")
		
		@time try
			cache_neighbours!(C(), cache; target=i)
		finally
			savecache(cache, C)
		end

		println()
	end
end