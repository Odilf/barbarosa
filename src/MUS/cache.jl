# MUS stands for "Moves until solved"

const extension = ".barbarosa"
const base_path = joinpath(@__DIR__, "cache")
const corner_path = joinpath(base_path, "corners" * extension)
const edge_path = joinpath(base_path, "edges" * extension)

mutable struct Cache
	corners::Vector{UInt8}
	edges::Vector{UInt8}
end

Cache() = Cache(fill(0xff, corner_permutations), fill(0xff, edge_permutations))

function getcache()
	caches = map(zip([corner_path, edge_path], [corner_permutations, edge_permutations])) do (path, perms)
		if !isfile(path)
			mkpath(base_path)
			write(path, fill(0xff, perms))
		end

		read(path)
	end

	Cache(caches...)
end

function cacheprogress(cache::Vector{UInt8})
	total = length(cache)
	cached = count(n -> n != 0xff, cache)
	percentage = cached/total
	(cached, total, percentage)
end

function cacheprogress(cache::Cache)
	(cacheprogress(cache.corners), cacheprogress(cache.edges))
end

function Base.show(io::IO, cache::Cache)
	((cc, ct, cp), (ec, et, ep)) = cacheprogress(cache)
	print(io, "Cache \n Corners: $cc/$ct ($(cp * 100)%) \n Edges: $ec/$et ($(ep * 100)%)")
end

function savecache(cache::Cache, corner_path::AbstractString, edge_path::AbstractString)
	write(corner_path, cache.corners)
	write(edge_path, cache.edges)

	@info "Saved cache $cache"

	return cache
end

savecache(cache::Cache) = savecache(cache, corner_path, edge_path )

resetcache(paths...) = savecache(Cache(), paths...)

# function cache(cache::Vector{UInt8}, index::Integer, value::Integer)::Vector{UInt8}
# 	if cache[index] != 0
# 		cache[index] = value
# 		@info "Cached hash $index"
# 	end

# 	cache
# end