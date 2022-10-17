struct StopException{T}
	S::T
	cache::Vector{UInt8}
end

function Base.showerror(io::IO, ex::StopException, bt; backtrace=true)
    Base.with_output_color(get(io, :color, false) ? :green : :nothing, io) do io
        showerror(io, ex.S)
    end
end

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