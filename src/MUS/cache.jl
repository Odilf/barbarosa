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

getcache(::Type{Corners}) = getcache().corners
getcache(::Type{Edges}) = getcache().edges

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

savecache(cache::Vector{UInt8}, path::AbstractString) = write(path, cache)

function savecache(cache::Cache, corner_path::AbstractString, edge_path::AbstractString)
	savecache(cache.corners, corner_path)
	savecache(cache.edges, edge_path)

	@info "Saved cache $cache"

	return cache
end

savecache(cache::Vector{UInt8}, ::Type{Corners}) = savecache(cache, corner_path)
savecache(cache::Vector{UInt8}, ::Type{Edges}) = savecache(cache, edge_path)
savecache(cache::Cache) = savecache(cache, corner_path, edge_path)

resetcache(paths...) = savecache(Cache(), paths...)

Base.max(cache::Vector{UInt8}, check_range=1:30) = max(check_range[[i ∈ cache for i ∈ check_range]]...)
Base.max(cache::Cache, check_range=1:30) = (corners=max(cache.corners, check_range), edges=max(cache.edges, check_range))