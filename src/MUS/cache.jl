# MUS stands for "Moves until solved"

const extension = ".barbarosa"
const base_path = joinpath(@__DIR__, "cache")
const corner_path = joinpath(base_path, "corners" * extension)
const edge_path = joinpath(base_path, "edges" * extension)

mutable struct Cache
	corners::Vector{UInt8}
	edges::Vector{UInt8}
end

const edge_permutations = factorial(12) ÷ factorial(6) * 2^6
const corner_permutations = factorial(8) * 3^7

Cube3x3.permutations(::Type{Corners}) = corner_permutations
Cube3x3.permutations(::Type{Edges}) = edge_permutations

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
getcache(::Union{Type{Edges}, Type{HalfEdges}}) = getcache().edges

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

function savecache(cache::Vector{UInt8}, path::AbstractString)
	write(path, cache)

	@info "Saved cache, now it is $(getcache())"
end

function savecache(cache::Cache, corner_path::AbstractString, edge_path::AbstractString)
	savecache(cache.corners, corner_path)
	savecache(cache.edges, edge_path)
end

savecache(cache::Vector{UInt8}, ::Type{Corners}) = savecache(cache, corner_path)
savecache(cache::Vector{UInt8}, ::Union{Type{Edges}, Type{HalfEdges}}) = savecache(cache, edge_path)
savecache(cache::Cache) = savecache(cache, corner_path, edge_path)

resetcache(paths...) = savecache(Cache(), paths...)

Base.max(cache::Vector{UInt8}, check_range=1:30) = max(check_range[[i ∈ cache for i ∈ check_range]]...)
Base.max(cache::Cache, check_range=1:30) = (corners=max(cache.corners, check_range), edges=max(cache.edges, check_range))

first_uncached(cache::Vector{UInt8}) = findfirst(v -> v == 0xff, cache)
first_uncached(cache::Cache) = max(first_uncached(cache.corners), first_uncached(cache.edges))

function cache_heuristic(cache::Cache=getcache(); fallback=manhattan)
	base = first_uncached(cache)
	function h(cube)
		corner_cache = cache.corners[Corners(cube) |> hash]
		edge_cache_1, edge_cache_2 = let 
			h1, h2 = Edges(cube) |> hash
			cache.edges[h1], cache.edges[h2]
		end

		r = max(corner_cache, edge_cache_1, edge_cache_2)
		if r == 0xff
			base + fallback(cube)
		else
			r
		end
	end

	return h
end

let
	h = cache_heuristic()
	h(move(Cube(), "R2 D2 F B"))
end