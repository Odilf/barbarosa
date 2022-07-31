include("../src/main.jl")

module Tests

using Test

@testset "Global" begin
	@testset "3x3" begin
		using ..Cube3x3
		using ..Cube3x3: v, isinrange, makecorners, makeedges, parsemove, movedata, X, Y, Z, R, U, L, D, F, B, rotate, twist, isoriented

		include("3x3.test.jl")
	end

	@testset "Search" begin
		using ..Cube3x3
		using ..Search

		include("search.test.jl")
	end

	@testset "MUS" begin
		using StaticArrays
		using ..Cube3x3
		using ..Cube3x3: permutations
		using ..MUS
		using ..MUS: hash_permutations, dehash_permutations

		include("MUS.test.jl")
	end

	@testset "Barbarosa" begin
		using ..Cube3x3

		include("barbarosa.test.jl")
	end
end

end